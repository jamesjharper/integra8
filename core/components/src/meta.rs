use std::borrow::Cow;
use std::convert::AsRef;
use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};

#[cfg(feature = "enable_serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentPath(Cow<'static, str>);

impl ComponentPath {
    pub fn from(path: &'static str) -> Self {
        Self(Cow::from(path))
    }

    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.0
    }
}

impl Display for ComponentPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<OsStr> for ComponentPath {
    fn as_ref<'a>(&'a self) -> &'a OsStr {
        self.0.as_ref().as_ref()
    }
}

impl AsRef<str> for ComponentPath {
    fn as_ref<'a>(&self) -> &str {
        &self.0
    }
}

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentId(usize);

impl ComponentId {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn from(id : usize) -> Self {
        Self(id)
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

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
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
    Sequential,
}

impl std::str::FromStr for ConcurrencyMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Parallel" => Ok(ConcurrencyMode::Parallel),
            "Sequential" => Ok(ConcurrencyMode::Sequential),
            _ => Err(format!("{} was not a valid concurrency mode. Valid values are either \"Parallel\" or \"Sequential\".", s))
        }
    }
}

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentDescription {
   
    #[cfg_attr(feature = "enable_serde", serde(skip_serializing_if = "Option::is_none"))]
    name: Option<Cow<'static, str>>,

    #[cfg_attr(feature = "enable_serde", serde(skip_serializing_if = "Option::is_none"))]
    description: Option<Cow<'static, str>>,
   
    // Note: this object is cloned often. To insure this remains preformat
    // the implementation should favor using `static or Arc when every possible
    /// The identity of the bookend. Used for uniquely identify the bookend and displaying the test name to the end user.
    path: ComponentPath,

    parent_path: ComponentPath,

    id: ComponentId,

    parent_id: ComponentId,

    component_type: ComponentType,

    #[cfg_attr(feature = "enable_serde", serde(skip_serializing_if = "Option::is_none"))]
    location: Option<ComponentLocation>,
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
            component_type,
            location,
            name: name.map(Cow::from),
            description: description.map(Cow::from),
        }
    }

    pub fn is_root(&self) -> bool {
        self.path == self.parent_path
    }

    pub fn full_name(&self) -> String {
        match &self.name {
            Some(name) => name.to_string(),
            None => self.path.to_string(),
        }
    }

    pub fn friendly_name(&self) -> String {
        match &self.name {
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
            .strip_prefix(self.parent_path.as_str())
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

    pub fn description<'a>(&'a self) -> Option<&'a str> {
        self.description.as_ref().map(|x| x.as_ref())
    }

    pub fn component_type(&self) -> &'_ ComponentType {
        &self.component_type
    }

    pub fn location(&self) -> Option<&'_ ComponentLocation> {
        self.location.as_ref()
    }
}

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentLocation {
    pub file_name: Cow<'static, str>,
    pub column: u32,
    pub line: u32,
}

impl ComponentLocation {
    pub fn hotlink_text(&self) -> String {
        format!("{}:{}:{}", self.file_name, self.line, self.column)
    }
}
