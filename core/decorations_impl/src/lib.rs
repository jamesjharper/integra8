extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;

mod decorations;

#[proc_macro_attribute]
pub fn integration_test(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    decorations::test::register_test(input_tokens)
}

#[proc_macro_attribute]
pub fn integration_suite(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    decorations::suite::register_suite(input_tokens)
}

#[proc_macro_attribute]
pub fn teardown(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    decorations::bookends::register_teardown(input_tokens)
}

#[proc_macro_attribute]
pub fn setup(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    decorations::bookends::register_setup(input_tokens)
}
