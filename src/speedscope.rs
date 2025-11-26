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

    use crate::speedscope::export;

    #[test]
    fn convert_heavy_rich() -> anyhow::Result<()> {
        let file = File::open("test-data/heavy.jfr")?;
        export(file, false)?;

        Ok(())
    }
}
