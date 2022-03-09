use std::io::Write;
use std::time::Duration;

use crate::parameters::DetailLevel;
use crate::styles::TreeStyle;
use crate::writer::{Prefix, PrefixedTextWriter};

use integra8_formatters::models::report::ComponentRunReport;
use integra8_formatters::models::ComponentResult;
use integra8_formatters::models::ComponentType;
use integra8_formatters::models::{FailureReason, WarningReason};

pub fn render_component_heading<W: Write>(
    output_formatter: &mut PrefixedTextWriter<W>,
    report: &ComponentRunReport,
    style: &TreeStyle,
    detail_level: &DetailLevel,
) -> std::io::Result<()> {
    let component_heading = match &report.result {
        ComponentResult::Fail(FailureReason::Overtime) => style.node.component_heading_with_remark(
            report,
            report.description.friendly_name(),
            &format!(
                "time limit exceeded {}",
                render_human_time(&report.timing.duration())
            ),
        ),
        ComponentResult::Warning(WarningReason::FailureAllowed) => {
            style.node.component_heading_with_remark(
                report,
                report.description.friendly_name(),
                "failure allowed",
            )
        }
        ComponentResult::Warning(WarningReason::OvertimeWarning) => {
            style.node.component_heading_with_remark(
                report,
                report.description.friendly_name(),
                &format!(
                    "time limit warning {}",
                    render_human_time(&report.timing.duration())
                ),
            )
        }
        ComponentResult::DidNotRun(_) => style
            .node
            .component_heading(report, report.description.friendly_name()),
        _ => {
            if detail_level != &DetailLevel::StopWatch {
                style
                    .node
                    .component_heading(report, report.description.friendly_name())
            } else {
                style.node.component_heading_with_remark(
                    report,
                    report.description.friendly_name(),
                    &format!("{}", render_human_time(&report.timing.duration())),
                )
            }
        }
    };

    output_formatter.writeln(component_heading)?;
    Ok(())
}

pub fn render_node_attributes<W: Write>(
    output_formatter: &mut PrefixedTextWriter<W>,
    report: &ComponentRunReport,
    style: &TreeStyle,
    detail_level: &DetailLevel,
) -> std::io::Result<()> {
    let mut has_attributes = false;
    output_formatter.push(Prefix::with(&style.branch.attribute_indent));

    if let Some(description) = report.description.description() {
        render_attribute(output_formatter, style, "description", description)?;
        has_attributes = true;
    }

    // Don't write location for suites
    if report.description.component_type() != &ComponentType::Suite {
        render_attribute(
            output_formatter,
            style,
            "src",
            &report.description.location().hotlink_text(),
        )?;
        has_attributes = true;
    }

    if detail_level != &DetailLevel::StopWatch {
        let duration = report.timing.duration();

        // Only print duration if the test takes longer then 2 seconds
        if duration > Duration::new(2, 0) {
            render_attribute(
                output_formatter,
                style,
                "duration",
                &render_human_time(&report.timing.duration()),
            )?;
            has_attributes = true;
        }
    }

    for (key, artifact) in &report.artifacts.map {
        match &artifact.as_string() {
            Ok(val) => render_attribute(output_formatter, style, &key, &val)?,
            Err(err) => render_attribute(
                output_formatter,
                style,
                &key,
                &format!("Failed to render artifact ot string, {}", err.to_string()),
            )?,
        }

        has_attributes = true;
    }

    if has_attributes {
        output_formatter.write_newline()?;
    }
    output_formatter.pop();

    Ok(())
}

pub fn render_attribute<W: Write>(
    output_formatter: &mut PrefixedTextWriter<W>,
    style: &TreeStyle,
    attribute_name: &str,
    attribute_text: &str,
) -> std::io::Result<()> {
    let attribute_text_lines: Vec<&str> = attribute_text.lines().collect();
    match attribute_text_lines.len() {
        0 => {
            // Skip this element
        }
        1 => {
            // write on a single line
            output_formatter.writeln(format!(
                "{} {}",
                style.node.attribute_style(attribute_name),
                attribute_text_lines[0]
            ))?;
        }
        _ => {
            // write on many lines
            output_formatter.writeln(format!("{}", style.node.attribute_style(attribute_name)))?;
            output_formatter.push(Prefix::with(&style.branch.attribute_indent));
            for line in attribute_text_lines {
                output_formatter.writeln(line)?;
            }
            output_formatter.pop();
        }
    }
    Ok(())
}

pub fn render_human_time(duration: &Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;

    if hours != 0 {
        if minutes != 0 {
            return format!("{}h {}m", hours, minutes);
        } else {
            return format!("{}h", hours);
        }
    }

    if minutes != 0 {
        if seconds != 0 {
            return format!("{}m {}s", minutes, seconds);
        } else {
            return format!("{}m", minutes);
        }
    }
    // Otherwise use default time formatter
    format!("{:?}", duration)
}
