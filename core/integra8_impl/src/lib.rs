#[cfg(all(feature = "async-std-runtime", feature = "tokio-runtime"))]
compile_error!(
"feature \"tokio-runtime\" and feature \"async-std-runtime\" cannot be enabled at the same time!
To configure `tokio` use `integra8 = {version = \"{VERSION}\", features = [\"core\", \"tokio-runtime\"], default-features = false }`
To configure `async-std` use `integra8 = {version = \"{VERSION}\", features = [\"core\", \"async-std-runtime\"], default-features = false }`
Otherwise using `integra8 = {version = \"{VERSION}\" }` will enable tokio by default"
);

#[cfg(all(not(feature = "async-std-runtime"), not(feature = "tokio-runtime")))]
compile_error!(
"No async runtime configured!
To configure `tokio` use `integra8 = {version = \"{VERSION}\", features = [\"core\", \"tokio-runtime\"], default-features = false }`
To configure `async-std` use `integra8 = {version = \"{VERSION}\", features = [\"core\", \"async-std-runtime\"], default-features = false }`
Otherwise using `integra8 = {version = \"{VERSION}\" }` will enable tokio by default "
);

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
