use std::time::Duration;
use crate::runner::context::ExecutionContext;
use crate::decorations::{BookEndDecoration, BookEndAttributesDecoration, BookEndDecorationPair};
use crate::components::{SuiteAttributes, ComponentIdentity, ComponentDescription, ComponentType};


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BookEnds<TParameters> {
    pub setup: Option<BookEnd<TParameters>>,
    pub tear_down: Option<BookEnd<TParameters>>,
}

impl<TParameters> BookEnds<TParameters> {

    pub fn new(
        parent_suite_description: &ComponentDescription,
        parent_suite_attributes: &SuiteAttributes,
        decoration_pair: BookEndDecorationPair<TParameters>,
    ) -> Self {

        Self {
            setup: decoration_pair.setup.map(|deco| {
                BookEnd::setup(parent_suite_description, parent_suite_attributes, deco)
            }),
            tear_down: decoration_pair.tear_down.map(|deco| {
                BookEnd::tear_down(parent_suite_description, parent_suite_attributes, deco)
            }),
        }
        
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BookEndAttributes {
    /// The identity of the bookend. Used for uniquely identify the bookend and displaying the test name to the end user.
    pub identity: ComponentIdentity,

    /// Indicates that bookend should not be run.
    pub ignore: bool,

    /// The owning suite of this bookend 
    pub parent_suite_identity: ComponentIdentity,

    /// Describes the maximum duration a bookend can take before it is forcibly aborted 
    pub critical_threshold: Option<Duration>,

}

impl BookEndAttributes {
    pub fn new(
        parent_desc: &SuiteAttributes,
        deco: BookEndAttributesDecoration, 
    ) -> Self {
        Self {
            identity: ComponentIdentity::new(deco.path,deco.path), // TODO: make it name-able?
            ignore: deco.ignore
                .unwrap_or_else(|| parent_desc.ignore),

            parent_suite_identity: parent_desc.identity.clone(),
            critical_threshold: deco.critical_threshold,
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BookEnd<TParameters> {
    pub attributes: BookEndAttributes,
    pub description: ComponentDescription,
    pub bookend_fn: BookEndFn<TParameters>
}


impl<TParameters> BookEnd<TParameters>  {
    pub fn setup(
        parent_suite_description: &ComponentDescription,
        parent_suite_attributes: &SuiteAttributes,
        decorations: BookEndDecoration<TParameters>,
    ) -> Self {
        Self {
            description: ComponentDescription {
                identity: ComponentIdentity::new(decorations.desc.path, decorations.desc.path),
                parent_identity: parent_suite_description.identity.clone(),
                component_type: ComponentType::Setup,
                location: decorations.desc.location.clone()
            },
            attributes: BookEndAttributes::new(parent_suite_attributes, decorations.desc),
            bookend_fn: BookEndFn {
                bookend_fn: decorations.bookend_fn
            }
        }
    }

    pub fn tear_down(
        parent_suite_description: &ComponentDescription,
        parent_suite_attributes: &SuiteAttributes,
        decorations: BookEndDecoration<TParameters>,
    ) -> Self {
        Self {
            description: ComponentDescription {
                identity: ComponentIdentity::new(decorations.desc.path, decorations.desc.path),
                parent_identity: parent_suite_description.identity.clone(),
                component_type: ComponentType::TearDown,
                location: decorations.desc.location.clone()
            },
            attributes: BookEndAttributes::new(parent_suite_attributes, decorations.desc),
            bookend_fn: BookEndFn {
                bookend_fn: decorations.bookend_fn
            }
        }
    }
}


#[cfg(feature = "async")]
pub type BookEndFn<TParameters> = bookend_async_impl::BookEndFnAsync<TParameters>;

#[cfg(feature = "async")]
mod bookend_async_impl {
    use super::*;

    use std::pin::Pin;
    use std::future::Future;

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct BookEndFnAsync<TParameters> {
        pub bookend_fn: fn(ExecutionContext<TParameters>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
    }
    
    impl<TParameters> BookEndFnAsync<TParameters> {
        pub async fn run(&self, params: ExecutionContext<TParameters>) {
            (self.bookend_fn)(params).await
        }
    }
}

#[cfg(feature = "sync")]
pub type BookEndFn<TParameters> = bookend_sync_impl::BookEndFnSync<TParameters>;

#[cfg(feature = "sync")]
mod bookend_sync_impl {

    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct BookEndFnSync<TParameters> {
        pub bookend_fn: fn(ExecutionContext<TParameters>)
    }
    
    impl<TParameters> BookEndFnSync<TParameters> {
        pub fn run(&self, params: ExecutionContext<TParameters>) {
            (self.bookend_fn)(params)
        }
    }
}
