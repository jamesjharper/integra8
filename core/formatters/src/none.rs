use crate::{OutputFormatter, OutputFormatterFactory};

use structopt::StructOpt;

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
        _test_parameters: &T,
    ) -> Box<dyn OutputFormatter> {
        Box::new(NoOutputFormatter::new())
    }
}

impl OutputFormatter for NoOutputFormatter {}