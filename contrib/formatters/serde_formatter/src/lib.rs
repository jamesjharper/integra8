pub mod parameters;
use std::error::Error;
use crate::parameters::{AnsiMode, DetailLevel, Encoding, Style, SerdeFormatterParameters};

use integra8::formatters::models::report::ComponentRunReport;
use integra8::formatters::models::summary::RunSummary;
use integra8::formatters::models::TestParameters;
use integra8::formatters::{OutputFormatter, OutputFormatterFactory};


pub struct SerdeFormatter {

}

impl SerdeFormatter {
    pub fn new(
    ) -> Self {
        Self {

        }
    }
}

impl OutputFormatterFactory for SerdeFormatter {
    type FormatterParameters = SerdeFormatterParameters;
    fn create<T: TestParameters>(
        _formatter_parameters: &Self::FormatterParameters,
        _parameters: &T,
    ) -> Box<dyn OutputFormatter> {

        
        Box::new(SerdeFormatter::new(
        ))
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

impl OutputFormatter for SerdeFormatter {
    fn write_run_complete(&mut self, summary: &RunSummary) -> Result<(), Box<dyn Error>> {
        let mut all = summary.all().collect::<Vec<&ComponentRunReport>>();
        all.sort_unstable_by_key(|x| x.description.id().as_unique_number());

        #[cfg(feature = "yaml")]
        println!("{}",  serde_yaml::to_string(&all)?);

        #[cfg(feature = "json")]
        println!("{}",  serde_json::to_string(&all)?);
        Ok(())
    }
}