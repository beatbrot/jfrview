use crate::flame_graph::{FlameGraph, Frame};
use serde::Serialize;

#[derive(Serialize)]
pub struct Sample {
    name: String,
    value: usize,
    children: Vec<Sample>,
}

impl Sample {
    pub fn from(fg: FlameGraph, include_native: bool) -> Self {
        fn handle(frame: &Frame, include_native: bool) -> Sample {
            Sample {
                name: format!("{:?}", frame.method),
                value: frame.ticks(include_native),
                children: frame
                    .children
                    .values()
                    .map(|m| handle(m, include_native))
                    .collect(),
            }
        }

        Self {
            name: "Root".to_string(),
            value: fg.ticks(include_native),
            children: fg
                .frames
                .values()
                .map(|m| handle(m, include_native))
                .collect(),
        }
    }
}
