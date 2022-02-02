use std::mem;
use syn::{parse_quote, Expr, FnArg, ItemFn, Path};

pub struct ExecFn {
    exec_fn: Option<ItemFn>,
    delegate_expr: Option<Expr>,
}

impl ExecFn {
    pub fn from(exec_fn: syn::ItemFn, integra8_path: &Path) -> Self {
        enum Asyncness {
            Async,
            Synchronous,
        }

        enum Signature {
            HasParameters,
            NoParameters,
        }

        use Asyncness::{Async, Synchronous};
        use Signature::{HasParameters, NoParameters};

        let asyncness = match exec_fn.sig.asyncness.is_some() {
            true => Async,
            false => Synchronous,
        };

        let parameters = match exec_fn.sig.inputs.is_empty() {
            true => NoParameters,
            false => HasParameters,
        };

        let fn_name_ident = &exec_fn.sig.ident;
        let delegate_expr = match (asyncness, parameters) {
            (Async, HasParameters) => {
                match exec_fn.sig.inputs.first().unwrap() {
                    FnArg::Receiver(_) => {
                        // This obviously wont work, but should produce a some what meaningful error
                        /*
                              7 | #[integration_test]
                                | ^^^^^^^^^^^^^^^^^^^ incorrect number of function parameters
                                |
                                = note: expected fn pointer `fn(integra8::components::ExecutionContext<BaseParameters<EmptySettingsExtension, TreeFormatterParameters>>) -> Pin<_>`
                                                found fn item `fn() -> Pin<_> {test2::test_def::wrap_pin}`
                        */
                        parse_quote!(
                            {
                                fn #fn_name_ident () -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
                                    Box::pin(super:: #fn_name_ident ())
                                }
                                #integra8_path ::components::delegates::Delegate::async_with_context(#fn_name_ident)
                            }
                        )
                    }
                    FnArg::Typed(p) => {
                        let ty = &p.ty;
                        parse_quote!(
                            {
                                fn #fn_name_ident (ctx : #ty) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
                                    Box::pin(super:: #fn_name_ident (ctx))
                                }
                                #integra8_path ::components::delegates::Delegate::async_with_context(#fn_name_ident )
                            }
                        )
                    }
                }
            }
            (Async, NoParameters) => {
                parse_quote!(
                    {
                        fn wrap_pin() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
                            Box::pin(super:: #fn_name_ident ())
                        }

                        #integra8_path ::components::delegates::Delegate::async_without_context(wrap_pin)
                    }
                )
            }
            (Synchronous, HasParameters) => {
                parse_quote!(
                    #integra8_path ::components::delegates::Delegate::sync_with_context(super:: #fn_name_ident)
                )
            }
            (Synchronous, NoParameters) => {
                parse_quote!(
                    #integra8_path ::components::delegates::Delegate::sync_without_context(super:: #fn_name_ident)
                )
            }
        };

        Self {
            exec_fn: Some(exec_fn),
            delegate_expr: Some(delegate_expr),
        }
    }

    pub fn take_exec_fn(&mut self) -> ItemFn {
        mem::take(&mut self.exec_fn).unwrap()
    }

    pub fn take_delegate_expr(&mut self) -> Expr {
        mem::take(&mut self.delegate_expr).unwrap()
    }
}
