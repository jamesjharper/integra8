use std::fmt::{self, Display, Formatter};
use std::convert::AsRef;
use std::ffi::OsStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentPath(&'static str);


impl ComponentPath {
    pub fn from(path: &'static str) -> Self {
        Self(path)
    }
}

impl Display for ComponentPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<OsStr> for ComponentPath  {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentType {
    Suite,
    Test,
    Setup,
    TearDown,
}

impl ComponentType {
    pub fn is_tear_down(&self) -> bool {
        match self {
            Self::TearDown => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConcurrencyMode {
    Parallel,
    Serial,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentDescription {
    /// The identity of the bookend. Used for uniquely identify the bookend and displaying the test name to the end user.
    pub path: ComponentPath,

    pub parent_path: ComponentPath,

    pub description: Option<&'static str>,

    pub component_type: ComponentType,

    pub location: ComponentLocation,

    name: Option<&'static str>,
}

impl ComponentDescription {

    pub fn new(
        path: ComponentPath,
        name: Option<&'static str>,    
        parent_path: ComponentPath,    
        description: Option<&'static str>,  
        component_type: ComponentType,
        location: ComponentLocation,
    ) -> Self {
        Self {
            path,
            parent_path,   
            name,
            description,
            component_type,
            location,
        }
    }

    pub fn is_root(&self) -> bool {
        self.path == self.parent_path
    }

    pub fn full_name(&self) -> String {
        match self.name {
            Some(name) => name.to_string(),
            None => self.path.to_string(),
        }
    }

    pub fn friendly_name(&self) -> String {
        match self.name {
            Some(name) => name.to_string(),
            None => self.relative_path(),
        }
    }

    pub fn relative_path(&self) -> String {
        if self.is_root() {
            return self.path.to_string();
        }

        self.path
            .0
            .strip_prefix(self.parent_path.0)
            .map(|relative| {
                // Remove the :: prefix left over from the path
                relative.trim_start_matches(':').to_string()
            })
            .unwrap_or_else(|| self.path.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentLocation {
    pub file_name: &'static str,
    pub column: u32,
    pub line: u32,
}

impl ComponentLocation {
    pub fn hotlink_text(&self) -> String {
        format!("{}:{}:{}", self.file_name, self.line, self.column)
    }
}

#[macro_export]
macro_rules! src_loc {
    () => {
        $crate::ComponentLocation {
            file_name: file!(),
            column: column!(),
            line: line!(),
        }
    };
}
