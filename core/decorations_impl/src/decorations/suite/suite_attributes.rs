use std::mem;
use std::time::Duration;

use syn::parse::Error;
use syn::{parse_quote, Attribute, Expr, Path, Lit, Result};

use crate::decorations::parse;

pub struct SuiteAttributes {
    integra8_path: Option<Path>,
    name: Option<Lit>,
    description: Option<Lit>,
    ignore: Option<bool>,
    allow_fail: Option<bool>,
    test_warning_time_limit: Option<Duration>,
    test_time_limit: Option<Duration>,
    setup_time_limit: Option<Duration>,
    tear_down_time_limit: Option<Duration>,
    suite_parallel_enabled: Option<bool>,
    test_parallel_enabled: Option<bool>,
}

impl SuiteAttributes {
    pub fn take_from(attrs: &mut Vec<Attribute>) -> Result<Self> {
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
        };



        for attr in attrs.drain(..) {

            // #[integra8(crate = path::to::integra8)]
            if let Some(path) = parse::try_parse_integra8_path(&attr)?  {
                builder.integra8_path = Some(path);
                continue;
            }

            // #[name = "the suite's given name"]
            if let Some(name) = parse::try_parse_lit(&attr, "name")?  {
                builder.name = Some(name);
                continue;
            }

            // #[description = "the description of this suite"]
            if let Some(description) = parse::try_parse_lit(&attr, "description")?  {
                builder.description = Some(description);
                continue;
            }

             // #[parallel]
             if let Some(flag) = parse::try_parse_flag(&attr, "parallel")?  {
                builder.suite_parallel_enabled = Some(flag);
                continue;
            }
    
            // #[sequential]
            if let Some(flag) = parse::try_parse_flag(&attr, "sequential")?  {
                builder.suite_parallel_enabled = Some(!flag);
                continue;
            }


            // #[test_parallel]
            if let Some(flag) = parse::try_parse_flag(&attr, "test_parallel")?  {
                builder.test_parallel_enabled = Some(flag);
                continue;
            }
    
            // #[test_sequential]
            if let Some(flag) = parse::try_parse_flag(&attr, "test_sequential")?  {
                builder.test_parallel_enabled = Some(!flag);
                continue;
            }

            // #[ignore]
            if let Some(flag) = parse::try_parse_flag(&attr, "ignore")?  {
                builder.ignore = Some(flag);
                continue;
            }

            // #[allow_fail]
            if let Some(flag) = parse::try_parse_flag(&attr, "allow_fail")?  {
                builder.allow_fail = Some(flag);
                continue;
            }

            // #[test_warning_time_limit = "1m")]
            if let Some(duration) = parse::try_parse_duration(&attr, "test_warning_time_limit")?  {
                builder.test_warning_time_limit = Some(duration);
                continue;
            }

            // #[test_time_limit = "1m 30s"]
            if let Some(duration) = parse::try_parse_duration(&attr, "test_time_limit")?  {
                builder.test_time_limit = Some(duration);
                continue;
            }

            // #[setup_time_limit = "1m 30s"]
            if let Some(duration) = parse::try_parse_duration(&attr, "setup_time_limit")?  {
                builder.setup_time_limit = Some(duration);
                continue;
            }

            // #[tear_down_time_limit = "1m 30s"]
            if let Some(duration) = parse::try_parse_duration(&attr, "tear_down_time_limit")?  {
                builder.tear_down_time_limit = Some(duration);
                continue;
            }

            return Err(Error::new_spanned(attr, "unexpected attribute"));
        }


        Ok(builder)
    }

    // Try Parse Attributes



    // Take values

    pub fn take_integra8_path(&mut self) -> Path {
        mem::take(&mut self.integra8_path).unwrap_or_else(|| parse_quote!(::integra8))
    }

    pub fn take_name(&mut self) -> Expr {
        mem::take(&mut self.name)
            .map(|val| {
                parse_quote!(Some(#val))
            })
            .unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_description(&mut self) -> Expr {
        mem::take(&mut self.description)
            .map(|val| {
                parse_quote!(Some(#val))
            })
            .unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_ignore(&mut self) -> Expr {
        mem::take(&mut self.ignore)
            .map(|val| {
                parse_quote!(Some(#val))
            })
            .unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_allow_fail(&mut self) -> Expr {
        mem::take(&mut self.allow_fail)
            .map(|val| {
                parse_quote!(Some(#val))
            })
            .unwrap_or_else(|| parse_quote!(None))
    }

    pub fn take_test_warning_time_limit(&mut self) -> Expr {
        match mem::take(&mut self.test_warning_time_limit) {
            Some(duration) => {
                let secs = duration.as_secs();    
                let subsec_nanos = duration.subsec_nanos();
                parse_quote!(Some(std::time::Duration::new(#secs, #subsec_nanos)))
            }
            None => {
                parse_quote!(None)
            }
        }
    }

    pub fn take_test_time_limit(&mut self) -> Expr {
        match mem::take(&mut self.test_time_limit) {
            Some(duration) => {
                let secs = duration.as_secs();    
                let subsec_nanos = duration.subsec_nanos();
                parse_quote!(Some(std::time::Duration::new(#secs, #subsec_nanos)))
            }
            None => {
                parse_quote!(None)
            }
        }
    }

    pub fn take_setup_time_limit(&mut self) -> Expr {
        match mem::take(&mut self.setup_time_limit) {
            Some(duration) => {
                let secs = duration.as_secs();    
                let subsec_nanos = duration.subsec_nanos();
                parse_quote!(Some(std::time::Duration::new(#secs, #subsec_nanos)))
            }
            None => {
                parse_quote!(None)
            }
        }
    }

    pub fn take_tear_down_time_limit(&mut self) -> Expr {
        match mem::take(&mut self.tear_down_time_limit) {
            Some(duration) => {
                let secs = duration.as_secs();    
                let subsec_nanos = duration.subsec_nanos();
                parse_quote!(Some(std::time::Duration::new(#secs, #subsec_nanos)))
            }
            None => {
                parse_quote!(None)
            }
        }
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
}
