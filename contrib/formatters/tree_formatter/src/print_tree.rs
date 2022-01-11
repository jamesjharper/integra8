use integra8_formatters::OutputLocation;
use integra8_formatters::models::report::ComponentRunReport;

use integra8_formatters::models::artifacts::stdio::TestResultStdio;
use integra8_formatters::models::{ComponentResult, ComponentTimeResult};
use integra8_formatters::models::{DidNotRunReason, FailureReason, PassReason};

use integra8_formatters::models::{ComponentLocation, ComponentType};

use std::error::Error;

#[derive(Clone, Debug)]
pub struct ComponentResultsTreeNode {
    pub display_name: String,
    pub result: ComponentResult,
    pub timing: ComponentTimeResult,
    pub src_location: Option<ComponentLocation>,
    pub component_type: ComponentType,
    pub stdio: TestResultStdio,
    pub children: Vec<ComponentResultsTreeNode>,
}

impl ComponentResultsTreeNode {
    pub fn from_report(report: &ComponentRunReport) -> Self {
        Self {
            display_name: report.description.friendly_name(),
            src_location: report.description.location.clone(),
            result: report.result.clone(),
            timing: report.timing.clone(),
            component_type: report.description.component_type.clone(),
            stdio: report.artifacts.stdio.clone(),
            children: Vec::new(),
        }
    }

    pub fn add_child_report(&mut self, report: &ComponentRunReport) {
        self.children.push(Self::from_report(report))
    }

    pub fn add_child_node(&mut self, child: ComponentResultsTreeNode) {
        self.children.push(child)
    }

    pub fn render_node(
        &self,
        prefix: &str,
        next_prefix: &str,
        out: &mut OutputLocation,
    ) -> Result<(), Box<dyn Error>> {
        out.write_plain(prefix)?;

        //△ ▲
        //▽ ▼
        //■ □ ▧ ▨
        let type_icon = match self.component_type {
            ComponentType::Suite => "○", //"☰", //"○",
            ComponentType::Test => "▧",
            ComponentType::Setup => "▲",
            ComponentType::TearDown => "▼",
        };
        //out.write_plain(format!("{} - ", type_icon))?;
        match self.result {
            ComponentResult::Pass(PassReason::Accepted) => {
                if self.timing.is_warn() {
                    out.write_pretty(type_icon, term::color::YELLOW)?;
                    out.write_plain(format!(
                        " - {} - (time limit warning) - {:?} ",
                        self.display_name,
                        self.timing.duration()
                    ))?;
                } else {
                    out.write_pretty(type_icon, term::color::GREEN)?;
                    out.write_plain(format!(
                        " - {} - {:?} ",
                        self.display_name,
                        self.timing.duration()
                    ))?;
                }
            }
            ComponentResult::Pass(PassReason::FailureAllowed) => {
                out.write_pretty(type_icon, term::color::YELLOW)?;
                out.write_plain(format!(" - {} - (allowed)", self.display_name))?;
            }
            ComponentResult::Fail(FailureReason::Rejected) => {
                out.write_pretty(type_icon, term::color::RED)?;
                out.write_plain(" - ")?;
                out.write_pretty(&self.display_name, term::color::RED)?;
                out.write_plain(format!("  - {:?} ", self.timing.duration()))?;

                let _type_icon = match self.component_type {
                    ComponentType::Suite => {
                        // dont log out anything else
                    }
                    _ => {
                        out.write_plain("\n")?;

                        out.write_plain(next_prefix)?;
                       
                        if let Some(src) = self.src_location.as_ref() {
                            out.write_plain(format!(
                                "    - location: {}",
                                src.hotlink_text()
                            ))?;
                        }
                       
                        out.write_plain("\n")?;

                        if let Some(std_out) = self.stdio.stdout_utf8() {
                            out.write_plain(next_prefix)?;
                            out.write_plain("    - stdout:")?;

                            out.write_plain("\n")?;
                            out.write_plain(next_prefix)?;
                            out.write_plain("      ```")?;

                            for line in std_out.unwrap().lines() {
                                out.write_plain("\n")?;
                                out.write_plain(next_prefix)?;
                                out.write_plain(format!("        {}", line))?;
                            }
                            out.write_plain("\n")?;
                            out.write_plain(next_prefix)?;
                            out.write_plain("      ```")?;
                            out.write_plain("\n")?;
                        }

                        if let Some(std_out) = self.stdio.stderr_utf8() {
                            out.write_plain(next_prefix)?;
                            out.write_plain("    - stderr:")?;

                            out.write_plain("\n")?;
                            out.write_plain(next_prefix)?;
                            out.write_plain("      ```")?;
                            for line in std_out.unwrap().lines() {
                                out.write_plain("\n")?;
                                out.write_plain(next_prefix)?;
                                out.write_plain(format!("        {}", line))?;
                            }

                            out.write_plain("\n")?;
                            out.write_plain(next_prefix)?;
                            out.write_plain("      ```")?;
                        }
                    }
                };
            }
            ComponentResult::Fail(FailureReason::Overtime) => {
                out.write_pretty(type_icon, term::color::RED)?;
                out.write_plain(format!(
                    " - {} - (time limit exceeded) - {:?} ",
                    self.display_name,
                    self.timing.duration()
                ))?;
            }
            ComponentResult::Fail(FailureReason::ChildFailure) => {
                out.write_pretty(type_icon, term::color::RED)?;
                out.write_plain(format!(
                    " - {} - {:?} ",
                    self.display_name,
                    self.timing.duration()
                ))?;
            }
            ComponentResult::DidNotRun(DidNotRunReason::Undetermined) => {
                out.write_pretty(type_icon, term::color::BRIGHT_BLACK)?;
                out.write_plain(" - ")?;
                out.write_pretty(&self.display_name, term::color::BRIGHT_BLACK)?;
            }
            ComponentResult::DidNotRun(DidNotRunReason::Filtered) => {
                out.write_pretty(type_icon, term::color::BRIGHT_BLACK)?;
                out.write_plain(" - ")?;
                out.write_pretty(&self.display_name, term::color::BRIGHT_BLACK)?;
            }
            ComponentResult::DidNotRun(DidNotRunReason::ParentFailure) => {
                out.write_pretty(type_icon, term::color::BRIGHT_BLACK)?;
                out.write_plain(" - ")?;
                out.write_pretty(&self.display_name, term::color::BRIGHT_BLACK)?;
            }
            ComponentResult::DidNotRun(DidNotRunReason::Ignored) => {
                out.write_pretty(type_icon, term::color::BRIGHT_BLACK)?;
                out.write_plain(" - ")?;
                out.write_pretty(&self.display_name, term::color::BRIGHT_BLACK)?;
            }
        }

        out.write_plain("\n")?;
        Ok(())
    }
}

pub struct TreeNodePrinter<'a> {
    out: &'a mut OutputLocation,
}

impl<'a> TreeNodePrinter<'a> {
    pub fn new(out: &'a mut OutputLocation) -> Self {
        Self { out }
    }

    pub fn print_tree(&mut self, root: &ComponentResultsTreeNode) {
        self.print_node("", "", root).unwrap();
    }

    pub fn print_node(
        &mut self,
        prefix: &str,
        next_prefix: &str,
        node: &ComponentResultsTreeNode,
    ) -> Result<(), Box<dyn Error>> {
        node.render_node(prefix, next_prefix, &mut self.out)
            .unwrap();

        if let Some((last_child, children)) = node.children.split_last() {
            let mid_prefix = format!("{}{}", next_prefix.clone(), "├── ");
            let mid_next_prefix = format!("{}{}", next_prefix.clone(), "│   ");
            for child in children {
                self.print_node(&mid_prefix, &mid_next_prefix, child)?;
            }

            let last_prefix = format!("{}{}", next_prefix.clone(), "└── ");
            let last_next_prefix = format!("{}{}", next_prefix.clone(), "    ");
            self.print_node(&last_prefix, &last_next_prefix, last_child)?;
        }

        Ok(())
    }
}
