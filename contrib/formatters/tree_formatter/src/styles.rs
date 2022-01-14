


use ansi_term::Style;
use ansi_term::Colour::{Black, Red, Green, Yellow, Purple};
use integra8_formatters::models::{ComponentType, ComponentResult, PassReason};
use integra8_formatters::models::report::ComponentRunReport;


pub struct StyleSettings {
    pub formatting: FormattingTheme,
    pub output: OutputTheme ,
    pub characters :CharacterTheme,
    pub level :OutputLevel
 }


#[derive(Clone, Eq, PartialEq)]
pub enum OutputLevel {
    Verbose,
    Info,
    Error,
 }

pub enum CharacterTheme {
    Utf8,
    Ascii,
}

pub enum OutputTheme {
    Symbols,
    Text,
 }

pub enum FormattingTheme {
    NoAnsi,
    Standard,
}

impl FormattingTheme {
    pub fn apply_pass_colour(&self, text: impl Into<String>) -> String {
        match self {
            Self::Standard => Green.paint(text.into()).to_string(),
            Self::NoAnsi => text.into(),
        }
    }

    pub fn apply_fail_colour(&self, text: impl Into<String>) -> String {
        match self {
            Self::Standard => Red.paint(text.into()).to_string(),
            Self::NoAnsi => text.into(),
        }
    }

    pub fn apply_warning_colour(&self, text: impl Into<String>) -> String {
        match self {
            Self::Standard => Yellow.paint(text.into()).to_string(),
            Self::NoAnsi => text.into(),
        }
    }

    pub fn apply_skipped_colour(&self, text: impl Into<String>) -> String {
        match self {
            Self::Standard => Style::default().dimmed().paint(text.into()).to_string(),
            Self::NoAnsi => text.into(),
        }
    }

    pub fn apply_tree_colour(&self, text: impl Into<String>) -> String {
        match self {
            Self::Standard => Style::default().dimmed().paint(text.into()).to_string(),
            Self::NoAnsi => text.into(),
        }
    }
}


pub struct ComponentNodeStyle {
    pub pass: String,
    pub failed: String, 
    pub overtime: String, 
    pub skipped: String, 
    pub warning: String
}

impl ComponentNodeStyle {

    pub fn icon(&self, report: &ComponentRunReport) -> &'_ str {
        match report.result {
            ComponentResult::Pass(PassReason::Accepted) => {
                match report.timing.is_warn() {
                    true => &self.warning,
                    false =>  &self.pass
                }
            },
            ComponentResult::Pass(PassReason::FailureAllowed) => {
                &self.warning
            },
            ComponentResult::Fail(_) => {
                &self.failed  
            },
            ComponentResult::DidNotRun(_) => {
                &self.skipped  
            },
        }
    }
}

pub struct NodeStyle {
    suite: ComponentNodeStyle,
    test: ComponentNodeStyle,
    setup: ComponentNodeStyle,
    tear_down: ComponentNodeStyle
}

impl NodeStyle {
    pub fn new(settings : &StyleSettings) -> Self {
        match settings.output {
            OutputTheme::Text => Self::text(&settings.formatting),
            OutputTheme::Symbols => Self::symbols(&settings.formatting),
        }
    }

    pub fn text(theme : &FormattingTheme) -> Self {

        let pass = theme.apply_pass_colour("[✓]");
        let failed = theme.apply_fail_colour("[x]");
        let overtime = theme.apply_fail_colour("[⏳]");
        let skipped = theme.apply_skipped_colour("[-]");
        let warning = theme.apply_warning_colour("[!]");

        Self {
            suite: ComponentNodeStyle {
                pass: format!("{} Suite", pass),
                failed: format!("{} Suite", failed),
                overtime: format!("{} Suite", overtime),
                skipped: format!("{} Suite", skipped),
                warning: format!("{} Suite", warning),
            },
            test: ComponentNodeStyle {
                pass: format!("{} Test", pass),
                failed: format!("{} Test", failed),
                overtime: format!("{} Test", overtime),
                skipped: format!("{} Test", skipped),
                warning: format!("{} Test", warning),
            },
            setup: ComponentNodeStyle {
                pass: format!("{} Setup", pass),
                failed: format!("{} Setup", failed),
                overtime: format!("{} Setup", overtime),
                skipped: format!("{} Setup", skipped),
                warning: format!("{} Setup", warning),
            },
            tear_down: ComponentNodeStyle {
                pass: format!("{} Tear Down", pass),
                failed: format!("{} Tear Down", failed),
                overtime: format!("{} Tear Down", overtime),
                skipped: format!("{} Tear Down", skipped),
                warning: format!("{} Tear Down", warning),
            },
        }
    }

    pub fn symbols(theme : &FormattingTheme) -> Self {

        Self {
            suite: ComponentNodeStyle {
                pass: theme.apply_pass_colour("○"),
                failed: theme.apply_fail_colour("●"),
                overtime: theme.apply_fail_colour("⊛"),
                skipped: theme.apply_skipped_colour("◌"),
                warning: theme.apply_warning_colour("◑"),
            },
            test: ComponentNodeStyle {
                pass: theme.apply_pass_colour("□"),
                failed: theme.apply_fail_colour("■"),
                overtime: theme.apply_fail_colour("▧"),
                skipped: theme.apply_skipped_colour("⬚"),
                warning: theme.apply_warning_colour("◪"),
            },
            setup: ComponentNodeStyle {
                pass: theme.apply_pass_colour("△"),
                failed: theme.apply_fail_colour("▲"),
                overtime: theme.apply_fail_colour("◭"),
                skipped: theme.apply_skipped_colour("△"),
                warning: theme.apply_warning_colour("◭"),
            },
            tear_down: ComponentNodeStyle {
                pass: theme.apply_pass_colour("▽"),
                failed: theme.apply_fail_colour("▼"),
                overtime: theme.apply_fail_colour("⧨"),
                skipped: theme.apply_skipped_colour("▽"),
                warning: theme.apply_warning_colour("⧨"),
            },
        }
    }


    pub fn icon_style<'a>(&'a self, report: &ComponentRunReport) -> &'a ComponentNodeStyle {
        match report.description.component_type {
            ComponentType::Suite => &self.suite,
            ComponentType::Test => &self.test,
            ComponentType::Setup => &self.setup,
            ComponentType::TearDown => &self.tear_down,
        }
    }

    pub fn component_heading(&self, report: &ComponentRunReport) -> String {
        let icon = self.icon_style(report).icon(report);
        format!("{} - {}", icon, report.description.friendly_name())
    }

    pub fn component_heading_with_remark(&self, report: &ComponentRunReport, remark: &str) -> String {
        let icon = self.icon_style(report).icon(report);
        format!("{} - {} ({})", icon, report.description.friendly_name(), remark)
    }

    pub fn attribute_style(&self, attribute_name: &str) -> String {
        Purple.italic().paint(format!("{}:", attribute_name)).to_string()
    }
}


pub struct TreeBranchStyle {
    pub child: String, 
    pub last_child: String, 
    pub no_child: String, 
    pub no_branch: String, 
    pub attribute_indent: String, 
}

impl TreeBranchStyle {
    pub fn new(settings : &StyleSettings) -> Self {
        match settings.characters {
            CharacterTheme::Utf8 => Self::utf8(&settings.formatting),
            CharacterTheme::Ascii => Self::ascii(&settings.formatting),
        }
    }

    pub fn ascii(theme : &FormattingTheme) -> Self {
        Self {
            child: theme.apply_tree_colour(" ¦-- "), 
            last_child: theme.apply_tree_colour(" '-- "), 
            no_child: theme.apply_tree_colour(" ¦   "), 
            no_branch: "     ".to_string(), 
            attribute_indent: "  ".to_string(), 
        }
    }

    pub fn utf8(theme : &FormattingTheme) -> Self {
        Self {
            child: theme.apply_tree_colour("├── "), 
            last_child: theme.apply_tree_colour("└── "), 
            no_child: theme.apply_tree_colour("│   "), 
            no_branch: "    ".to_string(), 
            attribute_indent: "  ".to_string(), 
        }
    }
}


pub struct TreeStyle {
    pub branch: TreeBranchStyle, 
    pub node: NodeStyle,
    pub level: OutputLevel,
}

impl TreeStyle {
  pub fn new(settings : &StyleSettings) -> Self {
      Self {
        branch: TreeBranchStyle::new(settings),
        node: NodeStyle::new(settings),
        level: settings.level.clone(),
      }
  }
}
