use proc_macro2::TokenStream;
use syn::parse_quote;

#[allow(dead_code)]
pub enum MainDefinitionValue {
    AsyncStd,
    Tokio,
    Custom(TokenStream),
    Sync,
}

impl MainDefinitionValue {
    #[cfg(feature = "async-std-runtime")]
    pub fn configured_runtime() -> Self {
        MainDefinitionValue::AsyncStd
    }

    #[cfg(feature = "tokio-runtime")]
    pub fn configured_runtime() -> Self {
        MainDefinitionValue::Tokio
    }

    #[cfg(not(any(feature = "tokio-runtime", feature = "async-std-runtime")))]
    pub fn configured_runtime() -> Self {
        MainDefinitionValue::Sync
    }

    pub fn render_tokens(self) -> TokenStream {
        match self {
            Self::Custom(token_stream) => token_stream,
            Self::AsyncStd => {
                parse_quote!(
                    fn main() {
                        // To get a clean exit without any additional logging
                        // we set std::process::exit(exit_code); rather then
                        // returning an error
                        let exit_code = integra8::async_runtime::block_on(async {
                            run_tests!(Parameters::from_command_line());
                        });
                        std::process::exit(exit_code);
                    }
                )
            }
            Self::Tokio => {
                parse_quote!(
                    fn main() {
                        let mut rt = integra8::async_runtime::Runtime::new().unwrap();
                        // To get a clean exit without any additional logging
                        // we set std::process::exit(exit_code); rather then
                        // returning an error
                        let exit_code =
                            rt.block_on(async { run_tests!(Parameters::from_command_line()) });
                        std::process::exit(exit_code);
                    }
                )
            }
            Self::Sync => {
                parse_quote!(
                    fn main() {
                        let exit_code = run_tests!(Parameters::from_command_line());
                        std::process::exit(exit_code);
                    }
                )
            }
        }
    }
}
