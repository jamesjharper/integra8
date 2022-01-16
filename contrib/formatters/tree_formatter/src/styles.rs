use ansi_term::Colour::{Black, Green, Purple, Red, Yellow};
use integra8_formatters::models::report::ComponentRunReport;
use integra8_formatters::models::{ComponentResult, ComponentType, PassReason};

#[derive(Clone)]
pub enum Formatting {
    Ansi,
    None
}

impl Formatting {

    pub fn new(ansi_mode: &AnsiMode) -> Self {
        match  ansi_mode.is_enabled() {
            true => Self::Ansi,
            false => Self::None, 
        }
    }

    pub fn apply_pass_formatting(&self, text: impl Into<String>) -> String {
        match self {
            Self::Ansi => Green.paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }

    pub fn apply_fail_formatting(&self, text: impl Into<String>) -> String {
        match self {
            Self::Ansi => Red.paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }

    pub fn apply_warning_formatting(&self, text: impl Into<String>) -> String {
        match self {
            Self::Ansi => Yellow.paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }

    pub fn apply_skipped_formatting(&self, text: impl Into<String>) -> String {
        match self {
            Self::Ansi => ansi_term::Style::default().dimmed().paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }

    pub fn apply_attribute_formatting(&self, text: impl Into<String>) -> String {

        match self {
            Self::Ansi => Purple.italic().paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }

    pub fn apply_tree_formatting(&self, text: impl Into<String>) -> String {
        match self {
            Self::Ansi => ansi_term::Style::default().dimmed().paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }
}

pub struct ComponentNodeStyle {
    pub pass: String,
    pub failed: String,
    pub overtime: String,
    pub skipped: String,
    pub warning: String,
}

impl ComponentNodeStyle {
    pub fn icon(&self, report: &ComponentRunReport) -> &'_ str {
        match report.result {
            ComponentResult::Pass(PassReason::Accepted) => match report.timing.is_warn() {
                true => &self.warning,
                false => &self.pass,
            },
            ComponentResult::Pass(PassReason::AcceptedWithWarning(_)) => &self.warning,
            ComponentResult::Fail(_) => &self.failed,
            ComponentResult::DidNotRun(_) => &self.skipped,
        }
    }
}

pub struct NodeStyle {
    suite: ComponentNodeStyle,
    test: ComponentNodeStyle,
    setup: ComponentNodeStyle,
    tear_down: ComponentNodeStyle,
    format: Formatting,
}

impl NodeStyle {
    pub fn new(format: &Formatting, encoding: &Encoding, style: &Style) -> Self {
        match style {
            Style::Text => Self::text(format),
            Style::Symbols => {
                match encoding {
                    Encoding::Utf8 => Self::symbols_utf8(format),
                    Encoding::Ascii => Self::symbols_ascii(format)
                }

            },
        }
    }

    pub fn text(format: &Formatting) -> Self {
        let pass = format.apply_pass_formatting("[✓]");
        let failed = format.apply_fail_formatting("[x]");
        let overtime = format.apply_fail_formatting("[⏳]");
        let skipped = format.apply_skipped_formatting("[-]");
        let warning = format.apply_warning_formatting("[!]");

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
            format: format.clone(),
        }
    }

    pub fn symbols_utf8(format: &Formatting) -> Self {
        Self {
            suite: ComponentNodeStyle {
                pass: format.apply_pass_formatting("○"),
                failed: format.apply_fail_formatting("●"),
                overtime: format.apply_fail_formatting("⊛"),
                skipped: format.apply_skipped_formatting("◌"),
                warning: format.apply_warning_formatting("◑"),
            },
            test: ComponentNodeStyle {
                pass: format.apply_pass_formatting("□"),
                failed: format.apply_fail_formatting("■"),
                overtime: format.apply_fail_formatting("▧"),
                skipped: format.apply_skipped_formatting("⬚"),
                warning: format.apply_warning_formatting("◪"),
            },
            setup: ComponentNodeStyle {
                pass: format.apply_pass_formatting("△"),
                failed: format.apply_fail_formatting("▲"),
                overtime: format.apply_fail_formatting("◭"),
                skipped: format.apply_skipped_formatting("△"),
                warning: format.apply_warning_formatting("◭"),
            },
            tear_down: ComponentNodeStyle {
                pass: format.apply_pass_formatting("▽"),
                failed: format.apply_fail_formatting("▼"),
                overtime: format.apply_fail_formatting("⧨"),
                skipped: format.apply_skipped_formatting("▽"),
                warning: format.apply_warning_formatting("⧨"),
            },
            format: format.clone()
        }
    }

    pub fn symbols_ascii(format: &Formatting) -> Self {
        Self {
            suite: ComponentNodeStyle {
                pass: format.apply_pass_formatting("( )"),
                failed: format.apply_fail_formatting("(x)"),
                overtime: format.apply_fail_formatting("(*)"),
                skipped: format.apply_skipped_formatting("(-)"),
                warning: format.apply_warning_formatting("(!)"),
            },
            test: ComponentNodeStyle {
                pass: format.apply_pass_formatting("[ ]"),
                failed: format.apply_fail_formatting("[x]"),
                overtime: format.apply_fail_formatting("[*]"),
                skipped: format.apply_skipped_formatting("[-]"),
                warning: format.apply_warning_formatting("[!]"),
            },
            setup: ComponentNodeStyle {
                pass: format.apply_pass_formatting("/ \\"),
                failed: format.apply_fail_formatting("/x\\"),
                overtime: format.apply_fail_formatting("/*\\"),
                skipped: format.apply_skipped_formatting("/-\\"),
                warning: format.apply_warning_formatting("/!\\"),
            },
            tear_down: ComponentNodeStyle {
                pass: format.apply_pass_formatting("\\ /"),
                failed: format.apply_fail_formatting("\\x/"),
                overtime: format.apply_fail_formatting("\\*/"),
                skipped: format.apply_skipped_formatting("\\-/"),
                warning: format.apply_warning_formatting("\\!/"),
            },
            format: format.clone(),
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

    pub fn component_heading_with_remark(
        &self,
        report: &ComponentRunReport,
        remark: &str,
    ) -> String {
        let icon = self.icon_style(report).icon(report);
        format!(
            "{} - {} ({})",
            icon,
            report.description.friendly_name(),
            remark
        )
    }

    pub fn attribute_style(&self, attribute_name: &str) -> String {
        self.format.apply_attribute_formatting(&format!("{}:", attribute_name))
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
    pub fn new(format: &Formatting, encoding: &Encoding) -> Self {

        match encoding {
            Encoding::Utf8 => Self::utf8(format),
            Encoding::Ascii => Self::ascii(format),
        }
    }

    pub fn ascii(format: &Formatting) -> Self {
        Self {
            child: format.apply_tree_formatting(" ¦-- "),
            last_child: format.apply_tree_formatting(" '-- "),
            no_child: format.apply_tree_formatting(" ¦   "),
            no_branch: "     ".to_string(),
            attribute_indent: "  ".to_string(),
        }
    }

    pub fn utf8(format: &Formatting) -> Self {
        Self {
            child: format.apply_tree_formatting("├── "),
            last_child: format.apply_tree_formatting("└── "),
            no_child: format.apply_tree_formatting("│   "),
            no_branch: "    ".to_string(),
            attribute_indent: "  ".to_string(),
        }
    }
}

use crate::Encoding;
use crate::DetailLevel;
use crate::Style;
use crate::AnsiMode;

pub struct TreeStyle {
    pub branch: TreeBranchStyle,
    pub node: NodeStyle,
    pub detail_level: DetailLevel,
}

impl TreeStyle {

    pub fn new(
        style: Style,
        detail_level: DetailLevel,
        encoding: Encoding,
        ansi_mode: AnsiMode,
    ) -> Self {

        let format = Formatting::new(&ansi_mode);
        Self {
            branch: TreeBranchStyle::new(&format, &encoding),
            node: NodeStyle::new(&format, &encoding, &style ),
            detail_level: detail_level,
        }
    } 


    /*pub fn new(settings: &StyleSettings) -> Self {
        Self {
            branch: TreeBranchStyle::new(settings),
            node: NodeStyle::new(settings),
            level: settings.level.clone(),
        }
    }*/
}
