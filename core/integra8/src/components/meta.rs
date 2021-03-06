use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::convert::AsRef;
use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentPath(Cow<'static, str>);

impl ComponentPath {
    pub fn from(path: &'static str) -> Self {
        Self(Cow::from(path))
    }

    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.0
    }
}

impl Ord for ComponentPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for ComponentPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(usize);

impl ComponentId {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn from(id: usize) -> Self {
        Self(id)
    }

    pub fn as_unique_number(&self) -> usize {
        self.0
    }
}

impl Ord for ComponentId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for ComponentId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentDescription {
    #[cfg_attr(
        feature = "enable_serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    name: Option<Cow<'static, str>>,

    // Note: this object is cloned often. To insure this remains preformat
    // the implementation should favor using `static or Arc when every possible
    #[cfg_attr(
        feature = "enable_serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    description: Option<Cow<'static, str>>,

    id: ComponentId,

    parent_id: ComponentId,

    component_type: ComponentType,

    location: ComponentLocation,

    parent_location: ComponentLocation,
}

impl ComponentDescription {
    pub fn new(
        name: Option<&'static str>,
        id: ComponentId,
        parent_id: ComponentId,
        location: ComponentLocation,
        parent_location: ComponentLocation,
        description: Option<&'static str>,
        component_type: ComponentType,
    ) -> Self {
        Self {
            id,
            parent_id,
            location,
            parent_location,
            component_type,
            name: name.map(Cow::from),
            description: description.map(Cow::from),
        }
    }

    pub fn is_root(&self) -> bool {
        self.location.path == self.parent_location.path
    }

    pub fn full_name(&self) -> String {
        match &self.name {
            Some(name) => name.to_string(),
            None => self.path().to_string(),
        }
    }

    pub fn friendly_name(&self) -> String {
        match &self.name {
            Some(name) => name.to_string(),
            None => self.relative_path(),
        }
    }

    pub fn path(&self) -> &'_ ComponentPath {
        &self.location.path
    }

    pub fn relative_path(&self) -> String {
        if self.is_root() {
            return self.location.path.to_string();
        }

        self.path()
            .as_str()
            .strip_prefix(self.parent_location.path.as_str())
            .map(|relative| {
                // Remove the :: prefix left over from the path
                relative.trim_start_matches(':').to_string()
            })
            .unwrap_or_else(|| self.path().to_string())
    }

    pub fn id(&self) -> &'_ ComponentId {
        &self.id
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

    pub fn location(&self) -> &'_ ComponentLocation {
        &self.location
    }

    pub fn parent_location(&self) -> &'_ ComponentLocation {
        &self.parent_location
    }

    pub fn reassign_ids(&mut self, id: ComponentId, parent_id: ComponentId) {
        self.id = id;
        self.parent_id = parent_id;
    }
}

impl Ord for ComponentLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        // todo add tests for this

        // within files, order by line number
        if self.file_name == other.file_name {
            return self.line.cmp(&other.line);
        }

        // order lexicographically across files
        match self.parent_path().cmp(other.parent_path()) {
            Ordering::Equal => self.line.cmp(&other.line),
            other => other,
        }
    }
}

impl PartialOrd for ComponentLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentLocation {
    pub file_name: Cow<'static, str>,
    pub column: u32,
    pub line: u32,
    pub path: ComponentPath,
}

impl ComponentLocation {
    pub fn hotlink_text(&self) -> String {
        format!("{}:{}:{}", self.file_name, self.line, self.column)
    }

    pub fn parent_path(&self) -> &'_ str {
        self.path
            .as_str()
            .rfind(':')
            .map(|i| &self.path.as_str()[..i.saturating_sub(1)])
            .unwrap_or_else(|| "")
    }
}
