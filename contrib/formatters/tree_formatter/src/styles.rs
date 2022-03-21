use ansi_term::Colour::{Cyan, Green, Purple, Red, Yellow};
use integra8::formatters::models::report::ComponentRunReport;
use integra8::formatters::models::{ComponentResult, ComponentType};

use crate::parameters::AnsiMode;
use crate::parameters::Encoding;
use crate::parameters::Style;

#[derive(Clone)]
pub enum Formatting {
    Ansi,
    None,
}

impl Formatting {
    pub fn new(ansi_mode: &AnsiMode) -> Self {
        match ansi_mode.is_enabled() {
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
            Self::Ansi => ansi_term::Style::default()
                .dimmed()
                .paint(text.into())
                .to_string(),
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
            Self::Ansi => ansi_term::Style::default()
                .dimmed()
                .paint(text.into())
                .to_string(),
            Self::None => text.into(),
        }
    }

    pub fn apply_progress_bar_running(&self, text: impl Into<String>) -> String {
        match self {
            Self::Ansi => Green.bold().paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }

    pub fn apply_progress_bar_in_progress(&self, text: impl Into<String>) -> String {
        match self {
            Self::Ansi => Cyan.bold().paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }

    pub fn apply_progress_bar_finished(&self, text: impl Into<String>) -> String {
        match self {
            Self::Ansi => Green.bold().paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }

    pub fn apply_progress_bar_failed(&self, text: impl Into<String>) -> String {
        match self {
            Self::Ansi => Red.bold().paint(text.into()).to_string(),
            Self::None => text.into(),
        }
    }
}

pub struct ComponentStyle {
    pub pass: String,
    pub failed: String,
    pub overtime: String,
    pub skipped: String,
    pub warning: String,
    format: Formatting,
}

impl ComponentStyle {
    pub fn icon(&self, report: &ComponentRunReport) -> &'_ str {
        match report.result {
            ComponentResult::Pass(_) => &self.pass,
            ComponentResult::Warning(_) => &self.warning,
            ComponentResult::Fail(_) => &self.failed,
            ComponentResult::DidNotRun(_) => &self.skipped,
        }
    }

    pub fn apply_heading_formatting(
        &self,
        report: &ComponentRunReport,
        text: impl Into<String>,
    ) -> String {
        match report.result {
            ComponentResult::Pass(_) => self.format.apply_pass_formatting(text),
            ComponentResult::Warning(_) => self.format.apply_warning_formatting(text),
            ComponentResult::Fail(_) => self.format.apply_fail_formatting(text),
            ComponentResult::DidNotRun(_) => self.format.apply_skipped_formatting(text),
        }
    }
}

pub struct ComponentTypeStyle {
    suite: ComponentStyle,
    test: ComponentStyle,
    setup: ComponentStyle,
    tear_down: ComponentStyle,
    format: Formatting,
}

impl ComponentTypeStyle {
    pub fn new(format: &Formatting, encoding: &Encoding, style: &Style) -> Self {
        match style {
            Style::Text => Self::text(format, encoding),
            Style::Symbols => match encoding {
                Encoding::Utf8 => Self::symbols_utf8(format),
                Encoding::Ascii => Self::symbols_ascii(format),
            },
        }
    }

    pub fn text(format: &Formatting, encoding: &Encoding) -> Self {
        let pass = match encoding {
            Encoding::Utf8 => format.apply_pass_formatting("[✓]"),
            Encoding::Ascii => format!("[{}]", format.apply_pass_formatting("ok")),
        };

        let failed = match encoding {
            Encoding::Utf8 => format.apply_fail_formatting("[x]"),
            Encoding::Ascii => format!("[{}]", format.apply_fail_formatting("FAIL")),
        };

        let overtime = match encoding {
            Encoding::Utf8 => format.apply_fail_formatting("[⧗]"),
            Encoding::Ascii => format!("[{}]", format.apply_fail_formatting("TIME")),
        };

        let skipped = match encoding {
            Encoding::Utf8 => format.apply_skipped_formatting("[-]"),
            Encoding::Ascii => format.apply_skipped_formatting("[skipped]"),
        };

        let warning = match encoding {
            Encoding::Utf8 => format.apply_warning_formatting("[!]"),
            Encoding::Ascii => format!("[{}]", format.apply_warning_formatting("WARN")),
        };

        Self {
            suite: ComponentStyle {
                pass: format!("{} Suite", pass),
                failed: format!("{} Suite", failed),
                overtime: format!("{} Suite", overtime),
                skipped: format!("{} Suite", skipped),
                warning: format!("{} Suite", warning),
                format: format.clone(),
            },
            test: ComponentStyle {
                pass: format!("{} Test", pass),
                failed: format!("{} Test", failed),
                overtime: format!("{} Test", overtime),
                skipped: format!("{} Test", skipped),
                warning: format!("{} Test", warning),
                format: format.clone(),
            },
            setup: ComponentStyle {
                pass: format!("{} Setup", pass),
                failed: format!("{} Setup", failed),
                overtime: format!("{} Setup", overtime),
                skipped: format!("{} Setup", skipped),
                warning: format!("{} Setup", warning),
                format: format.clone(),
            },
            tear_down: ComponentStyle {
                pass: format!("{} Tear Down", pass),
                failed: format!("{} Tear Down", failed),
                overtime: format!("{} Tear Down", overtime),
                skipped: format!("{} Tear Down", skipped),
                warning: format!("{} Tear Down", warning),
                format: format.clone(),
            },
            format: format.clone(),
        }
    }

    pub fn symbols_utf8(format: &Formatting) -> Self {
        Self {
            suite: ComponentStyle {
                pass: format.apply_pass_formatting("○"),
                failed: format.apply_fail_formatting("●"),
                overtime: format.apply_fail_formatting("⊛"),
                skipped: format.apply_skipped_formatting("◌"),
                warning: format.apply_warning_formatting("◑"),
                format: format.clone(),
            },
            test: ComponentStyle {
                pass: format.apply_pass_formatting("□"),
                failed: format.apply_fail_formatting("■"),
                overtime: format.apply_fail_formatting("▧"),
                skipped: format.apply_skipped_formatting("⬚"),
                warning: format.apply_warning_formatting("◪"),
                format: format.clone(),
            },
            setup: ComponentStyle {
                pass: format.apply_pass_formatting("△"),
                failed: format.apply_fail_formatting("▲"),
                overtime: format.apply_fail_formatting("◭"),
                skipped: format.apply_skipped_formatting("△"),
                warning: format.apply_warning_formatting("◭"),
                format: format.clone(),
            },
            tear_down: ComponentStyle {
                pass: format.apply_pass_formatting("▽"),
                failed: format.apply_fail_formatting("▼"),
                overtime: format.apply_fail_formatting("⧨"),
                skipped: format.apply_skipped_formatting("▽"),
                warning: format.apply_warning_formatting("⧨"),
                format: format.clone(),
            },
            format: format.clone(),
        }
    }

    pub fn symbols_ascii(format: &Formatting) -> Self {
        Self {
            suite: ComponentStyle {
                pass: format.apply_pass_formatting("( )"),
                failed: format.apply_fail_formatting("(x)"),
                overtime: format.apply_fail_formatting("(*)"),
                skipped: format.apply_skipped_formatting("(-)"),
                warning: format.apply_warning_formatting("(!)"),
                format: format.clone(),
            },
            test: ComponentStyle {
                pass: format.apply_pass_formatting("[ ]"),
                failed: format.apply_fail_formatting("[x]"),
                overtime: format.apply_fail_formatting("[*]"),
                skipped: format.apply_skipped_formatting("[-]"),
                warning: format.apply_warning_formatting("[!]"),
                format: format.clone(),
            },
            setup: ComponentStyle {
                pass: format.apply_pass_formatting("/ \\"),
                failed: format.apply_fail_formatting("/x\\"),
                overtime: format.apply_fail_formatting("/*\\"),
                skipped: format.apply_skipped_formatting("/-\\"),
                warning: format.apply_warning_formatting("/!\\"),
                format: format.clone(),
            },
            tear_down: ComponentStyle {
                pass: format.apply_pass_formatting("\\ /"),
                failed: format.apply_fail_formatting("\\x/"),
                overtime: format.apply_fail_formatting("\\*/"),
                skipped: format.apply_skipped_formatting("\\-/"),
                warning: format.apply_warning_formatting("\\!/"),
                format: format.clone(),
            },
            format: format.clone(),
        }
    }

    pub fn node_style<'a>(&'a self, report: &ComponentRunReport) -> &'a ComponentStyle {
        match report.description.component_type() {
            ComponentType::Suite => &self.suite,
            ComponentType::Test => &self.test,
            ComponentType::Setup => &self.setup,
            ComponentType::TearDown => &self.tear_down,
        }
    }

    pub fn component_heading(
        &self,
        report: &ComponentRunReport,
        name: impl Into<String>,
    ) -> String {
        let node_style = self.node_style(report);

        let heading = node_style.apply_heading_formatting(report, name);
        let icon = node_style.icon(report);
        format!("{} - {}", icon, heading)
    }

    pub fn component_heading_with_remark(
        &self,
        report: &ComponentRunReport,
        name: impl Into<String>,
        remark: &str,
    ) -> String {
        let node_style = self.node_style(report);
        let heading = node_style.apply_heading_formatting(report, name);
        let icon = node_style.icon(report);
        format!("{} - {} ({})", icon, heading, remark)
    }

    pub fn attribute_style(&self, attribute_name: &str) -> String {
        self.format
            .apply_attribute_formatting(&format!("{}:", attribute_name))
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

#[derive(Clone)]
pub struct ProgressBarStyle {
    pub template: String,
    pub progress_chars: String,
    pub running: String,
    pub in_progress: String,
    pub finished: String,
    pub failed: String,
}

impl ProgressBarStyle {
    pub fn new(ansi_mode: &AnsiMode) -> Self {
        let format = Formatting::new(ansi_mode);
        let running = format.apply_progress_bar_running("   Running ");
        let failed = format.apply_progress_bar_failed("   Failed  ");
        let in_progress = format.apply_progress_bar_in_progress("  Progress ");
        let template = format!("{}{}", in_progress, "[{bar}] {pos}/{len} {wide_msg} ");
        let progress_chars = "=> ".to_string();
        let finished = format.apply_progress_bar_finished("  Finished ");

        Self {
            running,
            template,
            in_progress,
            progress_chars,
            finished,
            failed,
        }
    }
}

pub struct TreeStyle {
    pub branch: TreeBranchStyle,
    pub node: ComponentTypeStyle,
}

impl TreeStyle {
    pub fn new(style: &Style, encoding: &Encoding, ansi_mode: &AnsiMode) -> Self {
        let format = Formatting::new(ansi_mode);
        Self {
            branch: TreeBranchStyle::new(&format, encoding),
            node: ComponentTypeStyle::new(&format, encoding, style),
        }
    }
}
