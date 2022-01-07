use syn::{parse_quote, Result};
use syn::parse::{Parse, ParseStream};
use syn::Expr;
use proc_macro2::TokenStream;

pub struct StringParameterValue  {
    expr: Box<Expr>
}

impl Parse for StringParameterValue {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            expr: input.parse::<Box<Expr>>()?
        })
    }
}

impl StringParameterValue {
    pub fn render_tokens(self) -> TokenStream {
        // TODO: detect if quotations are already in th expr
        let v = self.expr;
        parse_quote!(stringify!(#v))
    }
}
