use std::io::{Read, Seek};

use anyhow::{Context, Ok, Result};
use hashbrown::HashMap;
use jfrs::reader::{JfrReader, event::Accessor, value_descriptor::ValueDescriptor};
#[cfg(test)]
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

pub const EXEC_SAMPLE: &str = "jdk.ExecutionSample";

pub const NATIVE_EXEC_SAMPLE: &str = "jdk.NativeMethodSample";

type NameCache = HashMap<String, String>;

pub fn export<T>(input: T) -> anyhow::Result<Vec<MethodSample>>
where
    T: Read + Seek,
{
    let mut cache: NameCache = HashMap::with_capacity(32);
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
                let frames = read_stacktrace(&mut cache, st_raw)?;
                let sample = MethodSample { frames, native };
                result.push(sample);
            }
        }
    }

    Ok(result)
}

fn read_stacktrace(cache: &mut NameCache, value: Accessor<'_>) -> Result<Vec<Frame>> {
    let frames_raw = value.get_field("frames").unwrap().as_iter().unwrap();
    let mut frames = Vec::with_capacity(frames_raw.size_hint().0);
    for raw_frame in frames_raw {
        let f = read_frame(cache, raw_frame)?;
        frames.push(f);
    }
    frames.reverse();
    Ok(frames)
}

fn read_frame(cache: &mut NameCache, value: Accessor<'_>) -> Result<Frame> {
    let raw_method = value.get_field("method").unwrap();
    let method_name = extract_symbol(&raw_method, "name");
    let raw_class = raw_method.get_field("type").unwrap();
    raw_class.get_field("name").unwrap();
    let cls_name = extract_symbol(&raw_class, "name");
    let mut result = String::with_capacity(cls_name.len() + 1 + method_name.len());

    // Replacing slashes with dot in the class name is surprisingly expensive
    // Since we expect to encounter the same class rather often, we cache this operation

    let normalized: &str = cache.entry_ref(cls_name).or_insert_with_key(|c| {
        let mut cached_cls_name = String::with_capacity(c.len());
        for byte in c.chars() {
            let mut b = byte;
            if byte == '/' {
                b = '.';
            }
            cached_cls_name.push(b);
        }
        cached_cls_name
    });

    result.push_str(normalized);
    result.push(':');
    result.push_str(method_name);

    Ok(Frame { name: result })
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

/// A sample containing a method call.
///
/// Contains the stacktrace at the point of sampling
#[cfg_attr(test, derive(Serialize))]
#[wasm_bindgen(getter_with_clone)]
pub struct MethodSample {
    pub frames: Vec<Frame>,
    pub native: bool,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Serialize))]
#[wasm_bindgen(getter_with_clone)]
pub struct Frame {
    pub name: String,
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
