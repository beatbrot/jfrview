use anyhow::{Context, anyhow};
use jfrs::reader::value_descriptor::Primitive;
use jfrs::reader::{
    JfrReader,
    event::{Accessor, Event},
    value_descriptor::ValueDescriptor,
};
use std::io::{Read, Seek};

pub const EXEC_SAMPLE: &str = "jdk.ExecutionSample";

pub const NATIVE_EXEC_SAMPLE: &str = "jdk.NativeMethodSample";

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExecutionSample {
    pub start_time: i64,
    pub stack_trace: StackTrace,
    pub native: bool,
}

impl ExecutionSample {
    pub fn visit_events<T: Read + Seek>(
        source: T,
        mut visitor: impl FnMut(ExecutionSample),
    ) -> anyhow::Result<()> {
        let mut reader = JfrReader::new(source);

        for result in reader.chunks() {
            let (mut r, c) = result.with_context(|| "Unable to parse chunk.")?;
            for event_res in r.events(&c) {
                let event = event_res.with_context(|| "Unable to parse event.")?;
                if event.class.name() == EXEC_SAMPLE || event.class.name() == NATIVE_EXEC_SAMPLE {
                    let sample = ExecutionSample::from(event);
                    visitor(sample);
                }
            }
        }

        Ok(())
    }
}

impl From<Event<'_>> for ExecutionSample {
    fn from(value: Event) -> Self {
        let native = value.class.name() == NATIVE_EXEC_SAMPLE;
        let v = value.value();
        let start_time: i64 = extract_primitive(&v, "startTime");
        let stack_trace: StackTrace = value.value().get_field("stackTrace").unwrap().into();
        Self {
            start_time,
            stack_trace,
            native,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Thread {
    pub java_name: Option<String>,
    pub java_thread_id: i64,
}

impl From<Accessor<'_>> for Thread {
    fn from(value: Accessor<'_>) -> Self {
        let java_name = extract_nullable_str(&value, "javaName");
        let java_thread_id: i64 = extract_primitive(&value, "javaThreadId");
        Thread {
            java_name,
            java_thread_id,
        }
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.java_thread_id == other.java_thread_id
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct StackTrace {
    pub truncated: bool,
    pub frames: Vec<StackFrame>,
}

impl From<Accessor<'_>> for StackTrace {
    fn from(value: Accessor<'_>) -> Self {
        let truncated: bool = extract_primitive(&value, "truncated");
        let mut frames: Vec<StackFrame> = value
            .get_field("frames")
            .unwrap()
            .as_iter()
            .unwrap()
            .map(|a| a.into())
            .collect();
        frames.reverse();

        Self { truncated, frames }
    }
}

#[allow(dead_code)]
pub struct StackFrame {
    pub line_number: i32,
    pub bytecode_index: i32,
    pub method: Method,
}

impl From<Accessor<'_>> for StackFrame {
    fn from(value: Accessor<'_>) -> Self {
        let bytecode_index: i32 = extract_primitive(&value, "bytecodeIndex");
        let line_number: i32 = extract_primitive(&value, "lineNumber");
        let method: Method = value.get_field("method").map(|v| v.into()).unwrap();
        Self {
            method,
            line_number,
            bytecode_index,
        }
    }
}

impl std::fmt::Debug for StackFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.method, self.line_number)
    }
}

#[allow(dead_code)]
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Method {
    pub name: String,
    pub class: Class,
}

impl std::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.class.pretty_name(), self.name)
    }
}

impl From<Accessor<'_>> for Method {
    fn from(value: Accessor<'_>) -> Self {
        let name = extract_symbol(&value, "name");
        let class: Class = value.get_field("type").map(|v| v.into()).unwrap();
        return Self { name, class };
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn pretty_name(&self) -> String {
        self.name.replace('/', ".")
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<Accessor<'_>> for Class {
    fn from(value: Accessor<'_>) -> Self {
        Self {
            name: extract_symbol(&value, "name"),
        }
    }
}

fn extract_symbol(value: &Accessor, name: &str) -> String {
    let str: &str = extract_primitive(&value.get_field(name).unwrap(), "string");
    str.to_string()
}

fn extract_nullable_str(value: &Accessor, name: &str) -> Option<String> {
    value
        .get_field(name)
        .ok_or_else(|| anyhow!("Unable to find field {name}"))
        .map(|v| {
            if let ValueDescriptor::Primitive(Primitive::NullString) = v.value {
                None
            } else {
                let result = <&str>::try_from(v.value)
                    .map_err(|_| anyhow!("Unable to convert {name} to string"))
                    .unwrap();
                Some(result.to_string())
            }
        })
        .unwrap()
}

fn extract_primitive<'a, T>(value: &Accessor<'a>, name: &str) -> T
where
    T: TryFrom<&'a ValueDescriptor> + Default,
{
    value
        .get_field(name)
        .and_then(|v| <T>::try_from(v.value).ok())
        .unwrap()
}
