use std::convert::AsRef;
use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentPath(&'static str);

impl ComponentPath {
    pub fn from(path: &'static str) -> Self {
        Self(path)
    }

    pub fn as_str(&self) -> &'_ str {
        self.0
    }
}

impl Display for ComponentPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<OsStr> for ComponentPath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl AsRef<str> for ComponentPath {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentId(usize);

impl ComponentId {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn as_unique_number(&self) -> usize {
        self.0
    }
}

#[derive(Clone)]
pub struct ComponentGeneratorId(usize);

impl ComponentGeneratorId {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) -> ComponentId {
        let next = ComponentId(self.0);
        self.0 += 1;
        next
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

impl std::str::FromStr for ConcurrencyMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Parallel" => Ok(ConcurrencyMode::Parallel),
            "Serial" => Ok(ConcurrencyMode::Serial),
            _ => Err(format!("{} was not a valid concurrency mode. Valid values are either \"Parallel\" or \"Serial\".", s))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentDescription {
    /// The identity of the bookend. Used for uniquely identify the bookend and displaying the test name to the end user.
    path: ComponentPath,

    id: ComponentId,

    parent_path: ComponentPath,

    parent_id: ComponentId,

    description: Option<&'static str>,

    component_type: ComponentType,

    location: Option<ComponentLocation>,

    name: Option<&'static str>,
}

impl ComponentDescription {
    pub fn new(
        path: ComponentPath,
        name: Option<&'static str>,
        id: ComponentId,
        parent_path: ComponentPath,
        parent_id: ComponentId,
        description: Option<&'static str>,
        component_type: ComponentType,
        location: Option<ComponentLocation>,
    ) -> Self {
        Self {
            path,
            id,
            parent_path,
            parent_id,
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

    pub fn path(&self) -> &'_ ComponentPath {
        &self.path
    }

    pub fn relative_path(&self) -> String {
        if self.is_root() {
            return self.path.to_string();
        }

        self.path
            .as_str()
            .strip_prefix(self.parent_path.0)
            .map(|relative| {
                // Remove the :: prefix left over from the path
                relative.trim_start_matches(':').to_string()
            })
            .unwrap_or_else(|| self.path.to_string())
    }

    pub fn id(&self) -> &'_ ComponentId {
        &self.id
    }

    pub fn parent_path(&self) -> &'_ ComponentPath {
        &self.parent_path
    }

    pub fn parent_id(&self) -> &'_ ComponentId {
        &self.parent_id
    }

    pub fn description(&self) -> Option<&'static str> {
        self.description.clone()
    }

    pub fn component_type(&self) -> &'_ ComponentType {
        &self.component_type
    }

    pub fn location(&self) -> Option<&'_ ComponentLocation> {
        self.location.as_ref()
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
