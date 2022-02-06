use crate::bookends::bookend_attributes::BookendAttributes;
use crate::exec_fn::ExecFn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn register_teardown(input_tokens: TokenStream) -> TokenStream {
    let mut decorated_fn = parse_macro_input!(input_tokens as ItemFn);

    let mut test_attr = match BookendAttributes::take_from(&mut decorated_fn.attrs) {
        Ok(test_attr) => test_attr,
        Err(err) => return syn::Error::into_compile_error(err).into(),
    };

    // Attributes
    let integra8_path = test_attr.take_integra8_path();
    let name_expr = test_attr.take_name();
    let description_expr = test_attr.take_description();
    let ignore_expr = test_attr.take_ignore();
    let time_limit_expr = test_attr.take_time_limit();
    let concurrency_mode_expr = test_attr.take_concurrency_mode(&integra8_path);

    // Fn
    let mut teardown_fn = ExecFn::from(decorated_fn, &integra8_path);
    let teardown_method = teardown_fn.take_exec_fn();
    let delegate_expr = teardown_fn.take_delegate_expr();

    let teardown_name_ident = &teardown_method.sig.ident;

    let tokens = quote! {
        #teardown_method

        pub(crate) mod #teardown_name_ident {

            use crate::REGISTERED_COMPONENTS;

            #[#integra8_path ::linkme::distributed_slice(REGISTERED_COMPONENTS)]
            #[linkme(crate = #integra8_path ::linkme)]
            static REGISTERER_COMPONENTS: fn() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> = teardown_def;

            pub(crate) fn teardown_def() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> {
                #integra8_path ::decorations::ComponentDecoration::TearDown(
                    #integra8_path ::decorations::BookEndDecoration {
                        desc: #integra8_path ::decorations::BookEndAttributesDecoration {
                            name: #name_expr,
                            description: #description_expr,
                            path: module_path!(),
                            location: #integra8_path ::components::src_loc!(),
                            ignore: #ignore_expr,
                            time_limit: #time_limit_expr,
                            concurrency_mode: #concurrency_mode_expr,
                        },
                        bookend_fn: #delegate_expr,
                    }
                )
            }
        }
    };

    TokenStream::from(tokens)
}

pub fn register_setup(input_tokens: TokenStream) -> TokenStream {
    let mut decorated_fn = parse_macro_input!(input_tokens as ItemFn);

    let mut test_attr = match BookendAttributes::take_from(&mut decorated_fn.attrs) {
        Ok(test_attr) => test_attr,
        Err(err) => return syn::Error::into_compile_error(err).into(),
    };

    // Attributes
    let integra8_path = test_attr.take_integra8_path();
    let name_expr = test_attr.take_name();
    let description_expr = test_attr.take_description();
    let ignore_expr = test_attr.take_ignore();
    let time_limit_expr = test_attr.take_time_limit();
    let concurrency_mode_expr = test_attr.take_concurrency_mode(&integra8_path);

    // Fn
    let mut setup_fn = ExecFn::from(decorated_fn, &integra8_path);
    let setup_method = setup_fn.take_exec_fn();
    let delegate_expr = setup_fn.take_delegate_expr();

    let setup_name_ident = &setup_method.sig.ident;

    let tokens = quote! {
        #setup_method

        pub(crate) mod #setup_name_ident {

            use crate::REGISTERED_COMPONENTS;

            #[#integra8_path ::linkme::distributed_slice(REGISTERED_COMPONENTS)]
            #[linkme(crate = #integra8_path ::linkme)]
            static REGISTERER_COMPONENTS: fn() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> = setup_def;

            pub(crate) fn setup_def() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> {
                #integra8_path ::decorations::ComponentDecoration::Setup(
                    #integra8_path ::decorations::BookEndDecoration {
                        desc: #integra8_path ::decorations::BookEndAttributesDecoration {
                            name: #name_expr,
                            description: #description_expr,
                            path: module_path!(),
                            location: #integra8_path ::components::src_loc!(),
                            ignore: #ignore_expr,
                            time_limit: #time_limit_expr,
                            concurrency_mode: #concurrency_mode_expr,
                        },
                        bookend_fn: #delegate_expr,
                    }
                )
            }
        }
    };

    TokenStream::from(tokens)
}
