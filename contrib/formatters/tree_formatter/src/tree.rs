use std::io::Write;
use std::error::Error;

use integra8_formatters::models::report::ComponentRunReport;
use integra8_formatters::models::artifacts::stdio::TestResultStdio;
use integra8_formatters::models::{ComponentResult, ComponentTimeResult};
use integra8_formatters::models::{DidNotRunReason, FailureReason, PassReason};
use integra8_formatters::models::{ComponentLocation, ComponentType};

use crate::styles::{TreeStyle, NodeStyle};
use crate::writer::{Prefix, PrefixedTextWriter};

#[derive(Clone, Debug)]
pub struct ResultsNode<'a> {
    pub report: &'a ComponentRunReport,
    pub children: Vec<ResultsNode<'a>>,
}

impl<'a> ResultsNode<'a> {
    pub fn from_report(report: &'a ComponentRunReport) -> Self {
        Self {
            report: report,
            children: Vec::new(),
        }
    }

    pub fn add_child_node(&mut self, child: ResultsNode<'a>) {
        self.children.push(child)
    }

    pub fn render_node<W: Write>(
        &self,
        style: &TreeStyle,
        output_formatter: &mut PrefixedTextWriter<W>,
    ) -> std::io::Result<()> {
        match &self.report.result {
            ComponentResult::Fail(reason) => {
                // Failure results print a summary include additional attributes summary
                let component_heading = match reason == &FailureReason::Overtime {
                    true => style.node.component_heading_with_remark(&self.report, &format!("time limit exceeded {:?}", self.report.timing.duration())),
                    false => style.node.component_heading(&self.report)
                };
                output_formatter.writeln(component_heading)?;  
                self.render_node_attributes(style,output_formatter)?;   
            }
            _ => {
                let component_heading = match self.report.timing.is_warn() {
                    true => style.node.component_heading_with_remark(&self.report, &format!("time limit warning {:?}", self.report.timing.duration())),
                    false => style.node.component_heading(&self.report)
                };
                output_formatter.writeln(&component_heading)?;
            }
        }
        Ok(())
    }

    pub fn render_node_attributes<W: Write>(
        &self,
        style: &TreeStyle,
        output_formatter: &mut PrefixedTextWriter<W>,
    ) -> std::io::Result<()> {
        let mut has_attributes = false;
        output_formatter.push(Prefix::with(&style.branch.attribute_indent));

        if let Some(description) = self.report.description.description.as_ref() {
            self.render_attribute(style, "description", description, output_formatter)?;
            has_attributes = true;
        }
   
        if let Some(src) = self.report.description.location.as_ref() {
            self.render_attribute(style, "location", &src.hotlink_text(), output_formatter)?;
            has_attributes = true;
        }
        
        if let Some(std_out) = self.report.artifacts.stdio.stdout_utf8() {
            self.render_attribute(style, "stdout", std_out.unwrap(), output_formatter)?;
            has_attributes = true;
        }

        if let Some(std_err) = self.report.artifacts.stdio.stderr_utf8() {
            self.render_attribute(style, "stderr", std_err.unwrap(), output_formatter)?;
            has_attributes = true;
        }

        if has_attributes {
            output_formatter.write_newline()?;
        }
        output_formatter.pop();
    
       Ok(())
    }

    pub fn render_attribute<W: Write>(
        &self,
        style: &TreeStyle,
        attribute_name: &str,
        attribute_text: &str,
        output_formatter: &mut PrefixedTextWriter<W>,
    ) -> std::io::Result<()> {

        let attribute_text_lines : Vec<&str> = attribute_text.lines().collect();
        match attribute_text_lines.len() {
            0 => {
                // Skip this element
            },
            1 => {
                // write on a single line
                output_formatter.writeln(format!(
                    "{} {}",
                    style.node.attribute_style(attribute_name),
                    attribute_text_lines[0]
                ))?;
            },
            _ => {
                // write on many lines
                output_formatter.writeln(format!(
                    "{}", style.node.attribute_style(attribute_name)
                ))?;
                output_formatter.push(Prefix::with(&style.branch.attribute_indent));
                for line in attribute_text_lines {
                    output_formatter.writeln(line)?;
                }
                output_formatter.pop();
            }
        }
        Ok(())
    }
}

pub struct ResultsTree<'a> {
    root : ResultsNode<'a>
}

impl<'a> ResultsTree<'a> {
    pub fn new(root: ResultsNode<'a>) -> Self {
        Self { 
            root
        }
    }

    pub fn render_tree<W: Write>(&self, writer: W, style: &TreeStyle)  -> std::io::Result<()> {
        let mut prefixed_text_writer = PrefixedTextWriter::new(writer);
        // Ensure we are on a new line, in when running in non child process mode
        // its possible there is ran text logged on the current line
        prefixed_text_writer.write_newline()?;
        self.render_node_branches(&mut prefixed_text_writer, style, &self.root)
    }

    pub fn render_node_branches<W: Write>(
        &self,
        prefixed_writer: &mut PrefixedTextWriter<W>,
        style: &TreeStyle,
        node: &ResultsNode,
    ) -> std::io::Result<()> {
        
        self.render_node(prefixed_writer, style, node)?;

        if let Some((last_child, children)) = node.children.split_last() {
            for child in children {
                self.render_branch(prefixed_writer, style, child)?;
            }
            self.render_last_branch(prefixed_writer, style, last_child)?;
        }

        Ok(())
    }

    fn render_branch<W: Write>(
        &self,
        prefixed_writer: &mut PrefixedTextWriter<W>,
        style: &TreeStyle,
        node: &ResultsNode,
    )  -> std::io::Result<()> {

        prefixed_writer.push(
            Prefix::with(&style.branch.child).
                then_next(&style.branch.no_child)
        );
        let res = self.render_node_branches(prefixed_writer, style, node);
        prefixed_writer.pop();
        res
    }

    fn render_last_branch<W: Write>(
        &self,
        prefixed_writer: &mut PrefixedTextWriter<W>,
        style: &TreeStyle,
        node: &ResultsNode,
    ) -> std::io::Result<()> {
        prefixed_writer.push(
            Prefix::with(&style.branch.last_child).
                then_next(&style.branch.no_branch)
        );
        let res = self.render_node_branches(prefixed_writer, style, node);
        prefixed_writer.pop();
        res
    }

    fn render_node<W: Write>(
        &self,
        prefixed_writer: &mut PrefixedTextWriter<W>,
        style: &TreeStyle,
        node: &ResultsNode,
    )  -> std::io::Result<()> {
        // Nodes can print multiple lines of attributes, 
        // so the "next" line correct indentation is is determined
        // by if the current node has any children. 
        prefixed_writer.push(
            match node.children.is_empty() {
                true => Prefix::next_with(&style.branch.no_branch),
                false => Prefix::next_with(&style.branch.no_child),
            }
        );

        let res = node.render_node(
            style,
            prefixed_writer,
        );
        prefixed_writer.pop();
        res
    }
}