use std::str::FromStr;
use atty::Stream;

use structopt::StructOpt;
#[derive(StructOpt, Clone, Debug)] // TODO: Remove the need for clone here
pub struct TreeFormatterParameters {
    // No extended parameters
}

#[derive(Clone, Eq, PartialEq)]
pub enum Style {
    Text,
    Symbols
}

impl Style {
    pub fn default_value() -> Self {
        Self::Symbols
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "Text",
            Self::Symbols => "Symbols",
        } 
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Text", "Symbols"]
    }
}

impl FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Text" => Ok(Style::Text),
            "Symbols" => Ok(Style::Symbols),
            _ => Err(format!("{} was not a valid style. Valid values are either \"Text\" or \"Symbols\".", s))
        }
    }
}


#[derive(Clone, Eq, PartialEq)]
pub enum DetailLevel {
    Error,
    Verbose
}

impl DetailLevel {
    pub fn default_value() -> Self {
        Self::Error
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Verbose => "Verbose",
        } 
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Error", "Verbose"]
    }
}

impl FromStr for DetailLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Error" => Ok(DetailLevel::Error),
            "Verbose" => Ok(DetailLevel::Verbose),
            _ => Err(format!("{} was not a valid detail level. Valid values are either \"Error\" or \"Verbose\".", s))
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Encoding {
    Ascii,
    Utf8
}

impl Encoding {
    pub fn default_value() -> Self {
        Self::Utf8
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ascii => "Ascii",
            Self::Utf8 => "Utf8",
        } 
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Ascii", "Utf8"]
    }
}

impl FromStr for Encoding {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ascii" => Ok(Encoding::Ascii),
            "Utf8" => Ok(Encoding::Utf8),
            _ => Err(format!("{} was not a valid encoding type. Valid values are either \"Ascii\" or \"Utf8\".", s))
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum AnsiMode {
    Auto,
    Enabled,
    Disabled,
}

impl AnsiMode {

    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Auto => atty::is(Stream::Stdout),
            Self::Enabled => true,
            Self::Disabled => false,
        } 
    }

    pub fn default_value() -> Self {
        Self::Auto
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        } 
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Auto", "Enabled", "Disabled"]
    }
}


impl FromStr for AnsiMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Auto" => Ok(AnsiMode::Auto),
            "Enabled" => Ok(AnsiMode::Enabled),
            "Disabled" => Ok(AnsiMode::Disabled),
            _ => Err(format!("{} was not a ANSI mode. Valid values are \"Auto\", \"Enabled\" or \"Disabled\".", s))
        }
    }
}

