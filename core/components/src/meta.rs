#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentIdentity {
    // The friendly name of the component (Default: the namespace + ident)
    pub name: Option<&'static str>,

    /// The namespace + ident of the component
    pub path: &'static str,
}

impl ComponentIdentity {
    pub fn new(name: Option<&'static str>, path: &'static str) -> Self {
        Self { name, path }
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
    pub identity: ComponentIdentity,

    pub component_type: ComponentType,

    pub parent_identity: ComponentIdentity,

    pub location: ComponentLocation,
}

impl ComponentDescription {
    pub fn is_root(&self) -> bool {
        self.identity == self.parent_identity
    }

    pub fn full_name(&self) -> String {
        match self.identity.name {
            Some(name) => name.to_string(),
            None => self.identity.path.to_string(),
        }
    }

    pub fn friendly_name(&self) -> String {
        match self.identity.name {
            Some(name) => name.to_string(),
            None => self.relative_path(),
        }
    }

    pub fn relative_path(&self) -> String {
        if self.is_root() {
            return self.identity.path.to_string();
        }

        self.identity
            .path
            .strip_prefix(self.parent_identity.path)
            .map(|relative| {
                // Remove the :: prefix left over from the path
                relative.trim_start_matches(':').to_string()
            })
            .unwrap_or_else(|| self.identity.path.to_string())
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
