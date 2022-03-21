use async_trait::async_trait;
use std::panic::UnwindSafe;
use std::sync::Arc;

pub mod executor;
mod state;
pub use state::{ComponentState, ComponentStateToken, RunStateModel};

pub mod notify;
pub use notify::{ComponentProgressNotify, RunProgressNotify, NullComponentProgressChannelNotify};

mod fixture;
pub use fixture::ComponentFixture;

mod schedule_runner;
pub use schedule_runner::ScheduleRunner;

use crate::components::TestParameters;
use crate::scheduling::state_machine::TaskStateMachineNode;
use crate::scheduling::ScheduledComponent;
use crate::results::report::{ComponentReportBuilder, ComponentRunReport};
use crate::results::summary::ComponentTypeCountSummary;

/// IOC code seem for internal test and customization extensions to the framework
#[async_trait]
pub trait ResolveRunnerStrategy<Parameters: TestParameters, ProgressNotify: RunProgressNotify> {
    fn create(parameters: &Parameters) -> Self
    where
        Self: Sized;

    /// Runs the given schedule
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `notify` - The results observer, used for publishing test results back to an observing thread.
    ///
    /// * `schedule` - A ordered tree of components to be executed in order
    ///
    /// * `summary` - Count summary of all the components within the schedule
    ///
    async fn run_schedule(
        &mut self,
        parameters: Parameters,
        notify: ProgressNotify,
        schedule: TaskStateMachineNode<ScheduledComponent<Parameters>>,
        summary: ComponentTypeCountSummary,
    );

    /// Runs a given component
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `scheduled_component` - The component to be executed
    ///
    async fn run_component(
        &mut self,
        parameters: Parameters,
        scheduled_component: ScheduledComponent<Parameters>,
    ) -> ComponentRunReport;
}

/// Inbuilt implementation for resolving test results formatters
pub struct DefaultResolveRunnerStrategy;

#[async_trait]
impl<
        Parameters: TestParameters + Sync + Send + UnwindSafe + 'static,
        ProgressNotify: RunProgressNotify + Sync + Send + Clone + 'static,
    > ResolveRunnerStrategy<Parameters, ProgressNotify> for DefaultResolveRunnerStrategy
{
    fn create(_parameters: &Parameters) -> Self
    where
        Self: Sized,
    {
        Self
    }

    /// Runs the given schedule
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `notify` - The results observer, used for publishing test results back to an observing thread.
    ///
    /// * `schedule` - A ordered tree of components to be executed in order
    ///
    /// * `summary` - Count summary of all the components within the schedule
    ///
    async fn run_schedule(
        &mut self,
        parameters: Parameters,
        notify: ProgressNotify,
        schedule: TaskStateMachineNode<ScheduledComponent<Parameters>>,
        summary: ComponentTypeCountSummary,
    ) {
        ScheduleRunner::new(notify)
            .run(parameters, schedule, summary)
            .await
    }

    /// Runs a given component
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `scheduled_component` - The component to be executed
    ///
    async fn run_component(
        &mut self,
        parameters: Parameters,
        scheduled_component: ScheduledComponent<Parameters>,
    ) -> ComponentRunReport {
        let fixture =
            ComponentFixture::from_scheduled_component(scheduled_component, Arc::new(parameters));
        let report = ComponentReportBuilder::new(
            fixture.description().clone(),
            fixture.acceptance_criteria(),
        );
        executor::execute(NullComponentProgressChannelNotify {}, fixture, report)
            .await
            .build()
    }
}
