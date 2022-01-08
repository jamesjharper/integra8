use crate::components::exec_fn::ExecFn;
use crate::components::test::test_attributes::TestAttributes;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn register_test(input_tokens: TokenStream) -> TokenStream {
    let mut decorated_fn = parse_macro_input!(input_tokens as ItemFn);

    let mut test_attr = match TestAttributes::take_from(&mut decorated_fn.attrs) {
        Ok(test_attr) => test_attr,
        Err(err) => return err,
    };

    let mut test_fn = ExecFn::from(decorated_fn);

    let integra8_path = test_attr.take_integra8_path();
    let ignore_test_expr = test_attr.take_ignore_test();
    let allow_fail_expr = test_attr.take_allow_fail();
    let warn_threshold = test_attr.take_warn_threshold();
    let critical_threshold = test_attr.take_critical_threshold();
    let cascade_failure = test_attr.take_cascade_failure();
    let test_method = test_fn.take_exec_fn();
    let delegate_expr = test_fn.take_delegate_expr();

    let test_name_ident = &test_method.sig.ident;

    let tokens = quote! {
        #test_method

        pub mod #test_name_ident {
            use crate::REGISTERED_COMPONENTS;

            #[#integra8_path ::linkme::distributed_slice(REGISTERED_COMPONENTS)]
            #[linkme(crate = #integra8_path ::linkme)]
            static REGISTERER_COMPONENTS: fn() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> = test_def;

            pub fn test_def() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> {
                #integra8_path ::decorations::ComponentDecoration::IntegrationTest(
                    #integra8_path ::decorations::TestDecoration {
                        desc: #integra8_path ::decorations::TestAttributesDecoration {
                           name: stringify!(#test_name_ident), // mod name contains the test name in the path
                           path: module_path!(),
                           location: #integra8_path ::context::src!(),
                           ignore: #ignore_test_expr,
                           allow_fail: #allow_fail_expr,
                           warn_threshold: #warn_threshold,
                           critical_threshold: #critical_threshold,
                           cascade_failure: #cascade_failure,
                           concurrency_mode: None // TODO: add ability to select the concurrency mode
                        },
                        test_fn: #delegate_expr,
                    }
                )
            }
        }
    };

    TokenStream::from(tokens)
}
