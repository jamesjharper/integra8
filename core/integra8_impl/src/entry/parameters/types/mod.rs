mod formatter_output_type;
mod main_def;
mod string_literal;
mod structopt_struct;

pub use formatter_output_type::{OutputFormatterType, OutputFormatterTypeValue};
pub use main_def::MainDefinitionValue;
pub use string_literal::StringParameterValue;
pub use structopt_struct::{StructoptStruct, StructoptStructValue};

use syn::parse::{Parse, ParseStream};
use syn::Result;
use syn::{braced, parse_quote, token, Expr, Field, Ident, Token};

use proc_macro_error::abort;

pub struct Parameter {
    pub key: String,
    pub value: ParameterValue,
}

impl Parse for Parameter {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse::<Ident>()?.to_string();
        input.parse::<Token![:]>()?;

        let value = match key.as_str() {
            "max_concurrency"
            | "setup_critical_threshold_seconds"
            | "tear_down_critical_threshold_seconds"
            | "test_critical_threshold_seconds"
            | "test_warn_threshold_seconds"
            | "test_concurrency"
            | "suite_concurrency"
            | "console_output_level"
            | "console_output_style"
            | "console_output_encoding"
            | "console_output_ansi_mode"
            | "use_child_process" => input.call(ParameterValue::parse_string_parameter)?,
            "settings" => input.call(|s| ParameterValue::parse_settings_structopt_struct(s))?,
            "console_output" => input.call(|s| ParameterValue::parse_formatter_output_type(s))?,
            other => abort!("unexpected parameter `{}`", other),
        };

        Ok(Parameter {
            key: key,
            value: value,
        })
    }
}

pub enum ParameterValue {
    StringParameter(StringParameterValue),
    StructoptStruct(StructoptStructValue),
    OutputFormatterType(OutputFormatterTypeValue),
}

impl ParameterValue {
    fn parse_string_parameter(input: ParseStream) -> Result<Self> {
        Ok(Self::StringParameter(
            input.call(StringParameterValue::parse)?,
        ))
    }

    fn parse_settings_structopt_struct(input: ParseStream) -> Result<Self> {
        let result = if input.peek(token::Brace) {
            // detect inline
            // settings: {}
            let content;
            braced!(content in input);
            let fields = content.parse_terminated(Field::parse_named)?;
            StructoptStructValue::Inline {
                type_name: parse_quote!(SettingsExtension),
                fields: fields,
            }
        } else {
            //detect external types
            // settings: $tt
            StructoptStructValue::External {
                type_name: input.parse::<Box<Expr>>()?,
            }
        };

        return Ok(Self::StructoptStruct(result));
    }

    fn parse_formatter_output_type(input: ParseStream) -> Result<Self> {
        let result = OutputFormatterTypeValue::InlineFactoryType {
            formatter_factory_type: input.parse::<Box<Expr>>()?,
        };

        return Ok(Self::OutputFormatterType(result));
    }
}
