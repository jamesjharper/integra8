use proc_macro2::TokenStream;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::{Expr, Field, Path, Token};

pub struct StructoptStruct {
    pub structopt_type: Option<Box<Expr>>,
    pub definition: Option<TokenStream>,
}

pub enum StructoptStructValue {
    External {
        type_name: Box<Expr>,
    },
    Inline {
        type_name: Box<Expr>,
        fields: Punctuated<Field, Token![,]>,
    },
    Unit {
        type_name: Box<Expr>,
    },
}

impl StructoptStructValue {
    pub fn render_tokens(self) -> StructoptStruct {
        // TODO: Move to common location?
        let structopt_path: Path = parse_quote!(::integra8::structopt);

        match self {
            Self::External { type_name } => {
                // Detect user define external types
                // settings: <USER_DEFINED_TYPE>
                StructoptStruct {
                    structopt_type: Some(type_name),
                    definition: None,
                }
            }
            Self::Inline { type_name, fields } => {
                StructoptStruct {
                    // user has defined an inline type
                    definition: Some(parse_quote!(
                        #[derive(Clone, Debug, #structopt_path ::StructOpt)]
                        #[structopt()]
                        pub struct #type_name {
                            #fields
                        }
                    )),
                    structopt_type: Some(type_name),
                }
            }
            Self::Unit { type_name } => {
                StructoptStruct {
                    // user has defined an inline type
                    definition: Some(parse_quote!(

                        #[derive(Clone, Debug)]
                        pub struct #type_name;

                        impl #structopt_path ::StructOptInternal for #type_name {
                            fn augment_clap<'a, 'b>(
                                app: #structopt_path ::clap::App<'a, 'b>,
                            ) -> #structopt_path ::clap::App<'a, 'b> {
                                app.version(env!("CARGO_PKG_VERSION"))
                            }

                            fn is_subcommand() -> bool {
                                false
                            }
                        }

                        impl #structopt_path ::StructOpt for #type_name {
                            fn clap<'a, 'b>() -> #structopt_path ::clap::App<'a, 'b> {
                                let app = #structopt_path ::clap::App::new(env!("CARGO_PKG_NAME"));
                                <Self as #structopt_path ::StructOptInternal>::augment_clap(app)
                            }
                            fn from_clap(matches: & #structopt_path ::clap::ArgMatches) -> Self {
                                #type_name {}
                            }
                        }
                    )),
                    structopt_type: Some(type_name),
                }
            }
        }
    }
}
