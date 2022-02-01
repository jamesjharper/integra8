use std::any::Any;
use std::str;
use std::error::Error;
use std::borrow::Cow;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum OutputArtifact {
    String(String),
    File(PathBuf),
    TerminalOutput(Vec<u8>)
}

impl OutputArtifact {

    pub fn string(val : impl Into<String>) -> Self {
        Self::String(val.into())
     }
 
    pub fn terminal_output(val : impl Into<Vec<u8>>) -> Self {
        Self::TerminalOutput(val.into())
    } 

    pub fn as_string<'a>(&'a self) -> Result<Cow<'a, str>, Box<dyn Error>> {

        match &self {
            OutputArtifact::String(ref s) => {
                Ok(Cow::from(s))
            },
            OutputArtifact::File(ref filename) => {
                let mut file = File::open(&filename)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                Ok(Cow::from(contents))
            },
            OutputArtifact::TerminalOutput(ref out) => {
                Ok(Cow::from(str::from_utf8(out)?))
            },
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct ComponentRunArtifacts {
    pub map: HashMap<String, OutputArtifact>
}

impl ComponentRunArtifacts {


    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    } 


    pub fn append(&mut self, name : impl Into<String>, content: OutputArtifact) {
        self.map.insert(
            name.into(),
            content,
        );
    }

    pub fn append_panic(&mut self, payload: &(dyn Any + Send)) {
        // Unfortunately, panic unwind only gives the payload portion of
        // the panic info. Also, if panic formatter is used we don't even
        // get a panic message. This is here to get what little info we have,
        if let Some(s) = payload.downcast_ref::<&str>() {
            self.append("stderr", OutputArtifact::String(s.to_string()))
        } else if let Some(s) = payload.downcast_ref::<&String>() {
            self.append("stderr", OutputArtifact::string(*s))
        } else {
            // Can not determine type, so we cant extract anything from this
        }
    }

    pub fn append_stderr(&mut self, out: Vec<u8>) {
        self.append("stderr", OutputArtifact::terminal_output(out))
    }

    pub fn append_stdout(&mut self, out: Vec<u8>) {
        self.append("stdout", OutputArtifact::terminal_output(out))
    }
}