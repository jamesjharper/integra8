mod styles;
mod tree;
mod writer;
pub mod parameters;

use std::str::FromStr;
use std::error::Error;
use std::io::{self, Write, Stdout};



use crate::styles::{
    TreeStyle,
};
use crate::tree::{ResultsNode, ResultsTree};
use crate::parameters::{TreeFormatterParameters, Style, DetailLevel, Encoding, AnsiMode};

use integra8_formatters::models::summary::{ComponentTypeCountSummary, RunSummary, SuiteSummary};
use integra8_formatters::models::{ComponentDescription, TestParameters};

use integra8_formatters::{OutputFormatter, OutputFormatterFactory};

pub struct TreeFormatter {
    writer: Stdout,
    tree_style: TreeStyle
}

impl TreeFormatter {
    pub fn new(writer: Stdout,  tree_style: TreeStyle) -> Self {
        TreeFormatter {
            writer: writer,
            tree_style: tree_style
        }
    }

    fn get_tree<'a>(&self, state: &'a RunSummary) -> ResultsTree<'a> {
        let suite = state.get_root_suite().unwrap();
        ResultsTree::new(self.get_node(state, suite))
    }

    fn get_node<'a>(
        &self,
        state: &'a RunSummary,
        suite_summary: &'a SuiteSummary,
    ) -> ResultsNode<'a> {
        let mut suite_node = ResultsNode::from_report(suite_summary.suite_report.as_ref().unwrap());

        for setup_report in &suite_summary.setups.reports {
            suite_node.add_child_node(ResultsNode::from_report(&setup_report));
        }

        for test_report in &suite_summary.tests.reports {
            suite_node.add_child_node(ResultsNode::from_report(&test_report));
        }

        for tear_down_report in &suite_summary.tear_downs.reports {
            suite_node.add_child_node(ResultsNode::from_report(&tear_down_report));
        }

        for suite_report in &suite_summary.suites.reports {
            let suite_summary = state.get_suite(&suite_report.description.path).unwrap();
            suite_node.add_child_node(self.get_node(state, suite_summary));
        }

        suite_node
    }
}


impl OutputFormatterFactory for TreeFormatter {
    type FormatterParameters = TreeFormatterParameters;
    fn create<T: TestParameters>(
        _formatter_parameters: &Self::FormatterParameters,
        parameters: &T,
    ) -> Box<dyn OutputFormatter> {

        let style = Style::from_str(parameters.console_output_style()).unwrap();
        let detail_level = DetailLevel::from_str(parameters.console_output_detail_level()).unwrap();
        let encoding = Encoding::from_str(parameters.console_output_encoding()).unwrap();
        let ansi_mode = AnsiMode::from_str(parameters.console_output_ansi_mode()).unwrap();

        let tree_style = TreeStyle::new(
            style,
            detail_level,
            encoding,
            ansi_mode,
        );

            
        Box::new(TreeFormatter::new(io::stdout(), tree_style))
    }

    fn default_style() -> &'static str {
        Style::default_value().as_str()
    }

    fn supported_styles() -> Vec<&'static str> {
        Style::list_all()
    }

    fn default_detail_levels() -> &'static str {
        DetailLevel::default_value().as_str()
    }

    fn supported_detail_levels() -> Vec<&'static str> {
        DetailLevel::list_all()
    }

    fn default_encoding() -> &'static str {
        Encoding::default_value().as_str()
    }

    fn supported_encodings() -> Vec<&'static str> {
        Encoding::list_all()
    }

    fn default_ansi_mode() -> &'static str {
        AnsiMode::default_value().as_str()
    }
    fn supported_ansi_modes() -> Vec<&'static str> {
        AnsiMode::list_all()
    }
}

impl OutputFormatter for TreeFormatter {
    fn write_run_start(
        &mut self,
        summary: &ComponentTypeCountSummary,
    ) -> Result<(), Box<dyn Error>> {
        let noun = if summary.tests() != 1 {
            "tests"
        } else {
            "test"
        };


        writeln!(self.writer,"\nrunning {} {}\n", summary.tests(), noun)?;
        Ok(())
    }

    fn write_component_start(
        &mut self,
        _desc: &ComponentDescription,
    ) -> Result<(), Box<dyn Error>> {

        write!(self.writer, ".")?;

        Ok(())
    }

    fn write_run_complete(&mut self, state: &RunSummary) -> Result<(), Box<dyn Error>> {
        self.get_tree(state)
            .render_tree(&mut self.writer, &self.tree_style)?;

        Ok(())
    }
}
