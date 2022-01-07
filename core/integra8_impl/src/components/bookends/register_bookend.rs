use crate::components::bookends::bookend_attributes::BookendAttributes;
use crate::components::exec_fn::ExecFn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn register_teardown(input_tokens: TokenStream) -> TokenStream {
    let mut decorated_fn = parse_macro_input!(input_tokens as ItemFn);

    let mut test_attr = match BookendAttributes::take_from(&mut decorated_fn.attrs) {
        Ok(test_attr) => test_attr,
        Err(err) => return err,
    };

    let mut teardown_fn = ExecFn::from(decorated_fn);

    let integra8_path = test_attr.take_integra8_path();
    let ignore_expr = test_attr.take_ignore();
    let critical_threshold = test_attr.take_critical_threshold();
    let cascade_failure = test_attr.take_cascade_failure();
    let teardown_method = teardown_fn.take_exec_fn();
    let teardown_name_ident = &teardown_method.sig.ident;

    let tokens = quote! {
        #teardown_method

        // Prevent more then one tear down being defined with in the same mod
        static __ONE_TEAR_DOWN_PER_NAMESPACE: &'static str = "Teardown method can only be defined once per namespace";

        pub(crate) mod #teardown_name_ident {

            use crate::REGISTERED_COMPONENTS;

            #[#integra8_path ::linkme::distributed_slice(REGISTERED_COMPONENTS)]
            #[linkme(crate = #integra8_path ::linkme)]
            static REGISTERER_COMPONENTS: fn() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> = teardown_def;

            pub(crate) fn teardown_def() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> {
                #integra8_path ::decorations::ComponentDecoration::TearDown(
                    #integra8_path ::decorations::BookEndDecoration {
                        desc: #integra8_path ::decorations::BookEndAttributesDecoration {
                           path: module_path!(),
                           location: current_source_location!(),
                           ignore: #ignore_expr,
                           critical_threshold: #critical_threshold,
                           cascade_failure: #cascade_failure
                        },
                        bookend_fn: super::#teardown_name_ident,
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
        Err(err) => return err,
    };

    let mut teardown_fn = ExecFn::from(decorated_fn);

    let integra8_path = test_attr.take_integra8_path();
    let ignore_expr = test_attr.take_ignore();
    let critical_threshold = test_attr.take_critical_threshold();
    let cascade_failure = test_attr.take_cascade_failure();
    let setup_method = teardown_fn.take_exec_fn();
    let setup_name_ident = &setup_method.sig.ident;

    let tokens = quote! {
        #setup_method

        // Prevent more then one setup down being defined with in the same mod
        static __ONE_SETUP_PER_NAMESPACE: &'static str = "Setup method can only be defined once per namespace";

        pub(crate) mod #setup_name_ident {

            use crate::REGISTERED_COMPONENTS;

            #[#integra8_path ::linkme::distributed_slice(REGISTERED_COMPONENTS)]
            #[linkme(crate = #integra8_path ::linkme)]
            static REGISTERER_COMPONENTS: fn() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> = setup_def;

            pub(crate) fn setup_def() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> {
                #integra8_path ::decorations::ComponentDecoration::Setup(
                    #integra8_path ::decorations::BookEndDecoration {
                        desc: #integra8_path ::decorations::BookEndAttributesDecoration {
                           path: module_path!(),
                           location: current_source_location!(),
                           ignore: #ignore_expr,
                           critical_threshold: #critical_threshold,
                           cascade_failure: #cascade_failure
                        },
                        bookend_fn: super::#setup_name_ident,
                    }
                )
            }
        }
    };

    TokenStream::from(tokens)
}
