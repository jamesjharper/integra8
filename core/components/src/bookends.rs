use std::time::Duration;

use crate::{
    ComponentDescription, ComponentGeneratorId, ComponentLocation, ComponentPath, ComponentType,
    ConcurrencyMode, Delegate, SuiteAttributes,
};

#[derive(Clone, Debug)]
pub struct BookEndAttributes {
    /// Indicates that bookend should not be run.
    pub ignore: bool,

    /// Describes the maximum duration a bookend can take before it is forcibly aborted
    pub time_limit: Duration,

    /// The concurrency mode which this bookend will adhere to.
    /// `ConcurrencyMode::Parallel` will allow this bookend for be run at the same time as other bookends within this suite
    /// `ConcurrencyMode::Sequential` will ensure that this bookend wont run at the same time as any other bookend from this suite
    pub concurrency_mode: ConcurrencyMode,
}

impl BookEndAttributes {
    pub fn new_setup(
        parent_desc: &SuiteAttributes,
        ignore: Option<bool>,
        time_limit: Option<Duration>,
        concurrency_mode: Option<ConcurrencyMode>,
    ) -> Self {
        Self {
            ignore: ignore.unwrap_or_else(|| parent_desc.ignore),
            time_limit: time_limit.map_or_else(|| parent_desc.setup_time_limit, |val| val),

            concurrency_mode: concurrency_mode
                // Default Serial unless explicitly stated otherwise
                .map_or_else(|| ConcurrencyMode::Sequential, |val| val),
        }
    }

    pub fn new_tear_down(
        parent_desc: &SuiteAttributes,
        ignore: Option<bool>,
        time_limit: Option<Duration>,
        concurrency_mode: Option<ConcurrencyMode>,
    ) -> Self {
        Self {
            ignore: ignore.unwrap_or_else(|| parent_desc.ignore),
            time_limit: time_limit.map_or_else(|| parent_desc.tear_down_time_limit, |val| val),

            concurrency_mode: concurrency_mode
                // Default Serial unless explicitly stated otherwise
                .map_or_else(|| ConcurrencyMode::Sequential, |val| val),
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
        src: ComponentLocation,
        ignore: Option<bool>,
        time_limit: Option<Duration>,
        concurrency_mode: Option<ConcurrencyMode>,
        setup_fn: Delegate<TParameters>,
    ) -> Self {
        Self {
            description: ComponentDescription::new(
                ComponentPath::from(path),
                name,
                id_gen.next(),
                parent_suite_description.path().clone(),
                parent_suite_description.id().clone(),
                description,
                ComponentType::Setup,
                src,
            ),
            attributes: BookEndAttributes::new_setup(
                parent_suite_attributes,
                ignore,
                time_limit,
                concurrency_mode,
            ),
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
        src: ComponentLocation,
        ignore: Option<bool>,
        time_limit: Option<Duration>,
        concurrency_mode: Option<ConcurrencyMode>,
        setup_fn: Delegate<TParameters>,
    ) -> Self {
        Self {
            description: ComponentDescription::new(
                ComponentPath::from(path),
                name,
                id_gen.next(),
                parent_suite_description.path().clone(),
                parent_suite_description.id().clone(),
                description,
                ComponentType::TearDown,
                src,
            ),
            attributes: BookEndAttributes::new_tear_down(
                parent_suite_attributes,
                ignore,
                time_limit,
                concurrency_mode,
            ),
            bookend_fn: setup_fn,
        }
    }
}
