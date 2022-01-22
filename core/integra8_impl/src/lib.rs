extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod entry;

#[proc_macro_error]
#[proc_macro]
pub fn main_test(item: TokenStream) -> TokenStream {
    entry::main_test(item)
}
