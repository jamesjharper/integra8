
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;

mod components;
mod entry;

#[proc_macro]
pub fn main_test(item: TokenStream) -> TokenStream {
   entry::main_test(item)
}

#[proc_macro_attribute]
pub fn integration_test(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    components::test::register_test(input_tokens)
}

#[proc_macro_attribute]
pub fn integration_suite(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    components::suite::register_suite(input_tokens)
}

#[proc_macro_attribute]
pub fn teardown(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    components::bookends::register_teardown(input_tokens)
}

#[proc_macro_attribute]
pub fn setup(_args_tokens: TokenStream, input_tokens: TokenStream) -> TokenStream {
    components::bookends::register_setup(input_tokens)
}
