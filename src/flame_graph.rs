use indexmap::IndexMap;
use jfrs::reader::JfrReader;

use crate::data::{ExecutionSample, Method, EXEC_SAMPLE, NATIVE_EXEC_SAMPLE};
use std::io::{Read, Seek};

#[derive(Default, Debug)]
pub struct FlameGraph {
    pub frames: IndexMap<Method, Frame>,
}

impl FlameGraph {
    pub fn new<T>(value: T) -> FlameGraph
    where
        T: Read + Seek,
    {
        let mut reader = JfrReader::new(value);
        let mut fg = FlameGraph::default();

        for (mut r, c) in reader.chunks().flatten() {
            r.events(&c)
                .flatten()
                .filter(|e| e.class.name() == EXEC_SAMPLE || e.class.name() == NATIVE_EXEC_SAMPLE)
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
    jvm_ticks: usize,
    native_ticks: usize,
    pub children: IndexMap<Method, Frame>,
}

impl Frame {
    fn new(method: Method) -> Self {
        Self {
            method,
            jvm_ticks: Default::default(),
            native_ticks: Default::default(),
            children: Default::default(),
        }
    }

    pub fn has_no_samples(&self, include_native: bool) -> bool {
        self.ticks(include_native) == 0
    }

    pub fn ticks(&self, include_native: bool) -> usize {
        return if include_native {
            self.jvm_ticks + self.native_ticks
        } else {
            self.jvm_ticks
        };
    }

    fn add_ticks(&mut self, include_native: bool, ticks: usize) {
        if include_native {
            self.native_ticks += ticks;
        } else {
            self.jvm_ticks += ticks;
        }
    }
}

impl FlameGraph {
    pub fn add_sample(&mut self, sample: ExecutionSample) {
        let mut cframe = &mut self.frames;
        for frame in sample.stack_trace.frames {
            let entry = cframe
                .entry(frame.method.clone())
                .or_insert_with(|| Frame::new(frame.method.clone()));
            entry.add_ticks(sample.native, 1);
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
            writeln!(f, "{}{:?}: {}", "| ".repeat(indent), mt, frame.jvm_ticks)?;
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
