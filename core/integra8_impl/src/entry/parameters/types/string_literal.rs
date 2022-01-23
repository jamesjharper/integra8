use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Lit};
use syn::{parse_quote, Result};

pub enum StringParameterValue {
    Expr(Box<Expr>),
    Lit(Box<Lit>)
}

impl Parse for StringParameterValue {
    fn parse(input: ParseStream) -> Result<Self> {
        // Check that the value is encapsulated in a ""
        // Any suggestions if there is a better way to do this?
        if let Some((token, _)) = input.cursor().literal() {
            if  token.to_string().starts_with("\"") {
                return Ok(Self::Lit(input.parse::<Box<Lit>>()?))
            }
        }

        Ok(Self::Expr(input.parse::<Box<Expr>>()?))
    }
}

impl StringParameterValue {
    pub fn render_tokens(self) -> TokenStream {
        match self {
            Self::Expr(expr) =>  parse_quote!(stringify!(#expr)),
            Self::Lit(lit) => {
                match *lit {
                    Lit::Str(lit_str) => parse_quote!(#lit_str),
                    other_lit => parse_quote!(stringify!(#other_lit))
                }
            }
        }
    }
}
