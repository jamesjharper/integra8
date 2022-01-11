use std::time::Duration;

use integra8_components::{
    TestParameters, Delegate, BookEnd, BookEndAttributes, BookEnds, ComponentDescription, ComponentLocation, SuiteAttributes, ComponentGeneratorId
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BookEndDecorationPair<TParameters> {
    pub setup: Option<BookEndDecoration<TParameters>>,
    pub tear_down: Option<BookEndDecoration<TParameters>>,
}

impl<TParameters> BookEndDecorationPair<TParameters> {
    pub fn new() -> Self {
        Self {
            setup: None,
            tear_down: None,
        }
    }

    pub fn has_any(&self) -> bool {
        self.setup.is_some() || self.tear_down.is_some()
    }
}

impl<TParameters: TestParameters> BookEndDecorationPair<TParameters> {
    pub fn into_components(
        self,
        id_gen: &mut ComponentGeneratorId,
        parent_suite_description: &ComponentDescription,
        parent_suite_attributes: &SuiteAttributes,
    ) -> BookEnds<TParameters> {
        BookEnds {
            setup: self.setup.map(|deco| {
                deco.into_setup_component(id_gen, parent_suite_description, parent_suite_attributes)
            }),
            tear_down: self.tear_down.map(|deco| {
                deco.into_tear_down_component(id_gen, parent_suite_description, parent_suite_attributes)
            }),
        }
    }
}

impl<TParameters> Default for BookEndDecorationPair<TParameters> {
    fn default() -> Self {
        Self {
            setup: None,
            tear_down: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BookEndAttributesDecoration {
    // The name of the test (Default: the bookends namespace + method name)
    pub name: Option<&'static str>,

    // A description of the bookend which can be displayed by the output formatter if it supports it
    pub description: Option<&'static str>,

    /// The path used to calculate the bookends test group
    pub path: &'static str,

    /// The source code location of this bookend
    pub location: Option<ComponentLocation>,

    /// Indicates that bookend should not be run.
    pub ignore: Option<bool>,

    /// Describes the maximum duration a bookend can take before it is forcibly aborted
    pub critical_threshold: Option<Duration>,
}

impl BookEndAttributesDecoration {
    pub fn into_attributes(self, parent_desc: &SuiteAttributes) -> BookEndAttributes {
        BookEndAttributes::new(parent_desc, self.ignore, self.critical_threshold)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
            self.desc.path,
            self.desc.location,
            self.desc.ignore,
            self.desc.critical_threshold,
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
            self.desc.path,
            self.desc.location,
            self.desc.ignore,
            self.desc.critical_threshold,
            self.bookend_fn,
        )
    }
}
