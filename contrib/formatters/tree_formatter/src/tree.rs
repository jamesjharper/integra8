use std::io::Write;

use integra8_formatters::models::report::ComponentRunReport;
use integra8_formatters::models::{ComponentResult};

use crate::parameters::DetailLevel;
use crate::styles::TreeStyle;
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

    pub fn children(&self, detail_level : &DetailLevel) -> Vec<&ResultsNode<'a>> {
        match detail_level {
            // In Error or Warn detail level, remove 
            // all the non error and warning nodes.
            // * Note: 
            //      we have to include warn nodes even
            //      when in error detail level, as the 
            //      Error nodes has have warning nodes has parents  
            DetailLevel::Error | DetailLevel::Warning => {
                self.children
                    .iter()
                    .filter(|c| c.report.result.has_failed() || c.report.result.has_warn())
                    .collect()
            },
            _ => {
                self.children.iter().collect()
            }
        }
    }

    pub fn add_child_node(&mut self, child: ResultsNode<'a>) {
        self.children.push(child)
    }

    pub fn render_node<W: Write>(
        &self,
        output_formatter: &mut PrefixedTextWriter<W>,
        style: &TreeStyle,
        detail_level: &DetailLevel
    ) -> std::io::Result<()> {

        crate::render::render_component_heading(output_formatter, &self.report, style, detail_level)?;

        let render_attributes = match &self.report.result {
            ComponentResult::Fail(_) => true,
            ComponentResult::Warning(_) => {
                detail_level != &DetailLevel::Error && detail_level != &DetailLevel::StopWatch
            },
            ComponentResult::Pass(_) => {
                detail_level == &DetailLevel::Verbose
            },
            ComponentResult::DidNotRun(_) => {
                false
            },
        };

        if render_attributes {
            crate::render::render_node_attributes(output_formatter, &self.report, style, detail_level)?;
        }
        Ok(())
    }


}



pub struct ResultsTree<'a> {
    root: ResultsNode<'a>,
}

impl<'a> ResultsTree<'a> {
    pub fn new(root: ResultsNode<'a>) -> Self {
        Self { root }
    }

    pub fn render_tree<W: Write>(&self, writer: W, style: &TreeStyle, detail_level: &DetailLevel) -> std::io::Result<()> {
        let mut prefixed_text_writer = PrefixedTextWriter::new(writer);
        // Ensure we are on a new line, in when running in non child process mode
        // its possible there is ran text logged on the current line
        prefixed_text_writer.write_newline()?;
        self.render_node_branches(&mut prefixed_text_writer, style, &self.root, detail_level)
    }

    pub fn render_node_branches<W: Write>(
        &self,
        prefixed_writer: &mut PrefixedTextWriter<W>,
        style: &TreeStyle,
        node: &ResultsNode,
        detail_level: &DetailLevel
    ) -> std::io::Result<()> {
        self.render_node(prefixed_writer, style, node, detail_level)?;

        if let Some((last_child, children)) = node.children(detail_level).split_last() {
            for child in children {
                self.render_branch(prefixed_writer, style, child, detail_level)?;
            }
            self.render_last_branch(prefixed_writer, style, last_child, detail_level)?;
        }

        Ok(())
    }

    fn render_branch<W: Write>(
        &self,
        prefixed_writer: &mut PrefixedTextWriter<W>,
        style: &TreeStyle,
        node: &ResultsNode,
        detail_level: &DetailLevel
    ) -> std::io::Result<()> {
        prefixed_writer.push(Prefix::with(&style.branch.child).then_next(&style.branch.no_child));
        let res = self.render_node_branches(prefixed_writer, style, node, detail_level);
        prefixed_writer.pop();
        res
    }

    fn render_last_branch<W: Write>(
        &self,
        prefixed_writer: &mut PrefixedTextWriter<W>,
        style: &TreeStyle,
        node: &ResultsNode,
        detail_level: &DetailLevel
    ) -> std::io::Result<()> {
        prefixed_writer
            .push(Prefix::with(&style.branch.last_child).then_next(&style.branch.no_branch));
        let res = self.render_node_branches(prefixed_writer, style, node, detail_level);
        prefixed_writer.pop();
        res
    }

    fn render_node<W: Write>(
        &self,
        prefixed_writer: &mut PrefixedTextWriter<W>,
        style: &TreeStyle,
        node: &ResultsNode,
        detail_level: &DetailLevel
    ) -> std::io::Result<()> {
        // Nodes can print multiple lines of attributes,
        // so the "next" line correct indentation is is determined
        // by if the current node has any children.
        prefixed_writer.push(match node.children.is_empty() {
            true => Prefix::next_with(&style.branch.no_branch),
            false => Prefix::next_with(&style.branch.no_child),
        });

        let res = node.render_node(prefixed_writer, style, detail_level);
        prefixed_writer.pop();
        res
    }
}
