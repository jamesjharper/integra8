use crate::components::suite::suite_attributes::SuiteAttributes;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};


pub fn register_suite(input_tokens: TokenStream) -> TokenStream {

    let mut decorated_mod = parse_macro_input!(input_tokens as ItemMod);

    let mut test_attr = match SuiteAttributes::take_from(&mut decorated_mod.attrs) {
        Ok(test_attr) => test_attr,
        Err(err) => return err,
    };

    let integra8_path = test_attr.take_integra8_path();
    let ignore_expr = test_attr.take_ignore();
    let allow_fail_expr = test_attr.take_allow_fail();
    let warn_threshold = test_attr.take_warn_threshold();
    let critical_threshold = test_attr.take_critical_threshold();
    let cascade_failure = test_attr.take_cascade_failure();

    let suite_name_ident = decorated_mod.ident;
    let suite_vis = decorated_mod.vis;
    let mod_content = decorated_mod.content
        .map(|(_brace, mod_content)| mod_content)
        .unwrap_or_else(|| vec![]);

    let tokens = quote! {
        #suite_vis mod #suite_name_ident {
            #(#mod_content)*

            static SUITE_NAME: &'static str = stringify!( #suite_name_ident );

            use crate::REGISTERED_COMPONENTS;

            #[#integra8_path ::linkme::distributed_slice(REGISTERED_COMPONENTS)]
            #[linkme(crate = #integra8_path ::linkme)]
            static REGISTERER_COMPONENTS: fn() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> = __suite_def;

            pub (crate) fn __suite_def() -> #integra8_path ::decorations::ComponentDecoration<crate::Parameters> {
                #integra8_path ::decorations::ComponentDecoration::Suite(
                    #integra8_path ::decorations::SuiteAttributesDecoration {
                        name: SUITE_NAME,
                        path: module_path!(),
                        location: current_source_location!(),
                        ignore: #ignore_expr,
                        allow_suite_fail: #allow_fail_expr,
                        test_warn_threshold: #warn_threshold,
                        test_critical_threshold: #critical_threshold,
                        suite_cascade_failure: #cascade_failure,
                        suite_concurrency_mode:  None, // TODO: add ability to select the concurrency mode
                        test_concurrency_mode:  None, // TODO: add ability to select the concurrency mode
                    }
                )
            }
        }
    };

    TokenStream::from(tokens)
}