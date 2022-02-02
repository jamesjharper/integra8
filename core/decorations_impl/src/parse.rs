use std::time::Duration;
use syn::parse::{Error, ParseStream};
use syn::{Attribute, Lit, Meta, MetaNameValue, Path, Result, Token};

// looking for #[integra8(crate = path::to::integra8)]
pub fn try_parse_integra8_path(attr: &Attribute) -> Result<Option<Path>> {
    if !attr.path.is_ident("integra8") {
        return Ok(None);
    }

    Ok(Some(attr.parse_args_with(|input: ParseStream| {
        input.parse::<Token![crate]>()?;
        input.parse::<Token![=]>()?;
        input.call(Path::parse_mod_style)
    })?))
}

// looking for
// #[{attr_name} = "1m")]
pub fn try_parse_duration(attr: &Attribute, attr_name: &'static str) -> Result<Option<Duration>> {
    if !attr.path.is_ident(attr_name) {
        return Ok(None);
    }

    match attr.parse_meta()? {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(lit_str),
            ..
        }) => {
            let duration = humantime::parse_duration(&lit_str.value()).map_err(|e| {
                Error::new_spanned(
                    attr,
                    format!(
                        "Unable to parse duration string \"{}\",  {:?}",
                        lit_str.value(),
                        e.to_string()
                    ),
                )
            })?;
            Ok(Some(duration))
        }
        _ => Err(Error::new_spanned(
            attr,
            format!("expected #[{} = \"...\"]", attr_name),
        )),
    }
}

// looking for
// #[{description} = "...")]
pub fn try_parse_lit(attr: &Attribute, attr_name: &'static str) -> Result<Option<Lit>> {
    if !attr.path.is_ident(attr_name) {
        return Ok(None);
    }

    match attr.parse_meta().unwrap() {
        Meta::NameValue(MetaNameValue { lit, .. }) => Ok(Some(lit)),
        _ => Err(Error::new_spanned(
            attr,
            format!("expected #[{} = \"...\"]", attr_name),
        )),
    }
}

pub fn try_parse_flag(attr: &Attribute, attr_name: &'static str) -> Result<Option<bool>> {
    match attr.path.is_ident(attr_name) {
        true => Ok(Some(true)),
        false => Ok(None),
    }
}
