use std::error::Error;
use std::io::{Read, Seek};
use jfrs::reader::{event::{Accessor, Event}, JfrReader, value_descriptor::ValueDescriptor};
use log::{debug, warn};

pub const EXEC_SAMPLE: &str = "jdk.ExecutionSample";

pub const NATIVE_EXEC_SAMPLE: &str = "jdk.NativeMethodSample";

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExecutionSample {
    pub start_time: i64,
    pub thread: Thread,
    pub stack_trace: StackTrace,
    pub native: bool,
}

impl ExecutionSample {
    pub fn visit_events<T: Read + Seek>(source: T, mut visitor: impl FnMut(ExecutionSample)) {
        let mut reader = JfrReader::new(source);

        for (mut r, c) in reader.chunks().filter_map(|cr| warn_error("read chunk", cr)) {
            debug!("Parsing chunk with size {}", c.header.chunk_size);
            r.events(&c)
                .filter_map(|er| warn_error("read event", er))
                .filter(|e| e.class.name() == EXEC_SAMPLE || e.class.name() == NATIVE_EXEC_SAMPLE)
                .map(|e| ExecutionSample::from(e))
                .filter(|e| !e.stack_trace.truncated)
                .for_each(|e| visitor(e));
        }
    }
}

fn warn_error<T>(msg: &'static str, result: Result<T, impl Error>) -> Option<T> {
    result.inspect_err(|e| warn!("Unable to {msg}: {e}")).ok()
}

impl From<Event<'_>> for ExecutionSample {
    fn from(value: Event) -> Self {
        let native = value.class.name() == NATIVE_EXEC_SAMPLE;
        let v = value.value();
        let start_time: i64 = extract_primitive(&v, "startTime");
        let thread: Thread = value.value().get_field("sampledThread").unwrap().into();
        let stack_trace: StackTrace = value.value().get_field("stackTrace").unwrap().into();
        return Self {
            start_time,
            thread,
            stack_trace,
            native,
        };
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Thread {
    pub java_name: String,
    pub java_thread_id: i64,
}

impl From<Accessor<'_>> for Thread {
    fn from(value: Accessor<'_>) -> Self {
        let java_name = extract_primitive::<&str>(&value, "javaName").to_string();
        let java_thread_id: i64 = extract_primitive(&value, "javaThreadId");
        return Thread {
            java_name,
            java_thread_id,
        };
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

        return Self { truncated, frames };
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
        return Self {
            method,
            line_number,
            bytecode_index,
        };
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
    name: String,
    class: Class,
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
        let name = extract_symbol(&value, "name");
        return Self { name };
    }
}

fn extract_symbol(value: &Accessor, name: &str) -> String {
    let str: &str = extract_primitive(&value.get_field(name).unwrap(), "string");
    return str.to_string();
}

fn extract_primitive<'a, T>(value: &Accessor<'a>, name: &str) -> T
where
    T: TryFrom<&'a ValueDescriptor>,
{
    return value
        .get_field(name)
        .and_then(|v| <T>::try_from(v.value).ok())
        .unwrap();
}
