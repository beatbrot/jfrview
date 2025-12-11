use std::io::{Read, Seek};

use anyhow::{Context, Ok, Result};
use jfrs::reader::{JfrReader, event::Accessor, value_descriptor::ValueDescriptor};
use serde::Serialize;
use string_cache::DefaultAtom;

use crate::data::{EXEC_SAMPLE, ExecutionSample, NATIVE_EXEC_SAMPLE, StackFrame};

pub fn export<T>(input: T) -> anyhow::Result<Vec<MethodSample>>
where
    T: Read + Seek,
{
    let mut result = Vec::<MethodSample>::new();
    let mut reader = JfrReader::new(input);

    for chunk in reader.chunks() {
        let (mut r, c) = chunk.with_context(|| "Unable to parse chunk.")?;

        for event_res in r.events(&c) {
            let event = event_res.with_context(|| "Unable to parse event.")?;
            let cls_name = event.class.name();
            if cls_name == EXEC_SAMPLE || cls_name == NATIVE_EXEC_SAMPLE {
                let native = cls_name == NATIVE_EXEC_SAMPLE;
                let st_raw = event.value().get_field("stackTrace").unwrap();
                let frames = read_stacktrace(st_raw)?;
                let sample = MethodSample { frames, native };
                result.push(sample);
            }
        }
    }

    Ok(result)
}

fn read_stacktrace(value: Accessor<'_>) -> Result<Vec<Frame>> {
    let frames_raw = value.get_field("frames").unwrap().as_iter().unwrap();
    let mut frames = Vec::with_capacity(frames_raw.size_hint().0);
    for raw_frame in frames_raw {
        let f = read_frame(raw_frame)?;
        frames.push(f);
    }
    frames.reverse();
    Ok(frames)
}

fn read_frame(value: Accessor<'_>) -> Result<Frame> {
    let raw_method = value.get_field("method").unwrap();
    let method_name = extract_symbol(&raw_method, "name");
    let raw_class = raw_method.get_field("type").unwrap();
    raw_class.get_field("name").unwrap();
    let cls_name = extract_symbol(&raw_class, "name");
    let mut result = String::with_capacity(cls_name.len() + 1 + method_name.len());
    for byte in cls_name.chars() {
        let mut b = byte;
        if byte == '/' {
            b = '.';
        }
        result.push(b);
    }
    result.push(':');
    result.push_str(method_name);

    Ok(Frame {
        name: DefaultAtom::from(result),
    })
}

fn extract_symbol<'a>(value: &'a Accessor, name: &str) -> &'a str {
    let str: &str = extract_primitive(&value.get_field(name).unwrap(), "string");
    str
}

fn extract_primitive<'a, T>(value: &Accessor<'a>, name: &str) -> T
where
    T: TryFrom<&'a ValueDescriptor> + Default,
{
    value
        .get_field(name)
        .and_then(|v| <T>::try_from(v.value).ok())
        .unwrap()
}

pub fn export_old<T>(input: T) -> anyhow::Result<Vec<MethodSample>>
where
    T: Read + Seek,
{
    let mut result: Vec<MethodSample> = Vec::new();
    ExecutionSample::visit_events(input, |s| result.push(MethodSample::from(s)))?;

    Ok(result)
}

/// A sample containing a method call.
///
/// Contains the stacktrace at the point of sampling
#[derive(Serialize)]
pub struct MethodSample {
    pub frames: Vec<Frame>,
    pub native: bool,
}

#[derive(Clone, Serialize)]
pub struct Frame {
    pub name: DefaultAtom,
}

impl From<ExecutionSample> for MethodSample {
    fn from(value: ExecutionSample) -> Self {
        fn to_frame(sf: &StackFrame) -> Frame {
            Frame {
                name: sf.method.to_string(),
            }
        }
        let raw_frames = value.stack_trace.frames;
        let mut frames = Vec::with_capacity(raw_frames.len());
        raw_frames.iter().map(to_frame).for_each(|f| frames.push(f));

        Self {
            frames,
            native: value.native,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use insta::{assert_yaml_snapshot, glob};

    use crate::speedscope::export;

    #[test]
    fn convert_valid_files() -> anyhow::Result<()> {
        glob!("../test-data", "*.jfr", |path| {
            let file = File::open(path).unwrap();
            assert_yaml_snapshot!(export(file).unwrap());
        });
        Ok(())
    }

    #[test]
    fn handle_invalid_files() -> anyhow::Result<()> {
        glob!("../test-data", "*.jfr.fail", |path| {
            let file = File::open(path).unwrap();
            assert!(export(file).is_err());
        });

        Ok(())
    }
}
