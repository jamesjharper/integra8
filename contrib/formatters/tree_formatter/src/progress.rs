use indexmap::IndexMap;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};

use crate::styles::ProgressBarStyle;

use integra8_formatters::models::report::ComponentRunReport;
use integra8_formatters::models::summary::{ComponentTypeCountSummary, RunSummary};
use integra8_formatters::models::{ComponentType, ComponentDescription};

pub struct TestProgressFormatter {
    progress: ProgressBar,
    style: ProgressBarStyle,
    in_progress: IndexMap<&'static str, String>, // Use index map to ensure newest item is always shown first
}

impl TestProgressFormatter {
    pub fn new(style: ProgressBarStyle) -> Self {
        let progress = ProgressBar::new(0);
        progress.set_style(
            ProgressStyle::default_spinner()
                .template(&style.template)
                .with_key("len", |state| format!("{}", (state.len + 1) / 2))
                .with_key("pos", |state| format!("{}", (state.pos + 1) / 2))
                .progress_chars(&style.progress_chars)
            );

        Self {
            progress,
            style,
            in_progress: IndexMap::new()
        }
    }

    pub fn notify_run_start<W: Write>(
        &mut self,
        writer: &mut W,
        summary: &ComponentTypeCountSummary,
    ) -> std::io::Result<()> {
        let noun = if summary.tests() != 1 {
            "components"
        } else {
            "component"
        };

        let total_component_count = summary.tests() +  summary.setups() +  summary.tear_downs();
        self.writeln(writer, &format!("\nrunning {} {}\n", total_component_count, noun))?;
        self.progress.set_length((total_component_count * 2) as u64);
        Ok(())
    }

    pub fn notify_run_finished<W: Write>(
        &mut self,
        writer: &mut W,
        _state: &RunSummary
    ) -> std::io::Result<()> {

        self.writeln(writer, self.style.finished.clone())?;
        if !self.progress.is_hidden() {
            self.progress.finish_and_clear();
        }
        
        Ok(())
    }

    pub fn notify_component_start<W: Write>(
        &mut self,
        writer: &mut W,
        desc: &ComponentDescription,
    ) -> std::io::Result<()> {

        if desc.component_type() == &ComponentType::Suite {
            return Ok(());
        }

        self.writeln(writer, &format!("{} {}", self.style.running, desc.full_name()))?;
        if !self.progress.is_hidden() {
            self.add_in_progress(desc.path().as_str(), desc.friendly_name());          
        }
        Ok(())
    }

    pub fn notify_component_finished<W: Write>(
        &mut self,
        _writer: &mut W,
        report: &ComponentRunReport,
    ) -> std::io::Result<()>  {
        if report.description.component_type() == &ComponentType::Suite {
            return Ok(());
        }
        if !self.progress.is_hidden() {
            self.remove_in_progress(report.description.path().as_str());
        }
        Ok(())
    }

    fn writeln<W: Write, S: AsRef<str>>( 
        &mut self,
        writer: &mut W,
        msg: S
    ) -> std::io::Result<()>  {
        if self.progress.is_hidden() {
            writeln!(writer, "{}", msg.as_ref())?;
        } else {
            self.progress.println(msg);    
        }
        Ok(())
    }

    fn remove_in_progress(&mut self, path : &'static str) {
        self.in_progress.remove(path);
        self.update_in_progress();
        self.progress.inc(1);
    }

    fn add_in_progress(&mut self, path : &'static str, name : String) {

        self.in_progress.insert(path, name);
        self.update_in_progress();
        self.progress.inc(1);
    }

    fn update_in_progress(&self) {
        self.progress.set_message(self.in_progress.values().rev().map(|s| &**s).collect::<Vec<&str>>().join(", "));
    }
}