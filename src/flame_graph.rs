use indexmap::IndexMap;
use jfrs::reader::JfrReader;

use crate::data::{ExecutionSample, Method};
use std::io::{Read, Seek};

#[derive(Default, Debug)]
pub struct FlameGraph {
    depth: usize,
    pub frames: IndexMap<Method, Frame>,
}

const EXEC_SAMPLE: &str = "jdk.ExecutionSample";
const NATIVE_EXEC_SAMPLE: &str = "jdk.NativeMethodSample";

impl FlameGraph {
    
    pub fn new<T>(value: T, include_native: bool) -> FlameGraph
    where
        T: Read + Seek,
    {
        let mut reader = JfrReader::new(value);
        let mut fg = FlameGraph::default();

        for (mut r, c) in reader.chunks().flatten() {
            r.events(&c)
                .flatten()
                .filter(|e| e.class.name() == EXEC_SAMPLE || (include_native && e.class.name() == NATIVE_EXEC_SAMPLE))
                .map(|e| ExecutionSample::from(e))
                .filter(|e| !e.stack_trace.truncated)
                .for_each(|s| fg.add_sample(s));
        }
        return fg;
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub method: Method,
    pub ticks: usize,
    pub children: IndexMap<Method, Frame>,
}

impl Frame {
    fn new(method: Method) -> Self {
        Self {
            method,
            ticks: Default::default(),
            children: Default::default(),
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
