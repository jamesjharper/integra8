use std::mem;
use syn::parse::{Error, ParseStream, Result};
use syn::{parse_quote, Attribute, Expr, Path, Token};

use proc_macro::TokenStream;

pub struct SuiteAttributes {
    pub integra8_path: Option<Path>,
    pub name: Option<Expr>,
    pub description: Option<Expr>,
    pub ignore: Option<Expr>,
    pub allow_fail: Option<Expr>,
    pub test_warning_time_limit: Option<Expr>,
    pub test_time_limit: Option<Expr>,
    pub setup_time_limit: Option<Expr>,
    pub tear_down_time_limit: Option<Expr>,

    pub suite_parallel_enabled: Option<bool>,
    pub test_parallel_enabled: Option<bool>,
    pub errors: Option<Error>,
}

impl SuiteAttributes {
    pub fn take_from(attrs: &mut Vec<Attribute>) -> std::result::Result<Self, TokenStream> {
        let mut builder = Self {
            integra8_path: None,
            name: None,
            description: None,
            ignore: None,
            allow_fail: None,
            test_warning_time_limit: None,
            test_time_limit: None,
            setup_time_limit: None,
            tear_down_time_limit: None,
            suite_parallel_enabled: None,
            test_parallel_enabled: None,
            errors: None,
        };

        attrs.retain(|attr| {
            !(
                // Keep looking until we find a match
                builder.try_parse_integra8_path(attr)
                    || builder.try_parse_name_expr(attr)
                    || builder.try_parse_description_expr(attr)
                    || builder.try_parse_allow_fail_expr(attr)
                    || builder.try_parse_concurrency_mode_expr(attr)
                    || builder.try_parse_test_concurrency_mode_expr(attr)
                    || builder.try_parse_ignore_expr(attr)
                    || builder.try_parse_warn_test_time_limit_expr(attr)
                    || builder.try_parse_critical_threshold_expr(attr)
                    || builder.try_parse_setup_critical_time_limit_expr(attr)
                    || builder.try_parse_tear_down_time_limit_expr(attr)
            )
        });

        match builder.take_errors() {
            Ok(_) => Ok(builder),
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
    // #[name("the suites given name")]
    fn try_parse_name_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("name") {
            self.name = self.parse_string(attr);
            return true;
        }

        return false;
    }

    // looking for
    // #[description("the description of this suite")]
    fn try_parse_description_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("description") {
            self.description = self.parse_string(attr);
            return true;
        }

        return false;
    }

    // looking for
    // #[test_warning_time_threshold_seconds(1)]
    // #[test_warning_time_limit_milliseconds(1000)]
    fn try_parse_warn_test_time_limit_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("test_warning_time_threshold_seconds") {
            self.test_warning_time_limit = self.parse_duration_from_sec(attr);
            return true;
        }
        if attr.path.is_ident("test_warning_time_limit_milliseconds") {
            self.test_warning_time_limit = self.parse_duration_from_millis(attr);
            return true;
        }
        return false;
    }

    // looking for
    // #[test_time_limit_seconds(1)]
    // #[test_time_limit_milliseconds(1000)]
    fn try_parse_critical_threshold_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("test_time_limit_seconds") {
            self.test_time_limit = self.parse_duration_from_sec(attr);
            return true;
        }
        if attr.path.is_ident("test_time_limit_milliseconds") {
            self.test_time_limit = self.parse_duration_from_millis(attr);
            return true;
        }
        return false;
    }

    // looking for
    // #[setup_time_limit_seconds(1)]
    // #[setup_time_limit_milliseconds(1000)]
    fn try_parse_setup_critical_time_limit_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("setup_time_limit_seconds") {
            self.setup_time_limit = self.parse_duration_from_sec(attr);
            return true;
        }
        if attr.path.is_ident("setup_time_limit_milliseconds") {
            self.setup_time_limit = self.parse_duration_from_millis(attr);
            return true;
        }
        return false;
    }

    // looking for
    // #[tear_down_time_limit_seconds(1)]
    // #[tear_down_time_limit_milliseconds(1000)]
    fn try_parse_tear_down_time_limit_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("tear_down_time_limit_seconds") {
            self.tear_down_time_limit = self.parse_duration_from_sec(attr);
            return true;
        }
        if attr
            .path
            .is_ident("tear_down_time_limit_milliseconds")
        {
            self.tear_down_time_limit = self.parse_duration_from_millis(attr);
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

    // Suite concurrency mode
    // looking for #[parallel]
    // looking for #[sequential]
    fn try_parse_concurrency_mode_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("parallel") {
            self.suite_parallel_enabled = Some(true);
            return true;
        }

        if attr.path.is_ident("sequential") {
            self.suite_parallel_enabled = Some(false);
            return true;
        }

        return false;
    }

    // Suite test concurrency mode
    // looking for #[parallelize_test]
    // looking for #[sequence_tests]
    fn try_parse_test_concurrency_mode_expr(&mut self, attr: &Attribute) -> bool {
        if attr.path.is_ident("parallel_tests") {
            self.test_parallel_enabled = Some(true);
            return true;
        }

        if attr.path.is_ident("sequential_tests") {
            self.suite_parallel_enabled = Some(false);
            return true;
        }

        return false;
    }

    fn parse_duration_from_sec(&mut self, attr: &Attribute) -> Option<Expr> {
        let result =
            attr.parse_args_with(|input: ParseStream| input.call(Expr::parse_without_eager_brace));

        self.some_or_accumulate_error(result)
            .map(|exp| parse_quote!(Some(::std::time::Duration::from_secs(#exp))))
    }

    fn parse_duration_from_millis(&mut self, attr: &Attribute) -> Option<Expr> {
        let result =
            attr.parse_args_with(|input: ParseStream| input.call(Expr::parse_without_eager_brace));

        self.some_or_accumulate_error(result)
            .map(|exp| parse_quote!(Some(::std::time::Duration::from_millis(#exp))))
    }

    fn parse_string(&mut self, attr: &Attribute) -> Option<Expr> {
        let result =
            attr.parse_args_with(|input: ParseStream| input.call(Expr::parse_without_eager_brace));

        self.some_or_accumulate_error(result)
            .map(|exp| parse_quote!(Some(#exp)))
    }

    // Take values

    pub fn take_integra8_path(&mut self) -> Path {
        mem::take(&mut self.integra8_path).unwrap_or_else(|| parse_quote!(::integra8))
    }

    pub fn take_name(&mut self) -> Expr {
        mem::take(&mut self.name).unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_description(&mut self) -> Expr {
        mem::take(&mut self.description).unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_ignore(&mut self) -> Expr {
        mem::take(&mut self.ignore).unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_allow_fail(&mut self) -> Expr {
        mem::take(&mut self.allow_fail).unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_test_warning_time_limit(&mut self) -> Expr {
        mem::take(&mut self.test_warning_time_limit).unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_test_time_limit(&mut self) -> Expr {
        mem::take(&mut self.test_time_limit).unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_setup_time_limit(&mut self) -> Expr {
        mem::take(&mut self.setup_time_limit).unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_tear_down_time_limit(&mut self) -> Expr {
        mem::take(&mut self.tear_down_time_limit).unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_suite_concurrency_mode(&mut self, integra8_path: &Path) -> Expr {
        match mem::take(&mut self.suite_parallel_enabled) {
            Some(true) => {
                parse_quote!(Some(#integra8_path ::components::ConcurrencyMode::Parallel))
            }
            Some(false) => {
                parse_quote!(Some(#integra8_path ::components::ConcurrencyMode::Sequential))
            }
            None => {
                parse_quote!(None)
            }
        }
    }

    pub fn take_test_concurrency_mode(&mut self, integra8_path: &Path) -> Expr {
        match mem::take(&mut self.test_parallel_enabled) {
            Some(true) => {
                parse_quote!(Some(#integra8_path ::components::ConcurrencyMode::Parallel))
            }
            Some(false) => {
                parse_quote!(Some(#integra8_path ::components::ConcurrencyMode::Sequential))
            }
            None => {
                parse_quote!(None)
            }
        }
    }

    fn some_or_accumulate_error<T>(&mut self, result: Result<T>) -> Option<T> {
        match result {
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
