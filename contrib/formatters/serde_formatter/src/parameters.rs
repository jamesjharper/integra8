
use std::str::FromStr;

use structopt::StructOpt;
#[derive(StructOpt, Clone, Debug)]
pub struct SerdeFormatterParameters {
    // No extended parameters
}

#[derive(Clone, Eq, PartialEq)]
pub enum Style {
    Concise,
    Indented,
}

impl Style {
    pub fn default_value() -> Self {
        Self::Indented
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Concise => "Concise",
            Self::Indented => "Indented",
        }
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Indented", "Concise"]
    }
}

impl FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Indented" => Ok(Style::Indented),
            "Concise" => Ok(Style::Concise),
            _ => Err(format!(
                "{} was not a valid style. Valid values are either \"Indented\" or \"Concise\".",
                s
            )),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum DetailLevel {
    ErrorOnly,
    WarningAndErrorOnly,
    All,
}

impl DetailLevel {
    pub fn default_value() -> Self {
        Self::All
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ErrorOnly => "ErrorOnly",
            Self::WarningAndErrorOnly => "WarningAndErrorOnly",
            Self::All => "All",
        }
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["ErrorOnly", "WarningAndErrorOnly", "All"]
    }
}

impl FromStr for DetailLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ErrorOnly" => Ok(DetailLevel::ErrorOnly),
            "WarningAndErrorOnly" => Ok(DetailLevel::WarningAndErrorOnly),
            "All" => Ok(DetailLevel::All),
            _ => Err(format!("{} was not a valid detail level. Valid values are \"ErrorOnly\", \"WarningAndErrorOnly\" or \"All\".", s))
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Encoding {
    Utf8,
}

impl Encoding {
    pub fn default_value() -> Self {
        Self::Utf8
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Utf8 => "Utf8",
        }
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Utf8"]
    }
}

impl FromStr for Encoding {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Utf8" => Ok(Encoding::Utf8),
            _ => Err(format!(
                "{} was not a valid encoding type. Valid value is only \"Utf8\".",
                s
            )),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum AnsiMode {
    Disabled,
}

impl AnsiMode {

    pub fn default_value() -> Self {
        Self::Disabled
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
        }
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Disabled"]
    }
}

impl FromStr for AnsiMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Disabled" => Ok(AnsiMode::Disabled),
            _ => Err(format!(
                "{} was not a ANSI mode. Valid value is only \"Disabled\".",
                s
            )),
        }
    }
}
