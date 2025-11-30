use std::io::{Read, Seek};

#[cfg(test)]
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::data::{ExecutionSample, StackFrame};

pub fn export<T>(input: T, include_native: bool) -> anyhow::Result<Vec<MethodSample>>
where
    T: Read + Seek,
{
    let mut result: Vec<MethodSample> = Vec::new();
    ExecutionSample::visit_events(input, |s| {
        if s.native && !include_native {
            // NoOp
        } else {
            result.push(MethodSample::from(s))
        }
    })?;

    return Ok(result);
}

/// A sample containing a method call.
///
/// Contains the stacktrace at the point of sampling
#[cfg_attr(test, derive(Serialize))]
#[wasm_bindgen(getter_with_clone)]
pub struct MethodSample {
    pub frames: Vec<Frame>,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Serialize))]
#[wasm_bindgen(getter_with_clone)]
pub struct Frame {
    pub name: String,
}

impl From<ExecutionSample> for MethodSample {
    fn from(value: ExecutionSample) -> Self {
        fn to_frame(sf: &StackFrame) -> Frame {
            Frame {
                name: format!("{:?}", sf.method),
            }
        }
        let frames: Vec<_> = value.stack_trace.frames.iter().map(to_frame).collect();
        Self { frames }
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
            assert_yaml_snapshot!("non-native", export(file, false).unwrap());
        });
        Ok(())
    }

    #[test]
    fn handle_invalid_files() -> anyhow::Result<()> {
        glob!("../test-data", "*.jfr.fail", |path| {
            let file = File::open(path).unwrap();
            assert!(export(file, false).is_err());
        });

        Ok(())
    }
}
