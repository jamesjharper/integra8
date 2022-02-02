extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;

mod bookends;
mod exec_fn;
mod parse;
mod suite;
mod test;

#[proc_macro_attribute]
pub fn integration_test(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    test::register_test(input_tokens)
}

#[proc_macro_attribute]
pub fn suite(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    suite::register_suite(input_tokens)
}

#[proc_macro_attribute]
pub fn teardown(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    bookends::register_teardown(input_tokens)
}

#[proc_macro_attribute]
pub fn setup(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    bookends::register_setup(input_tokens)
}
