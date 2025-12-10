use std::io::{Read, Seek};

use serde::Serialize;
use string_cache::DefaultAtom;

use crate::data::{ExecutionSample, StackFrame};

pub fn export<T>(input: T) -> anyhow::Result<Vec<MethodSample>>
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
