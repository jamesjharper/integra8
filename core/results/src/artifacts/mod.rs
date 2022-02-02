use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str;
use indexmap::IndexMap;

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
   // Use index map to ensure items alway print in the same order 
    // when outputting results
    pub map: IndexMap<String, OutputArtifact>,
}

impl ComponentRunArtifacts {
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }

    pub fn from_execution_artifacts(artifacts: &ExecutionArtifacts) -> Self {
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
}
