use indexmap::IndexMap;
use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str;

use crate::components::{ExecutionArtifact, ExecutionArtifacts};

#[cfg(feature = "enable_serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub enum OutputArtifact {
    Text(String),
    TextFile(PathBuf),
    #[cfg_attr(feature = "enable_serde", serde(with = "as_utf_string"))]
    TextBuffer(Vec<u8>),
    Value {
        value: String, 
        type_name: String
    },
}

impl OutputArtifact {
    pub fn text(val: impl Into<String>) -> Self {
        Self::Text(val.into())
    }

    pub fn text_file(val: impl Into<PathBuf>) -> Self {
        Self::TextFile(val.into())
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

            OutputArtifact::Value { ref value,  .. } => {
                Ok(Cow::from(value))
            }
        }
    }
}

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub struct ComponentRunArtifacts {
    // Use index map to ensure items alway print in the same order
    // when outputting results
    #[cfg_attr(feature = "enable_serde", serde(with = "indexmap::serde_seq"))]
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
                            ExecutionArtifact::Text(s) => OutputArtifact::text(s),
                            ExecutionArtifact::TextFile(path) => OutputArtifact::text_file(path),
                            ExecutionArtifact::TextBuffer(buff) => {
                                OutputArtifact::text_buffer(buff)
                            }
                            ExecutionArtifact::TextStream(mut reader) => {
                                OutputArtifact::TextBuffer(reader.read_all().unwrap())
                            }
                            ExecutionArtifact::Value(value, type_name) => {
                                OutputArtifact::Value { value,  type_name }
                            }
                        },
                    )
                })
                .collect(),
        }
    }
}

#[cfg(feature = "enable_serde")]
mod as_utf_string {
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};
    use std::str;

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let utf8_str = str::from_utf8(v).map_err(|e| serde::ser::Error::custom(e))?;
        str::serialize(&utf8_str, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        Ok(String::deserialize(d)?.as_bytes().to_vec())
    }
}
