extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;

mod entry;

#[proc_macro]
pub fn main_test(item: TokenStream) -> TokenStream {
    entry::main_test(item)
}
