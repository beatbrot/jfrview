use crate::data::{ExecutionSample, Method};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct FlameGraph {
    depth: usize,
    pub frames: HashMap<Method, Frame>,
}

#[derive(Debug)]
pub struct Frame {
    pub method: Method,
    pub ticks: usize,
    pub children: HashMap<Method, Frame>,
}

impl Frame {
    fn new(method: Method) -> Self {
        Self {
            method,
            ticks: 0,
            children: HashMap::default(),
        }
    }
}

impl FlameGraph {
    pub fn add_sample(&mut self, sample: ExecutionSample) {
        let mut cframe = &mut self.frames;
        let mut depth: usize = 0;
        for frame in sample.stack_trace.frames {
            let entry = cframe
                .entry(frame.method.clone())
                .or_insert_with(|| Frame::new(frame.method.clone()));
            depth += 1;
            self.depth = std::cmp::max(self.depth, depth);
            entry.ticks += 1;
            cframe = &mut entry.children;
        }
    }
}

impl std::fmt::Display for FlameGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn printer(
            f: &mut std::fmt::Formatter<'_>,
            indent: usize,
            mt: &Method,
            frame: &Frame,
        ) -> std::fmt::Result {
            writeln!(f, "{}{:?}: {}", "| ".repeat(indent), mt, frame.ticks)?;
            for (k, v) in &frame.children {
                printer(f, indent + 1, k, v)?;
            }
            Ok(())
        }
        for (k, v) in &self.frames {
            printer(f, 0, k, v)?;
        }
        Ok(())
    }
}
