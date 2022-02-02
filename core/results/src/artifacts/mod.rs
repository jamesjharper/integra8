use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str;
use std::sync::Arc;

use integra8_components::{ExecutionArtifact, ExecutionArtifacts};

#[derive(Clone, PartialEq, Debug)]
pub enum OutputArtifact {
    Text(String),
    TextFile(PathBuf),
    TextBuffer(Vec<u8>),
}

impl OutputArtifact {
    pub fn text(val: impl Into<String>) -> Self {
        Self::Text(val.into())
    }

    pub fn text_buffer(val: impl Into<Vec<u8>>) -> Self {
        Self::TextBuffer(val.into())
    }

    pub fn as_string<'a>(&'a self) -> Result<Cow<'a, str>, Box<dyn Error>> {
        match &self {
            OutputArtifact::Text(ref s) => Ok(Cow::from(s)),
            OutputArtifact::TextFile(ref filename) => {
                let mut file = File::open(&filename)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                Ok(Cow::from(contents))
            }
            OutputArtifact::TextBuffer(ref out) => Ok(Cow::from(str::from_utf8(out)?)),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentRunArtifacts {
    pub map: HashMap<String, OutputArtifact>,
}

impl ComponentRunArtifacts {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn from_execution_artifacts(artifacts: Arc<ExecutionArtifacts>) -> Self {
        Self {
            map: artifacts
                .drain()
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        match v {
                            ExecutionArtifact::Text(s) => OutputArtifact::Text(s),
                            ExecutionArtifact::TextFile(path) => OutputArtifact::TextFile(path),
                            ExecutionArtifact::TextBuffer(buff) => OutputArtifact::TextBuffer(buff),
                            ExecutionArtifact::TextStream(mut reader) => {
                                OutputArtifact::TextBuffer(reader.read_all().unwrap())
                            }
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn append(&mut self, name: impl Into<String>, content: OutputArtifact) {
        self.map.insert(name.into(), content);
    }

    pub fn append_panic(&mut self, payload: &(dyn Any + Send)) {
        // Unfortunately, panic unwind only gives the payload portion of
        // the panic info. Also, if panic formatter is used we don't even
        // get a panic message. This is here to get what little info we have,
        if let Some(s) = payload.downcast_ref::<&str>() {
            self.append("stderr", OutputArtifact::text(s.to_string()))
        } else if let Some(s) = payload.downcast_ref::<&String>() {
            self.append("stderr", OutputArtifact::text(*s))
        } else {
            // Can not determine type, so we cant extract anything from this
        }
    }

    pub fn append_stderr(&mut self, out: Vec<u8>) {
        self.append("stderr", OutputArtifact::text_buffer(out))
    }

    pub fn append_stdout(&mut self, out: Vec<u8>) {
        self.append("stdout", OutputArtifact::text_buffer(out))
    }
}
