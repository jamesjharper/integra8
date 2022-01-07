use syn::parse::{Error, ParseStream, Result};
use syn::{parse_quote, Attribute, Path, Token, Expr};
use std::mem;

use proc_macro::TokenStream;

pub struct SuiteAttributes {
    pub integra8_path: Option<Path>,
    pub ignore: Option<Expr>,
    pub allow_fail: Option<Expr>,
    pub warn_threshold: Option<Expr>,
    pub critical_threshold: Option<Expr>,
    pub cascade_failure: Option<Expr>,
    pub errors: Option<Error>
}

impl SuiteAttributes {
    pub fn take_from(attrs: &mut Vec<Attribute>) -> std::result::Result<Self, TokenStream> {
        let mut builder = Self {
            integra8_path: None,
            ignore: None,
            allow_fail: None,
            warn_threshold: None,
            critical_threshold: None,
            cascade_failure: None,
            errors: None,
        };

        attrs.retain(|attr| {
            !(
                // Keep looking until we find a match
                builder.try_parse_integra8_path(attr) ||
                builder.try_parse_allow_fail_expr(attr) ||
                builder.try_parse_ignore_expr(attr) ||
                builder.try_parse_warn_threshold_expr(attr) ||
                builder.try_parse_critical_threshold_expr(attr) ||
                builder.try_parse_cascade_failure_expr(attr)
            )
        });


        match builder.take_errors() {
            Ok(_) =>  Ok(builder),
            Err(err) => return Err(TokenStream::from(err.to_compile_error())),
        }
    }

    // Try Parse Attributes

    // looking for #[integra8(crate = path::to::integra8)]
    fn try_parse_integra8_path(&mut self, attr: &Attribute) -> bool {
        if !attr.path.is_ident("integra8") {
            return false;
        }

        let result = attr.parse_args_with(|input: ParseStream| {
            input.parse::<Token![crate]>()?;
            input.parse::<Token![=]>()?;
            input.call(Path::parse_mod_style)
        });

        self.integra8_path = self.some_or_accumulate_error(result);
        true
    }

    // looking for 
    // #[warn_threshold_seconds(1)]
    // #[warn_threshold_milliseconds(1000)]
    fn try_parse_warn_threshold_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("warn_threshold_seconds") {
            self.warn_threshold = self.parse_duration_from_sec(attr);
            return true;
        }
        if attr.path.is_ident("warn_threshold_milliseconds") {
            self.warn_threshold = self.parse_duration_from_millis(attr);
            return true;
        }
        return false;
    }

    // looking for 
    // #[critical_threshold_seconds(1)]
    // #[critical_threshold_milliseconds(1000)]
    fn try_parse_critical_threshold_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("critical_threshold_seconds") {
            self.critical_threshold = self.parse_duration_from_sec(attr);
            return true;
        }
        if attr.path.is_ident("critical_threshold_milliseconds") {
            self.critical_threshold = self.parse_duration_from_millis(attr);
            return true;
        }
        return false;
    }

    // cascade failure 
    // looking for #[cascade_failure]
    // looking for #[do_not_cascade_failure]
    fn try_parse_cascade_failure_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("cascade_failure") {
            self.cascade_failure = Some(parse_quote!(Some(true)));
            return true;
        }

        if attr.path.is_ident("do_not_cascade_failure") {
            self.cascade_failure = Some(parse_quote!(Some(false)));
            return true;
        }

        return false;
    }

    // looking for #[ignore()]
    fn try_parse_ignore_expr(&mut self, attr: &Attribute) -> bool {
        if !attr.path.is_ident("ignore") {
            return false;
        }

        self.ignore = Some(parse_quote!(Some(true)));
        true
    }

    // looking for #[allow_fail()]
    fn try_parse_allow_fail_expr(&mut self, attr: &Attribute) -> bool {
        if !attr.path.is_ident("allow_fail") {
            return false;
        }

        self.allow_fail = Some(parse_quote!(Some(true)));
        true
    }

    fn parse_duration_from_sec(&mut self, attr: &Attribute)-> Option<Expr> {
        let result = attr.parse_args_with(|input: ParseStream| {
            input.call(Expr::parse_without_eager_brace)
        });

        self.some_or_accumulate_error(result)
            .map(|exp| parse_quote!(Some(::std::time::Duration::from_secs(#exp))))
    }

    fn parse_duration_from_millis(&mut self, attr: &Attribute) -> Option<Expr> {
        let result = attr.parse_args_with(|input: ParseStream| {
            input.call(Expr::parse_without_eager_brace)
        });

        self.some_or_accumulate_error(result)
            .map(|exp| parse_quote!(Some(::std::time::Duration::from_millis(#exp))))
    }

    // Take values

    pub fn take_integra8_path(&mut self) -> Path {
        mem::take(&mut self.integra8_path)
            .unwrap_or_else(|| parse_quote!(::integra8))
    }

    pub fn take_ignore(&mut self) -> Expr {
        mem::take(&mut self.ignore)
            .unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_allow_fail(&mut self) -> Expr {
        mem::take(&mut self.allow_fail)
            .unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_warn_threshold(&mut self) -> Expr {
        mem::take(&mut self.warn_threshold)
            .unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_critical_threshold(&mut self) -> Expr {
        mem::take(&mut self.critical_threshold)
            .unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_cascade_failure(&mut self) -> Expr {
        mem::take(&mut self.cascade_failure)
            .unwrap_or_else(|| parse_quote!(None))
    }

    fn some_or_accumulate_error<T>(&mut self, result: Result<T>) -> Option<T> {
        match result{
            Ok(t) => Some(t),
            Err(err) => {
                match &mut self.errors {
                    None => self.errors = Some(err),
                    Some(ref mut errors) => errors.combine(err),
                };
                None
            }
        } 
    }

    fn take_errors(&mut self) -> Result<()> {
        mem::take(&mut self.errors)
            .map(|err| Err(err))
            .unwrap_or_else(|| Ok(()))
    }   
}
