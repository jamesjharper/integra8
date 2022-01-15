use std::time::Duration;

use crate::{
    ComponentDescription, ComponentGeneratorId, ComponentLocation, ComponentPath, ComponentType,
    Delegate, SuiteAttributes,
};

#[derive(Clone, Debug)]
pub struct BookEnds<TParameters> {
    pub setup: Option<BookEnd<TParameters>>,
    pub tear_down: Option<BookEnd<TParameters>>,
}

#[derive(Clone, Debug)]
pub struct BookEndAttributes {
    /// Indicates that bookend should not be run.
    pub ignore: bool,

    /// Describes the maximum duration a bookend can take before it is forcibly aborted
    pub critical_threshold: Duration,
}

impl BookEndAttributes {
    pub fn new_setup(
        parent_desc: &SuiteAttributes,
        ignore: Option<bool>,
        critical_threshold: Option<Duration>,
    ) -> Self {
        Self {
            ignore: ignore.unwrap_or_else(|| parent_desc.ignore),
            critical_threshold: critical_threshold
                .map_or_else(|| parent_desc.setup_critical_threshold, |val| val),
        }
    }

    pub fn new_tear_down(
        parent_desc: &SuiteAttributes,
        ignore: Option<bool>,
        critical_threshold: Option<Duration>,
    ) -> Self {
        Self {
            ignore: ignore.unwrap_or_else(|| parent_desc.ignore),
            critical_threshold: critical_threshold
                .map_or_else(|| parent_desc.tear_down_critical_threshold, |val| val),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BookEnd<TParameters> {
    pub attributes: BookEndAttributes,
    pub description: ComponentDescription,
    pub bookend_fn: Delegate<TParameters>,
}

impl<TParameters> BookEnd<TParameters> {
    pub fn new_setup(
        parent_suite_description: &ComponentDescription,
        parent_suite_attributes: &SuiteAttributes,
        id_gen: &mut ComponentGeneratorId,
        name: Option<&'static str>,
        description: Option<&'static str>,
        path: &'static str,
        src: Option<ComponentLocation>,
        ignore: Option<bool>,
        critical_threshold: Option<Duration>,
        setup_fn: Delegate<TParameters>,
    ) -> Self {
        Self {
            description: ComponentDescription::new(
                ComponentPath::from(path),
                name,
                id_gen.next(),
                parent_suite_description.path.clone(),
                parent_suite_description.parent_id.clone(),
                description,
                ComponentType::Setup,
                src,
            ),
            attributes: BookEndAttributes::new_setup(parent_suite_attributes, ignore, critical_threshold),
            bookend_fn: setup_fn,
        }
    }

    pub fn new_tear_down(
        parent_suite_description: &ComponentDescription,
        parent_suite_attributes: &SuiteAttributes,
        id_gen: &mut ComponentGeneratorId,
        name: Option<&'static str>,
        description: Option<&'static str>,
        path: &'static str,
        src: Option<ComponentLocation>,
        ignore: Option<bool>,
        critical_threshold: Option<Duration>,
        setup_fn: Delegate<TParameters>,
    ) -> Self {
        Self {
            description: ComponentDescription::new(
                ComponentPath::from(path),
                name,
                id_gen.next(),
                parent_suite_description.path.clone(),
                parent_suite_description.id.clone(),
                description,
                ComponentType::TearDown,
                src,
            ),
            attributes: BookEndAttributes::new_tear_down(parent_suite_attributes, ignore, critical_threshold),
            bookend_fn: setup_fn,
        }
    }
}
