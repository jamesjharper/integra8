use std::time::Duration;

use integra8_components::{
    BookEnd, ComponentDescription, ComponentGeneratorId, ComponentLocation, ConcurrencyMode,
    Delegate, SuiteAttributes, TestParameters,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BookEndAttributesDecoration {
    // The name of the test (Default: the bookends namespace + method name)
    pub name: Option<&'static str>,

    // A description of the bookend which can be displayed by the output formatter if it supports it
    pub description: Option<&'static str>,

    /// The source code location of this bookend
    pub location: ComponentLocation,

    /// Indicates that bookend should not be run.
    pub ignore: Option<bool>,

    /// Describes the maximum duration a bookend can take before it is forcibly aborted
    pub time_limit: Option<Duration>,

    /// The concurrency mode which this bookend will adhere to.
    /// `ConcurrencyMode::Parallel` will allow this bookend for be run at the same time as other bookends within this suite
    /// `ConcurrencyMode::Sequential` will ensure that this bookend wont run at the same time as any other bookend from this suite
    pub concurrency_mode: Option<ConcurrencyMode>,
}

#[derive(Clone, Debug)]
pub struct BookEndDecoration<TParameters> {
    pub desc: BookEndAttributesDecoration,
    pub bookend_fn: Delegate<TParameters>,
}

impl<TParameters: TestParameters> BookEndDecoration<TParameters> {
    pub fn into_setup_component(
        self,
        id_gen: &mut ComponentGeneratorId,
        parent_suite_description: &ComponentDescription,
        parent_suite_attributes: &SuiteAttributes,
    ) -> BookEnd<TParameters> {
        BookEnd::new_setup(
            parent_suite_description,
            parent_suite_attributes,
            id_gen,
            self.desc.name,
            self.desc.description,
            self.desc.location,
            self.desc.ignore,
            self.desc.time_limit,
            self.desc.concurrency_mode,
            self.bookend_fn,
        )
    }

    pub fn into_tear_down_component(
        self,
        id_gen: &mut ComponentGeneratorId,
        parent_suite_description: &ComponentDescription,
        parent_suite_attributes: &SuiteAttributes,
    ) -> BookEnd<TParameters> {
        BookEnd::new_tear_down(
            parent_suite_description,
            parent_suite_attributes,
            id_gen,
            self.desc.name,
            self.desc.description,
            self.desc.location,
            self.desc.ignore,
            self.desc.time_limit,
            self.desc.concurrency_mode,
            self.bookend_fn,
        )
    }
}
