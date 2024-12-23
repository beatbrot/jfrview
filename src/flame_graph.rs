use std::cmp::max;
use std::io::{Read, Seek};

use crate::data::{ExecutionSample, Method};
use indexmap::IndexMap;

#[derive(Default, Debug)]
pub struct FlameGraph {
    pub depth: usize,
    pub frames: IndexMap<Method, Frame>,
}

impl FlameGraph {
    pub fn try_new<T>(value: T) -> anyhow::Result<FlameGraph>
    where
        T: Read + Seek,
    {
        let mut fg = FlameGraph::default();
        ExecutionSample::visit_events(value, |e| fg.add_sample(e))?;
        Ok(fg)
    }

    pub fn ticks(&self, include_native: bool) -> usize {
        self.frames.values().map(|v| v.ticks(include_native)).sum()
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub method: Method,
    pub jvm_ticks: usize,
    pub native_ticks: usize,
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
        if include_native {
            self.jvm_ticks + self.native_ticks
        } else {
            self.jvm_ticks
        }
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
        let num_frames = sample.stack_trace.frames.len();
        self.depth = max(self.depth, num_frames);
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
            writeln!(
                f,
                "{}{:?}: ({},{})",
                "| ".repeat(indent),
                mt,
                frame.jvm_ticks,
                frame.native_ticks
            )?;
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

#[cfg(test)]
mod tests {
    use crate::flame_graph::FlameGraph;
    use insta::{assert_snapshot, glob};
    use std::fs::File;

    #[test]
    fn parse_without_panic() {
        glob!("../test-data", "*.jfr", |path| {
            let f = File::open(path).unwrap();
            let flame_graph = FlameGraph::try_new(f);
            insta::with_settings!({
                omit_expression => true,
                description => path.file_name().unwrap().to_string_lossy()
            }, {
                assert_snapshot!(flame_graph.unwrap());
            });
        });
    }

    #[test]
    fn test_invalid() -> anyhow::Result<()> {
        let file = File::open("test-data/invalid.jfr.fail")?;
        assert!(FlameGraph::try_new(file).is_err());
        Ok(())
    }
}
