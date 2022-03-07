use crate::components::{TestParameters, Component, ChildProcessComponentArgs};
use crate::decorations::{ComponentDecoration, ComponentGroup};
use crate::formatters::{FormatterParameters, OutputFormatter};
use crate::runner::ResolveRunnerStrategy;
use crate::scheduling::{IntoTaskStateMachine, ScheduledComponent, TaskStateMachineNode};
use crate::core::channel::RunProgressChannelNotify;

/// IOC code seem for internal test and customization extensions to the framework
pub trait ResolveDecorationStrategy<Parameters: TestParameters> {
    fn create(parameters: &Parameters) -> Self
    where
        Self: Sized;

    
    fn resolve_decorations(
        &mut self,
        parameters: &Parameters,
        auto_detected: Vec<ComponentDecoration<Parameters>>,
    ) ->  Vec<ComponentDecoration<Parameters>> {

        let decorations = self.resolve_additional_decorations(parameters, auto_detected);

        match parameters.child_process_target() {
            Some(child_process_parameters) => {
                self.filter_child_process(parameters, child_process_parameters, decorations).into_iter().collect()
            },
            None => {
                decorations
            }
        }
    }

    fn filter_child_process(
        &mut self,
        _parameters: &Parameters,
        child_process_parameters: &ChildProcessComponentArgs,
        decorations: Vec<ComponentDecoration<Parameters>>,
    ) -> Option<ComponentDecoration<Parameters>> {
        let target_component_path = &child_process_parameters.meta().path;
        // Child process is used to run a single component in a separate process
        decorations
            .into_iter()
            .find(|c| {
                &c.location().path == target_component_path
            })
    }


    /// Returns list of decorated component to be compiled into the test schedule.
    /// This is intended for 3rd party custom component registration and filtering logic.
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `auto_detected` - The list of already automatically detected component decoration.
    ///
    fn resolve_additional_decorations(
        &mut self,
        _parameters: &Parameters,
        auto_detected: Vec<ComponentDecoration<Parameters>>,
    ) -> Vec<ComponentDecoration<Parameters>> {
        auto_detected
    }
}


/// IOC code seem for internal test and customization extensions to the framework
pub trait ResolveComponentHierarchyStrategy<Parameters: TestParameters>  {
    fn create(parameters: &Parameters) -> Self
    where
        Self: Sized;

    
    fn resolve_component_hierarchy(
        &mut self,
        parameters: &Parameters,
        mut decorations: Vec<ComponentDecoration<Parameters>>,
    ) -> Component<Parameters> {
        match parameters.child_process_target() {
            Some(child_process_parameters) => {
                // TODO: Tidy up
                let d = decorations.pop().unwrap();
                child_process_parameters.clone().into_component(
                    d.name(),
                    d.description(),
                    d.location().clone(),
                    d.into_delegate().unwrap(),
                )
            },
            None => {
                Component::Suite(ComponentGroup::into_root_component(decorations, parameters))
            }
        }
    }
}

/// IOC code seem for internal test and customization extensions to the framework
pub trait ResolveComponentScheduleStrategy<Parameters: TestParameters>  {
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
    /// * `root_component` - The decoration list as determined by `ResolveDecorationStrategy::resolve_decorations(..)`
    ///
    fn resolve_schedule(
        &mut self,
        parameters: &Parameters,
        root_component: Component<Parameters>
    ) -> TaskStateMachineNode<ScheduledComponent<Parameters>>;
}

/// IOC code seem for internal test and customization extensions to the framework
pub trait ResolveFormatterStrategy<Parameters: TestParameters>  {
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



/// Inbuilt implementation for resolving components
pub struct DefaultResolveDecorationStrategy;

impl<Parameters: TestParameters>  ResolveDecorationStrategy<Parameters> for DefaultResolveDecorationStrategy {
    fn create(_parameters: &Parameters) -> Self
    where
        Self: Sized,
    {
        Self
    }
}

/// Inbuilt implementation for resolving Component Hierarchy
pub struct DefaultResolveComponentHierarchyStrategy;

impl<Parameters: TestParameters>  ResolveComponentHierarchyStrategy<Parameters> for DefaultResolveComponentHierarchyStrategy {
    fn create(_parameters: &Parameters) -> Self
    where
        Self: Sized,
    {
        Self
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

    /// Returns a state machine graph used to determine the order and concurrency behavior of the test components
    /// This is intended for 3rd party custom component registration and filtering logic.
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `root_component` - The decoration list as determined by `ResolveDecorationStrategy::resolve_decorations(..)`
    ///
    fn resolve_schedule(
        &mut self,
        _parameters: &Parameters,
        root_component: Component<Parameters>
    ) -> TaskStateMachineNode<ScheduledComponent<Parameters>> {
        root_component.into_task_state_machine()
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



/// IOC code seem for internal test and customization extensions to the framework
pub trait TestApplicationLocator<Parameters> {
    fn resolve_decorations_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveDecorationStrategy<Parameters>>;


    fn resolve_component_hierarchy_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveComponentHierarchyStrategy<Parameters>> ;

    fn resolve_component_schedule_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveComponentScheduleStrategy<Parameters>>;

    fn resolve_formatter_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveFormatterStrategy<Parameters>>;

    fn resolve_runner_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveRunnerStrategy<Parameters, RunProgressChannelNotify> + Send + Sync + 'static>;
}

// Resolver Formatter

pub struct DefaultTestApplicationLocator<
    TParameters,
    ResolveDecorations,
    ResolveComponentHierarchy,
    ResolveComponentSchedule,
    ResolveFormatter,
    ResolveRunner,
> {
    _parameter: std::marker::PhantomData<TParameters>,
    _resolve_decorations: std::marker::PhantomData<ResolveDecorations>,
    _resolve_component_hierarchy: std::marker::PhantomData<ResolveComponentHierarchy>,
    _resolve_component_schedule: std::marker::PhantomData<ResolveComponentSchedule>,
    _resolve_formatter: std::marker::PhantomData<ResolveFormatter>,
    _resolve_runner: std::marker::PhantomData<ResolveRunner>,
}

impl<
        Parameters: FormatterParameters + TestParameters,
        ResolveDecorations: ResolveDecorationStrategy<Parameters> + 'static,
        ResolveComponentHierarchy: ResolveComponentHierarchyStrategy<Parameters> + 'static,
        ResolveComponentSchedule: ResolveComponentScheduleStrategy<Parameters> + 'static,
        ResolveFormatter: ResolveFormatterStrategy<Parameters> + 'static,
        ResolveRunner: ResolveRunnerStrategy<Parameters, RunProgressChannelNotify> + Send + Sync + 'static,
    > TestApplicationLocator<Parameters>
    for DefaultTestApplicationLocator<
        Parameters,
        ResolveDecorations,
        ResolveComponentHierarchy,
        ResolveComponentSchedule,
        ResolveFormatter,
        ResolveRunner,
    >
{
    fn resolve_decorations_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveDecorationStrategy<Parameters>> {
        Box::new(ResolveDecorations::create(parameters))
    }

    fn resolve_component_hierarchy_strategy(
        parameters: &Parameters,
    ) -> Box<dyn ResolveComponentHierarchyStrategy<Parameters>> {
        Box::new(ResolveComponentHierarchy::create(parameters))
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
    ) -> Box<dyn ResolveRunnerStrategy<Parameters, RunProgressChannelNotify> + Send + Sync + 'static> {
        Box::new(ResolveRunner::create(parameters))
    }
}
