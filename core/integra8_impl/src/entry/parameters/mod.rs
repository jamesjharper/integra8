mod types;

use types::{
    MainDefinitionValue, OutputFormatterTypeValue, ParameterValue, StringParameterValue,
    StructoptStructValue,
};
use types::{OutputFormatterType, Parameter, StructoptStruct};

use proc_macro2::TokenStream;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_quote, Result};
use syn::{Expr, Path, Token};

use proc_macro_error::abort;
use std::collections::HashMap;

pub struct ApplicationParameters {
    attrs: HashMap<String, ParameterValue>,
}

impl Parse for ApplicationParameters {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input
            .call(Punctuated::<Parameter, Token![,]>::parse_terminated)?
            .into_iter()
            .map(|i| (i.key, i.value))
            .collect::<HashMap<String, ParameterValue>>();

        Ok(Self { attrs: attrs })
    }
}

impl ApplicationParameters {
    pub fn take_max_concurrency_expr(&mut self) -> TokenStream {
        self.take_string_parameter("max_concurrency")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!("Auto"))
    }

    pub fn take_test_concurrency(&mut self) -> TokenStream {
        self.take_string_parameter("default_test_concurrency")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!("Sequential"))
    }

    pub fn take_suite_concurrency(&mut self) -> TokenStream {
        self.take_string_parameter("default_suite_concurrency")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!("Sequential"))
    }

    pub fn take_default_setup_time_limit(&mut self) -> TokenStream {
        self.take_string_parameter("default_setup_time_limit")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!("30s"))
    }

    pub fn take_tear_down_time_limit_seconds(&mut self) -> TokenStream {
        self.take_string_parameter("default_tear_down_time_limit")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!("30s"))
    }

    pub fn take_test_time_limit_seconds(&mut self) -> TokenStream {
        self.take_string_parameter("default_test_time_limit")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!("30s"))
    }

    pub fn take_test_warning_time_threshold_seconds(&mut self) -> TokenStream {
        self.take_string_parameter("default_test_warning_time_limit")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!("30s"))
    }

    pub fn take_console_output_style(
        &mut self,
        formatter_factory_type: &Box<Expr>,
        integra8_path: &Path,
    ) -> TokenStream {
        self.take_string_parameter("console_output_style")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!{
                < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::default_style()
            })
    }

    pub fn take_console_output_level(
        &mut self,
        formatter_factory_type: &Box<Expr>,
        integra8_path: &Path,
    ) -> TokenStream {
        self.take_string_parameter("console_output_level")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!{
                < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::default_detail_levels()
            })
    }

    pub fn take_console_output_encoding(
        &mut self,
        formatter_factory_type: &Box<Expr>,
        integra8_path: &Path,
    ) -> TokenStream {
        self.take_string_parameter("console_output_encoding")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!{
                < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::default_encoding()
            })
    }

    pub fn take_console_output_ansi_mode(
        &mut self,
        formatter_factory_type: &Box<Expr>,
        integra8_path: &Path,
    ) -> TokenStream {
        self.take_string_parameter("console_output_ansi_mode")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!{
                < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::default_ansi_mode()
            })
    }

    pub fn take_use_child_process(&mut self) -> TokenStream {
        self.take_string_parameter("use_child_process")
            .map(|x| x.render_tokens())
            .unwrap_or_else(|| parse_quote!("true"))
    }

    pub fn take_main(&mut self) -> TokenStream {
        MainDefinitionValue::configured_runtime().render_tokens()
    }

    pub fn take_settings_extensions(&mut self) -> StructoptStruct {
        self.take_structopt_struct("parameters")
            .unwrap_or_else(|| {
                // Detected no custom type
                StructoptStructValue::Unit {
                    type_name: parse_quote!(EmptySettingsExtension),
                }
            })
            .render_tokens()
    }

    pub fn take_console_output_formatter(&mut self) -> OutputFormatterType {
        self.take_output_formatter_type("console_output")
            .unwrap_or_else(|| OutputFormatterTypeValue::InlineFactoryType {
                formatter_factory_type: parse_quote!(
                    // TODO: should be configured via features
                    ::integra8::formatters::pretty::PrettyFormatter
                ),
            })
            .render_tokens()
    }

    fn take_output_formatter_type(
        &mut self,
        name: &'static str,
    ) -> Option<OutputFormatterTypeValue> {
        self.take(name)
            .map(|attr| {
                match attr {
                    ParameterValue::OutputFormatterType(v) => v,
                    _ => abort!("expected felid `{}` to resolve to a type which implements trait integra8::formatters::OutputFormatterFactory ", name)
                }
            })
    }

    fn take_structopt_struct(&mut self, name: &'static str) -> Option<StructoptStructValue> {
        self.take(name).map(|attr| match attr {
            ParameterValue::StructoptStruct(v) => v,
            _ => abort!(
                "expected felid `{}` to resolve to a body of a struct, or a type",
                name
            ),
        })
    }

    fn take_string_parameter(&mut self, name: &'static str) -> Option<StringParameterValue> {
        self.take(name).map(|attr| match attr {
            ParameterValue::StringParameter(v) => v,
            _ => abort!("expected felid `{}` to resolve to a string literal", name),
        })
    }

    fn take(&mut self, name: &'static str) -> Option<ParameterValue> {
        self.attrs.remove(name)
    }
}
