use std::io::{Read, Seek};

use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::data::{ExecutionSample, StackFrame};

pub fn export<T>(input: T, include_native: bool) -> anyhow::Result<Vec<Frames>>
where
    T: Read + Seek,
{
    let mut result: Vec<Frames> = Vec::new();
    ExecutionSample::visit_events(input, |s| {
        if s.native && !include_native {
            // NoOp
        } else {
            result.push(Frames::from(s))
        }
    })?;

    return Ok(result);
}

#[derive(Debug, Serialize)]
pub struct Frames {
    pub frames: Vec<Frame>,
}

#[derive(Debug, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Frame {
    name: String,
}

impl From<ExecutionSample> for Frames {
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
