use std::{
    hash::Hash,
    io::{Read, Seek},
};

use crate::data::ExecutionSample;
use indexmap::{set::MutableValues, IndexSet};
use serde::Serialize;

#[derive(Serialize)]
pub struct Sample {
    name: String,
    value: usize,
    children: IndexSet<Sample>,
}

impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        return self.name.eq(&other.name);
    }
}

impl Eq for Sample {}

impl Hash for Sample {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Sample {
    pub fn from_file<T>(input: T, include_native: bool, threads: bool) -> anyhow::Result<Self>
    where
        T: Read + Seek,
    {
        let mut result = Self::new("root".to_string());
        ExecutionSample::visit_events(input, |e| result.add_sample(&e, include_native, threads))?;

        Ok(result)
    }

    fn add_sample(&mut self, sample: &ExecutionSample, include_native: bool, threads: bool) {
        if sample.native && !include_native {
            return;
        }
        let mut curr = self;
        curr.value += 1;
        if threads {
            curr = curr.thread_root(sample);
            curr.value += 1;
        }
        for frame in sample.stack_trace.frames.iter() {
            let name = format!("{:?}", frame.method);
            let (idx, _) = curr.children.insert_full(Self::new(name));
            let sample = curr.children.get_index_mut2(idx).unwrap();
            sample.value += 1;
            curr = sample;
        }
    }

    fn thread_root(&mut self, sample: &ExecutionSample) -> &mut Sample {
        let thread_name = sample
            .thread
            .java_name
            .clone()
            .unwrap_or_else(|| sample.thread.java_thread_id.to_string());
        let (idx, _) = self.children.insert_full(Self::new(thread_name));
        return self.children.get_index_mut2(idx).unwrap();
    }

    fn new(name: String) -> Self {
        Self {
            name,
            value: 0,
            children: Default::default(),
        }
    }
}

#[cfg(test)]
impl core::fmt::Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn printer(
            f: &mut std::fmt::Formatter<'_>,
            indent: usize,
            sample: &Sample,
        ) -> std::fmt::Result {
            writeln!(
                f,
                "{}{}: ({})",
                "| ".repeat(indent),
                sample.name,
                sample.value
            )?;
            for c in &sample.children {
                printer(f, indent + 1, c)?;
            }
            Ok(())
        }
        for c in &self.children {
            printer(f, 0, c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use insta::{assert_snapshot, glob};
    use std::fs::File;

    use crate::export::Sample;

    #[test]
    fn parse_without_panic_nonnative() {
        glob!("../test-data", "*.jfr", |path| {
            let f = File::open(path).unwrap();
            let flame_graph = Sample::from_file(f, false);
            let file_name = path.file_name().unwrap().to_string_lossy();
            insta::with_settings!({
                omit_expression => true,
                description => format!("non-native, {}", file_name)
            }, {
                assert_snapshot!(flame_graph.unwrap());
            });
        });
    }

    #[test]
    fn parse_without_panic_native() {
        glob!("../test-data", "*.jfr", |path| {
            let f = File::open(path).unwrap();
            let flame_graph = Sample::from_file(f, true);
            let file_name = path.file_name().unwrap().to_string_lossy();
            insta::with_settings!({
                omit_expression => true,
                description => format!("native, {}", file_name)
            }, {
                assert_snapshot!(flame_graph.unwrap());
            });
        });
    }

    #[test]
    fn test_invalid() -> anyhow::Result<()> {
        let file = File::open("test-data/invalid.jfr.fail")?;
        assert!(Sample::from_file(file, false).is_err());
        Ok(())
    }
}
