use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::error::Error;
use std::io::{BufRead, Cursor, Read, Seek, SeekFrom, Write};
use std::mem;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use crate::components::{
    BookEnd, BookEndAttributes, Component, ComponentDescription, ComponentId, ComponentLocation,
    ComponentPath, ComponentType, ConcurrencyMode, Delegate, Test, TestAttributes,
};

pub struct ExecutionContext<TParameters> {
    pub parameters: Arc<TParameters>,
    pub description: ComponentDescription,
    pub artifacts: Arc<ExecutionArtifacts>,
}

pub enum ExecutionArtifact {
    Text(String),
    Value(String, String),
    TextFile(PathBuf),
    TextBuffer(Vec<u8>),
    TextStream(Box<dyn BufferSource + Send + Sync>),
}

pub struct ExecutionArtifacts {
    // Use index map to ensure items alway print in the same order
    // when outputting results
    map: RwLock<IndexMap<String, ExecutionArtifact>>,
}

impl ExecutionArtifacts {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(IndexMap::new()),
        }
    }

    pub fn writer<'a>(&'a self, name: impl Into<String>) -> ExecutionArtifactCursor<'a> {
        ExecutionArtifactCursor::new(self, name.into())
    }

    pub fn include_panic(&self, name: impl Into<String>, payload: &(dyn Any + Send)) -> &Self {
        // Unfortunately, panic unwind only gives the payload portion of
        // the panic info. Also, if panic formatter is used we don't even
        // get a panic message. This is here to get what little info we have,
        if let Some(s) = payload.downcast_ref::<&str>() {
            self.include_text(name, *s)
        } else if let Some(s) = payload.downcast_ref::<&String>() {
            self.include_text(name, *s)
        } else {
            // Can not determine type, so we cant extract anything from this
            self
        }
    }

    pub fn include_text(&self, name: impl Into<String>, string: impl Into<String>) -> &Self {
        self.include(name, ExecutionArtifact::Text(string.into()));
        self
    }

    pub fn include_value<T: std::fmt::Display>(&self, name: impl Into<String>, value: T) -> &Self {
        self.include(name, ExecutionArtifact::Value(format!("{}", value), std::any::type_name::<T>().to_string()));
        self
    }

    pub fn include_text_file(
        &self,
        name: impl Into<String>,
        filename: impl Into<PathBuf>,
    ) -> &Self {
        self.include(name, ExecutionArtifact::TextFile(filename.into()));
        self
    }

    pub fn include_utf8_text_buffer(
        &self,
        name: impl Into<String>,
        buff: impl Into<Vec<u8>>,
    ) -> &Self {
        self.include(name, ExecutionArtifact::TextBuffer(buff.into()));
        self
    }

    pub fn include_utf8_text_stream<R: Read + Seek + Send + Sync + 'static>(
        &self,
        name: impl Into<String>,
        reader: R,
    ) -> &Self {
        self.include(
            name,
            ExecutionArtifact::TextStream(Box::new(SeekAndReadBufferSource { reader })),
        );
        self
    }

    pub fn include(&self, name: impl Into<String>, artifact: ExecutionArtifact) {
        self.map.write().unwrap().insert(name.into(), artifact);
    }

    pub fn drain(&self) -> IndexMap<String, ExecutionArtifact> {
        let mut drain_map = IndexMap::new();
        mem::swap(&mut *self.map.write().unwrap(), &mut drain_map);
        drain_map
    }
}

pub trait BufferSource {
    fn read_all(&mut self) -> std::io::Result<Vec<u8>>;
}

struct SeekAndReadBufferSource<R> {
    reader: R,
}

impl<R: Read + Seek> BufferSource for SeekAndReadBufferSource<R> {
    fn read_all(&mut self) -> std::io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.reader.seek(SeekFrom::Start(0))?;
        self.reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

pub struct ExecutionArtifactCursor<'a> {
    inner: Option<Cursor<Vec<u8>>>,
    execution_artifacts: &'a ExecutionArtifacts,
    key: String,
}

impl<'a> ExecutionArtifactCursor<'a> {
    pub fn new(execution_artifacts: &'a ExecutionArtifacts, key: String) -> Self {
        Self {
            inner: Some(Cursor::new(Vec::new())),
            execution_artifacts: execution_artifacts,
            key: key,
        }
    }
}

impl<'a> Drop for ExecutionArtifactCursor<'a> {
    fn drop(&mut self) {
        // Add self to artifacts automatically once dropped
        if let Some(reader) = mem::take(&mut self.inner) {
            self.execution_artifacts
                .include_utf8_text_buffer(&self.key, reader.into_inner());
        }
    }
}

impl<'a> Read for ExecutionArtifactCursor<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.as_mut().map(|r| r.read(buf)).unwrap()
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.inner.as_mut().map(|r| r.read_exact(buf)).unwrap()
    }
}

impl<'a> BufRead for ExecutionArtifactCursor<'a> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.as_mut().map(|r| r.fill_buf()).unwrap()
    }
    fn consume(&mut self, amt: usize) {
        self.inner.as_mut().map(|r| r.consume(amt)).unwrap()
    }
}

impl<'a> Write for ExecutionArtifactCursor<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.as_mut().map(|r| r.write(buf)).unwrap()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.as_mut().map(|r| r.flush()).unwrap()
    }
}

impl<'a> Seek for ExecutionArtifactCursor<'a> {
    fn seek(&mut self, style: SeekFrom) -> std::io::Result<u64> {
        self.inner.as_mut().map(|r| r.seek(style)).unwrap()
    }

    fn stream_position(&mut self) -> std::io::Result<u64> {
        self.inner.as_mut().map(|r| r.stream_position()).unwrap()
    }
}

#[derive(Clone)]
pub enum ExecutionStrategy {
    GreenThread,
    ChildProcess,
    CurrentThread,
}

pub trait TestParameters {
    // Parameter Projections

    fn is_child_process(&self) -> bool {
        self.child_process_target().is_some()
    }

    fn execution_strategy(&self) -> ExecutionStrategy {
        if self.is_child_process() {
            return ExecutionStrategy::CurrentThread;
        }

        if self.use_child_processes() {
            return ExecutionStrategy::ChildProcess;
        }
        ExecutionStrategy::GreenThread
    }

    // User defined

    fn test_concurrency(&self) -> ConcurrencyMode;
    fn suite_concurrency(&self) -> ConcurrencyMode;
    fn child_process_target(&self) -> Option<&'_ ChildProcessComponentArgs>;

    fn setup_time_limit_duration(&self) -> Duration;
    fn tear_down_time_limit_duration(&self) -> Duration;
    fn test_time_limit_duration(&self) -> Duration;
    fn test_warning_time_limit_duration(&self) -> Duration;

    fn max_concurrency(&self) -> usize;
    fn root_namespace(&self) -> &'static str;
    fn use_child_processes(&self) -> bool;

    fn console_output_style(&self) -> &'_ str;
    fn console_output_detail_level(&self) -> &'_ str;
    fn console_output_encoding(&self) -> &'_ str;
    fn console_output_ansi_mode(&self) -> &'_ str;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChildProcessComponentMetaArgs {
    pub path: ComponentPath,
    pub parent_location: ComponentLocation,
    pub id: ComponentId,
    pub parent_id: ComponentId,
}

impl ChildProcessComponentMetaArgs {
    pub fn from_description(description: ComponentDescription) -> Self {
        Self {
            path: description.location().path.clone(),
            parent_location: description.parent_location().clone(),
            id: description.id().clone(),
            parent_id: description.parent_id().clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChildProcessComponentArgs {
    Test {
        meta: ChildProcessComponentMetaArgs,
        attributes: TestAttributes,
    },
    Setup {
        meta: ChildProcessComponentMetaArgs,
        attributes: BookEndAttributes,
    },
    TearDown {
        meta: ChildProcessComponentMetaArgs,
        attributes: BookEndAttributes,
    },
}

impl ChildProcessComponentArgs {
    pub fn from_str(str_value: &str) -> Result<Self, Box<dyn Error>> {
        let val = serde_json::from_str(str_value)?;
        Ok(val)
    }

    pub fn meta<'a>(&'a self) -> &'a ChildProcessComponentMetaArgs {
        match self {
            Self::Test { meta, .. } => meta,
            Self::Setup { meta, .. } => meta,
            Self::TearDown { meta, .. } => meta,
        }
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        let as_string = serde_json::to_string(&self)?;
        Ok(as_string)
    }

    pub fn into_component<TParameters>(
        self,
        name: Option<&'static str>,
        description: Option<&'static str>,
        location: ComponentLocation,
        component_fn: Delegate<TParameters>,
    ) -> Component<TParameters> {
        match self {
            Self::Test { meta, attributes } => Component::Test(Test {
                description: ComponentDescription::new(
                    name,
                    meta.id,
                    meta.parent_id,
                    location,
                    meta.parent_location,
                    description,
                    ComponentType::Test,
                ),
                attributes: attributes,
                test_fn: component_fn,
            }),
            Self::Setup { meta, attributes } => Component::Setup(BookEnd {
                description: ComponentDescription::new(
                    name,
                    meta.id,
                    meta.parent_id,
                    location,
                    meta.parent_location,
                    description,
                    ComponentType::Setup,
                ),
                attributes: attributes,
                bookend_fn: component_fn,
            }),
            Self::TearDown { meta, attributes } => Component::TearDown(BookEnd {
                description: ComponentDescription::new(
                    name,
                    meta.id,
                    meta.parent_id,
                    location,
                    meta.parent_location,
                    description,
                    ComponentType::TearDown,
                ),
                attributes: attributes,
                bookend_fn: component_fn,
            }),
        }
    }
}
