use structopt::StructOpt;
use crate::formatters::{OutputFormatter, OutputFormatterFactory};

#[derive(StructOpt, Clone)] // TODO: Remove the need for clone here
pub struct NoOutputFormatterParameters {}

pub struct NoOutputFormatter {}

impl NoOutputFormatter {
    pub fn new() -> Self {
        Self {}
    }
}

impl OutputFormatterFactory for NoOutputFormatter {
    type FormatterParameters = NoOutputFormatterParameters;

    fn create<T>(
        _formatter_parameters: &Self::FormatterParameters,
        _framework: &T,
    ) -> Box<dyn OutputFormatter> {
        Box::new(NoOutputFormatter::new())
    }
}

impl OutputFormatter for NoOutputFormatter {}
