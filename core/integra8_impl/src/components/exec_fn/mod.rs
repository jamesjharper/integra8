use std::mem;
use syn::{parse_quote, ItemFn};

pub struct ExecFn {
    exec_fn: Option<ItemFn>,
}

impl ExecFn {
    pub fn from(exec_fn: syn::ItemFn) -> Self {
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

        let test_name_ident = &exec_fn.sig.ident;
        let fn_item = match (asyncness, parameters) {
            (Async, HasParameters) => {
                parse_quote!(
                    pub fn #test_name_ident (p: crate::ExecutionContext) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
                        #exec_fn
                        Box::pin(#test_name_ident(p))
                    }
                )
            }
            (Async, NoParameters) => {
                parse_quote!(
                    pub fn #test_name_ident (_: crate::ExecutionContext) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
                        #exec_fn
                        Box::pin(#test_name_ident())
                    }
                )
            }
            (Synchronous, HasParameters) => {
                parse_quote!(
                    pub fn #test_name_ident (p: crate::ExecutionContext) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
                        #exec_fn
                        Box::pin(async {
                            #test_name_ident(p)
                        })
                    }
                )
            }
            (Synchronous, NoParameters) => {
                parse_quote!(
                    pub fn #test_name_ident (_: crate::ExecutionContext) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
                        #exec_fn
                        Box::pin(async {
                            #test_name_ident()
                        })
                    }
                )
            }
        };

        Self {
            exec_fn: Some(fn_item),
        }
    }

    pub fn take_exec_fn(&mut self) -> ItemFn {
        mem::take(&mut self.exec_fn).unwrap()
    }
}
