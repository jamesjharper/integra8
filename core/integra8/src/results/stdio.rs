use std::any::Any;
use std::str;
use std::str::Utf8Error;

#[derive(Clone, PartialEq, Debug)]
pub struct TestResultStdio {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl TestResultStdio {
    pub fn from_panic(payload: &(dyn Any + Send)) -> Self {

        // Unfortunately, panic unwind only gives the payload portion of 
        // the panic info. Also, if panic formatter is used we don't even
        // get a panic message. This is here to get what little info we have,
        if let Some(s) = payload.downcast_ref::<&str>() {
            TestResultStdio::stderr(s.to_string())
        } else if let Some(s) = payload.downcast_ref::<&String>() {
            TestResultStdio::stderr(*s)
        } else {
            TestResultStdio::no_output()
        }
    }


    pub fn stdout_utf8<'a>(&'a self) -> Option<Result<&'a str, Utf8Error>> {
        match self.stdout.is_empty() {
            true => None,
            false => {
               Some(str::from_utf8(& self.stdout))
            }
        }
    }

    pub fn stderr_utf8<'a>(&'a self) -> Option<Result<&'a str, Utf8Error>> {
        match self.stderr.is_empty() {
            true => None,
            false => {
               Some(str::from_utf8(& self.stderr))
            }
        }
    }

    pub fn stderr(message: impl Into<String>) -> Self {
        Self {
            stdout: message.into().into_bytes(),
            stderr:vec![],
        }
    }


    pub fn no_output() -> Self {
        Self {
            stdout: vec![],
            stderr: vec![]
        }
    }
}

