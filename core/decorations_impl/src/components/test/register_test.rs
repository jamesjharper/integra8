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

    // Attributes
    let integra8_path = test_attr.take_integra8_path();
    let ignore_test_expr = test_attr.take_ignore_test();
    let name_expr = test_attr.take_name();
    let description_expr = test_attr.take_description();
    let allow_fail_expr = test_attr.take_allow_fail();
    let warn_threshold_expr = test_attr.take_warn_threshold();
    let critical_threshold_expr = test_attr.take_critical_threshold();
    let concurrency_mode_expr = test_attr.take_concurrency_mode();

    // Fn
    let mut test_fn = ExecFn::from(decorated_fn);
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
                           name: #name_expr,
                           path: module_path!(),
                           description: #description_expr,
                           location: Some(#integra8_path ::components::src_loc!()),
                           ignore: #ignore_test_expr,
                           allow_fail: #allow_fail_expr,
                           warn_threshold: #warn_threshold_expr,
                           critical_threshold: #critical_threshold_expr,
                           concurrency_mode: #concurrency_mode_expr,
                        },
                        test_fn: #delegate_expr,
                    }
                )
            }
        }
    };

    TokenStream::from(tokens)
}
