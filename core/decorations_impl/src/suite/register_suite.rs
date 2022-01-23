use crate::suite::suite_attributes::SuiteAttributes;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

pub fn register_suite(input_tokens: TokenStream) -> TokenStream {
    let mut decorated_mod = parse_macro_input!(input_tokens as ItemMod);

    let mut test_attr = match SuiteAttributes::take_from(&mut decorated_mod.attrs) {
        Ok(test_attr) => test_attr,
        Err(err) => return syn::Error::into_compile_error(err).into(),
    };

    let integra8_path = test_attr.take_integra8_path();
    let ignore_expr = test_attr.take_ignore();
    let name_expr = test_attr.take_name();
    let description_expr = test_attr.take_description();
    let allow_fail_expr = test_attr.take_allow_fail();
    let test_warning_time_limit_expr = test_attr.take_test_warning_time_limit();
    let test_time_limit_expr = test_attr.take_test_time_limit();
    let setup_time_limit_expr = test_attr.take_setup_time_limit();
    let tear_down_time_limit_expr = test_attr.take_tear_down_time_limit();
    let suite_concurrency_mode_expr = test_attr.take_suite_concurrency_mode(&integra8_path);
    let test_concurrency_mode_expr = test_attr.take_test_concurrency_mode(&integra8_path);

    let suite_name_ident = decorated_mod.ident;
    let suite_vis = decorated_mod.vis;
    let mod_content = decorated_mod
        .content
        .map(|(_brace, mod_content)| mod_content)
        .unwrap_or_else(|| vec![]);

    let tokens = quote! {
        #suite_vis mod #suite_name_ident {
            #(#mod_content)*

            use crate::REGISTERED_COMPONENTS;

            #[#integra8_path ::linkme::distributed_slice(REGISTERED_COMPONENTS)]
            #[linkme(crate = #integra8_path ::linkme)]
            static REGISTERER_COMPONENTS: fn() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> = __suite_def;

            pub (crate) fn __suite_def() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> {
                #integra8_path ::decorations::ComponentDecoration::Suite(
                    #integra8_path ::decorations::SuiteAttributesDecoration {
                        name: #name_expr,
                        description: #description_expr,
                        path: module_path!(),
                        location: Some(#integra8_path ::components::src_loc!()),
                        ignore: #ignore_expr,
                        allow_suite_fail: #allow_fail_expr,
                        test_warning_time_limit: #test_warning_time_limit_expr,
                        test_time_limit: #test_time_limit_expr,
                        setup_time_limit: #setup_time_limit_expr,
                        tear_down_time_limit: #tear_down_time_limit_expr,
                        suite_concurrency_mode:  #suite_concurrency_mode_expr,
                        test_concurrency_mode:  #test_concurrency_mode_expr,
                    }
                )
            }
        }
    };

    TokenStream::from(tokens)
}
