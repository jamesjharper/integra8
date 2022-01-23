use std::mem;
use std::time::Duration;

use syn::parse::Error;
use syn::{parse_quote, Attribute, Expr, Path, Lit, Result};
use crate::parse;

pub struct TestAttributes {
    integra8_path: Option<Path>,
    name: Option<Lit>,
    description: Option<Lit>,
    ignore: Option<bool>,
    allow_fail: Option<bool>,
    warn_time_limit: Option<Duration>,
    time_limit: Option<Duration>,
    parallel_enabled: Option<bool>,
}

impl TestAttributes {
    pub fn take_from(attrs: &mut Vec<Attribute>) -> Result<Self> {
        let mut builder = Self {
            integra8_path: None,
            name: None,
            description: None,
            ignore: None,
            allow_fail: None,
            warn_time_limit: None,
            time_limit: None,
            parallel_enabled: None,
        };


        for attr in attrs.drain(..) {

            // #[integra8(crate = path::to::integra8)]
            if let Some(path) = parse::try_parse_integra8_path(&attr)?  {
                builder.integra8_path = Some(path);
                continue;
            }

            // #[name = "the test's given name"]
            if let Some(name) = parse::try_parse_lit(&attr, "name")?  {
                builder.name = Some(name);
                continue;
            }

            // #[description = "the description of this test"]
            if let Some(description) = parse::try_parse_lit(&attr, "description")?  {
                builder.description = Some(description);
                continue;
            }

            // #[warning_time_limit = "1m")]
            if let Some(duration) = parse::try_parse_duration(&attr, "warning_time_limit")?  {
                builder.warn_time_limit = Some(duration);
                continue;
            }

            // #[time_limit = "1m 30s"]
            if let Some(duration) = parse::try_parse_duration(&attr, "time_limit")?  {
                builder.time_limit = Some(duration);
                continue;
            }

            // #[parallel]
            if let Some(flag) = parse::try_parse_flag(&attr, "parallel")?  {
                builder.parallel_enabled = Some(flag);
                continue;
            }
    
            // #[sequential]
            if let Some(flag) = parse::try_parse_flag(&attr, "sequential")?  {
                builder.parallel_enabled = Some(!flag);
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

            return Err(Error::new_spanned(attr, "unexpected attribute"));
        }

        Ok(builder)
    }

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

    pub fn take_warn_time_limit(&mut self) ->  Expr {
        match mem::take(&mut self.warn_time_limit) {
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

    pub fn take_time_limit(&mut self) -> Expr {
        match mem::take(&mut self.time_limit) {
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

    pub fn take_concurrency_mode(&mut self, integra8_path: &Path) -> Expr {
        match mem::take(&mut self.parallel_enabled) {
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
