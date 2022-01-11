use integra8_formatters::OutputLocation;
use integra8_formatters::models::report::ComponentRunReport;

use integra8_formatters::models::artifacts::stdio::TestResultStdio;
use integra8_formatters::models::{ComponentResult, ComponentTimeResult};
use integra8_formatters::models::{DidNotRunReason, FailureReason, PassReason};

use integra8_formatters::models::{ComponentLocation, ComponentType};

use crate::styles::{TreeStyle, NodeStyle};

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
        style: &NodeStyle,
        output_formatter: &mut PrefixedOutputWriter<'_>,
        out: &mut OutputLocation,
    ) -> Result<(), Box<dyn Error>> {
        //out.write_plain(prefix)?;

        //△ ▲
        //▽ ▼
        //■ □ ▧ ▨
        let type_icon_style = style.icon_style(&self.component_type);
        
        /*match self.component_type {
            ComponentType::Suite => "○", //"☰", //"○",
            ComponentType::Test => "▧",
            ComponentType::Setup => "▲",
            ComponentType::TearDown => "▼",
        };*/

       // output_formatter.write(out, &format!("{} - ", type_icon))?;

        //output_formatter.write_newline(out)?;
        //Ok(())
        //out.write_plain(format!("{} - ", type_icon))?;
        match self.result {
            ComponentResult::Pass(PassReason::Accepted) => {
                if self.timing.is_warn() {
                    output_formatter.writeln(out, &format!("{} - {} (time limit warning) - {:?} ", type_icon_style.warning, self.display_name, self.timing.duration()))?;
                } else {

                    output_formatter.writeln(out, &format!("{} - {}", type_icon_style.pass, self.display_name, ))?;
                }
            }
            ComponentResult::Pass(PassReason::FailureAllowed) => {
                output_formatter.writeln(out, &format!("{} - {} (allowed)", type_icon_style.warning, self.display_name))?;           
            }
            ComponentResult::Fail(FailureReason::Rejected) => {
                output_formatter.writeln(out, &format!("{} - {}", type_icon_style.failed, self.display_name))?;   

                match self.component_type {
                    ComponentType::Suite => {
                        // dont log out anything else
                    }
                    _ => {

                        if let Some(src) = self.src_location.as_ref() {
                            output_formatter.writeln(out, &format!(
                                "      location: {}",
                                src.hotlink_text()
                            ))?;
                        }
                       

                        if let Some(std_out) = self.stdio.stdout_utf8() {

                            let mut stdout_points = output_formatter.append_prefix("      ", "      ");
                            stdout_points.writeln(out, &"stdout:".to_string())?;
                            stdout_points.writeln(out, &"```".to_string())?;
                            for line in std_out.unwrap().lines() {
                                stdout_points.writeln(out, &line)?;
                            }
                            stdout_points.writeln(out, &"```".to_string())?;
                        }

                        if let Some(std_err) = self.stdio.stderr_utf8() {


                            let mut stderr_points = output_formatter.append_prefix("      ", "      ");
                            stderr_points.writeln(out, &"stderr:".to_string())?;
                            stderr_points.writeln(out, &"```".to_string())?;
                            for line in std_err.unwrap().lines() {
                                stderr_points.writeln(out, &line)?;
                            }
                            stderr_points.writeln(out, &"```".to_string())?;

                        }
                    }
                };
            }
            ComponentResult::Fail(FailureReason::Overtime) => {
                output_formatter.writeln(out, &format!("{} - {} - (time limit exceeded) - {:?}", type_icon_style.overtime, self.display_name, self.timing.duration()))?;   
            }
            ComponentResult::Fail(FailureReason::ChildFailure) => {
                output_formatter.writeln(out, &format!("{} - {}", type_icon_style.failed, self.display_name))?;   
            }
            ComponentResult::DidNotRun(_) => {
                output_formatter.writeln(out, &format!("{} - {}", type_icon_style.skipped, self.display_name))?;   
            }
        }
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
        self.print_node(
            &TreeStyle::standard_colour(),
            &mut PrefixedOutputWriter::new(), 
        root).unwrap();
    }

    pub fn print_node(
        &mut self,
        tree_style: &TreeStyle,
        output_formatter: &mut PrefixedOutputWriter<'_>,
        node: &ComponentResultsTreeNode,
    ) -> Result<(), Box<dyn Error>> {
        node.render_node(
            &tree_style.node,
            output_formatter, 
            &mut self.out)
            .unwrap();

        if let Some((last_child, children)) = node.children.split_last() {
        
            for child in children {
                let mut mid_node_formatter = output_formatter.append_prefix(&tree_style.branch.child, &tree_style.branch.no_child);
                self.print_node(tree_style, &mut mid_node_formatter, child)?;
            }

            let mut last_node_formatter = output_formatter.append_prefix(&tree_style.branch.last_child, &tree_style.branch.no_branch);

            self.print_node(tree_style, &mut last_node_formatter, last_child)?;
        }

        Ok(())
    }
}






pub struct PrefixSegment<'a> {
    segments: Vec<&'a str>
}


impl<'a> PrefixSegment<'a> {
    pub fn new() -> Self {
        Self {
            segments: vec![]
        }
    }

    pub fn append(&self, val : &'a str ) -> Self {
        let mut segments = self.segments.clone();
        segments.push(val);
        Self {
            segments: segments
        }
    }
}

use std::fmt;

impl<'a> fmt::Display for PrefixSegment<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for seg in &self.segments {
            write!(f, "{}", seg)?;
        }
        Ok(())
    }
}


use std::mem;
use std::io::Write;

pub struct PrefixedOutputWriter<'a> {
    new_line: bool,
    current_prefix: PrefixSegment<'a>,
    next_prefix: Option<PrefixSegment<'a>>,
}

impl<'a> PrefixedOutputWriter<'a> {

    pub fn new() -> Self {
        Self {
            current_prefix: PrefixSegment::new(),
            next_prefix: None,
            new_line: true
        }
    }

    pub fn append_prefix(&self, current : &'a str, next : &'a str,) -> Self {
        Self {
            new_line: true,
            current_prefix: self.next_prefix
                .as_ref()
                .map(|n| n.append(current))
                .unwrap_or_else(|| self.current_prefix.append(current)),
            next_prefix: self.next_prefix
                .as_ref()
                .map(|n| Some(n.append(next)))
                .unwrap_or_else(|| Some(self.current_prefix.append(next))),

        }
    }

    pub fn write<W: Write, D: fmt::Display>(&mut self, writer:  &mut W, display: &D) -> Result<(), Box<dyn Error>>{
        if self.new_line {
            self.new_line = false;
            write!(writer, "{}{}",  self.current_prefix, display)?;
            Ok(())
        } else {
            write!(writer, "{}", display)?;
            Ok(())
        }
    } 


    pub fn writeln<W: Write, D: fmt::Display>(&mut self, writer:  &mut W, display: &D) -> Result<(), Box<dyn Error>>{
        if self.new_line {
            
            writeln!(writer, "{}{}",  self.current_prefix, display)?;
        } else {
            writeln!(writer, "{}", display)?;
        }

        self.new_line = true;
        // Move to the next line if we have one
        if let Some(next) = mem::take(&mut self.next_prefix) {
           self.current_prefix = next;
        }
        Ok(())
    } 

    pub fn write_newline<W: Write>(&mut self, writer:  &mut W) -> Result<(), Box<dyn Error>>{
        // Move to the next line if we have one
        if let Some(next) =  mem::take(&mut self.next_prefix) {
            self.current_prefix = next;
        }

        writeln!(writer, "")?;
        Ok(())
    } 
}