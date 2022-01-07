use std::panic::UnwindSafe;

use crate::components::RootSuite;
use crate::context::parameters::TestParameters;
use crate::decorations::ComponentDecoration;
use crate::formatters::{FormatterParameters, OutputFormatter};
use crate::runner::{DefaultScheduleRunner, ScheduleRunner};
use crate::scheduling::{IntoTaskStateMachine, ScheduledComponent, TaskStateMachineNode};

use crate::channel::RunProgressChannelNotify;

/// IOC code seem for internal test and customization extensions to the framework
pub trait ResolveComponentsStrategy<Parameters> {
    fn create(parameters: &Parameters) -> Self
    where
        Self: Sized;

    /// Returns list of decorated component to be compiled into the test schedule.
    /// This is intended for 3rd party custom component registration and filtering logic.
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `auto_detected` - The list of already automatically detected component decoration.
    ///
    fn resolve_components(
        &mut self,
        parameters: &Parameters,
        auto_detected: Vec<ComponentDecoration<Parameters>>,
    ) -> Vec<ComponentDecoration<Parameters>>;
}

/// IOC code seem for internal test and customization extensions to the framework
pub trait ResolveComponentScheduleStrategy<Parameters> {
    fn create(parameters: &Parameters) -> Self
    where
        Self: Sized;

    /// Returns a state machine graph used to determine the order and concurrency behavior of the test components
    /// This is intended for 3rd party custom component registration and filtering logic.
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `components` - The component list as determined by `ResolveComponentsStrategy`
    ///
    fn resolve_schedule(
        &mut self,
        parameters: &Parameters,
        components: Vec<ComponentDecoration<Parameters>>,
    ) -> TaskStateMachineNode<ScheduledComponent<Parameters>>;
}

/// IOC code seem for internal test and customization extensions to the framework
pub trait ResolveFormatterStrategy<Parameters> {
    fn create(parameters: &Parameters) -> Self
    where
        Self: Sized;

    /// Returns the output formatter which will consume the test results
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    fn resolve_formatter(&mut self, parameters: &Parameters) -> Box<dyn OutputFormatter>;
}

/// IOC code seem for internal test and customization extensions to the framework
pub trait ResolveRunnerStrategy<Parameters> {
    fn create(parameters: &Parameters) -> Self
    where
        Self: Sized;

    /// Returns the test runner which will consume the test schedule
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `sender` - The results sender, used for publishing test results back to the main tread.
    ///
    fn resolve_runner(
        &mut self,
        parameters: &Parameters,
        notify: RunProgressChannelNotify,
    ) -> Box<dyn ScheduleRunner<Parameters>>;
}

/// Inbuilt implementation for resolving components
pub struct DefaultResolveComponentsStrategy;

impl<Parameters> ResolveComponentsStrategy<Parameters> for DefaultResolveComponentsStrategy {
    fn create(_parameters: &Parameters) -> Self
    where
        Self: Sized,
    {
        Self
    }

    /// Returns list of decorated component to be compiled into the test schedule.
    /// Default implementation simply returns the auto detected components
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author (ignored).
    ///
    /// * `auto_detected` - The list of already automatically detected component decoration.
    ///
    fn resolve_components(
        &mut self,
        _parameters: &Parameters,
        auto_detected: Vec<ComponentDecoration<Parameters>>,
    ) -> Vec<ComponentDecoration<Parameters>> {
        auto_detected
    }
}

/// Inbuilt implementation for resolving components schedules
pub struct DefaultResolveComponentScheduleStrategy;

impl<Parameters: TestParameters> ResolveComponentScheduleStrategy<Parameters>
    for DefaultResolveComponentScheduleStrategy
{
    fn create(_parameters: &Parameters) -> Self
    where
        Self: Sized,
    {
        Self
    }

    /// Returns a state machine graph used to determine the order and concurrency behavior of the test components.
    /// Default implementation builds decorated components into a tree based of the each components path in the authored tests.
    ///
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `components` - The component list as determined by `ResolveComponentsStrategy`
    ///
    fn resolve_schedule(
        &mut self,
        parameters: &Parameters,
        components: Vec<ComponentDecoration<Parameters>>,
    ) -> TaskStateMachineNode<ScheduledComponent<Parameters>> {
        RootSuite::from_decorated_components(components, parameters).into_task_state_machine()
    }
}

/// Inbuilt implementation for resolving test results formatters
pub struct DefaultResolveFormatterStrategy;

impl<Parameters: FormatterParameters + TestParameters> ResolveFormatterStrategy<Parameters>
    for DefaultResolveFormatterStrategy
{
    fn create(_parameters: &Parameters) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn resolve_formatter(&mut self, parameters: &Parameters) -> Box<dyn OutputFormatter> {
        parameters.create_formatter().unwrap()
    }
}

/// Inbuilt implementation for resolving test results formatters
pub struct DefaultResolveRunnerStrategy;

impl<Parameters: TestParameters + Sync + Send + UnwindSafe + 'static>
    ResolveRunnerStrategy<Parameters> for DefaultResolveRunnerStrategy
{
    fn create(_parameters: &Parameters) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn resolve_runner(
        &mut self,
        _parameters: &Parameters,
        notify: RunProgressChannelNotify,
    ) -> Box<dyn ScheduleRunner<Parameters>> {
        Box::new(DefaultScheduleRunner::new(notify))
    }
}

/// IOC code seem for internal test and customization extensions to the framework
pub trait TestApplicationLocator<Parameters> {
    fn resolve_components_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveComponentsStrategy<Parameters>>;

    fn resolve_component_schedule_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveComponentScheduleStrategy<Parameters>>;

    fn resolve_formatter_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveFormatterStrategy<Parameters>>;

    fn resolve_runner_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveRunnerStrategy<Parameters>>;
}

// Resolver Formatter

pub struct DefaultTestApplicationLocator<
    TParameters,
    ResolveComponents,
    ResolveComponentSchedule,
    ResolveFormatter,
    ResolveRunner,
> {
    _parameter: std::marker::PhantomData<TParameters>,
    _resolve_components: std::marker::PhantomData<ResolveComponents>,
    _resolve_component_schedule: std::marker::PhantomData<ResolveComponentSchedule>,
    _resolve_formatter: std::marker::PhantomData<ResolveFormatter>,
    _resolve_runner: std::marker::PhantomData<ResolveRunner>,
}

impl<
        Parameters: FormatterParameters + TestParameters,
        ResolveComponents: ResolveComponentsStrategy<Parameters> + 'static,
        ResolveComponentSchedule: ResolveComponentScheduleStrategy<Parameters> + 'static,
        ResolveFormatter: ResolveFormatterStrategy<Parameters> + 'static,
        ResolveRunner: ResolveRunnerStrategy<Parameters> + 'static,
    > TestApplicationLocator<Parameters>
    for DefaultTestApplicationLocator<
        Parameters,
        ResolveComponents,
        ResolveComponentSchedule,
        ResolveFormatter,
        ResolveRunner,
    >
{
    fn resolve_components_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveComponentsStrategy<Parameters>> {
        Box::new(ResolveComponents::create(parameters))
    }

    fn resolve_component_schedule_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveComponentScheduleStrategy<Parameters>> {
        Box::new(ResolveComponentSchedule::create(parameters))
    }

    fn resolve_formatter_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveFormatterStrategy<Parameters>> {
        Box::new(ResolveFormatter::create(parameters))
    }

    fn resolve_runner_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveRunnerStrategy<Parameters>> {
        Box::new(ResolveRunner::create(parameters))
    }
}
