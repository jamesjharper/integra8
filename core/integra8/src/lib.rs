mod channel;

pub mod strategy;

pub mod formatters {
    pub use integra8_formatters::*;
}

pub mod results {
    pub use integra8_results::*;
}

pub mod scheduling {
    pub use integra8_scheduling::*;
}

pub mod context {
    pub use integra8_context::*;
}

pub mod decorations {
    pub use integra8_decorations::*;
}

pub use integra8_decorations_impl::*;

pub mod components {
    pub use integra8_components::*;
}

pub mod runner {
    pub use integra8_runner::*;
}

#[doc(hidden)]
pub mod linkme {
    pub use linkme::*;
}

#[doc(hidden)]
pub mod structopt {
    pub use structopt::*;
}

#[macro_export]
macro_rules! run_tests {
    ($parameters:expr) => {
        $crate::run_test(
            $parameters,
            REGISTERED_COMPONENTS.into_iter().map(|f| (f)()).collect(),
        )
        .await
    };
}

use futures::join;
use std::panic::UnwindSafe;

use channel::notify::RunProgressChannelNotify;
use channel::{ResultsChannel, ResultsOutputWriterSink};
use strategy::{
    DefaultResolveComponentScheduleStrategy, DefaultResolveComponentsStrategy,
    DefaultResolveFormatterStrategy, DefaultResolveRunnerStrategy, DefaultTestApplicationLocator,
    TestApplicationLocator,
};

use integra8_results::ComponentResult;
use integra8_runner::{DefaultScheduleRunner, ScheduleRunner};

use integra8_context::parameters::TestParameters;
use integra8_formatters::none::NoOutputFormatter;
use integra8_formatters::FormatterParameters;

use integra8_decorations::ComponentDecoration;

use integra8_scheduling::state_machine::{TaskStateMachineNode, TaskStream};
use integra8_scheduling::ScheduledComponent;

pub async fn run_test<
    TParameters: TestParameters
        + FormatterParameters
        + Clone
        + Sync
        + Send
        + UnwindSafe
        + 'static
        + std::fmt::Debug,
>(
    parameters: TParameters,
    components: Vec<ComponentDecoration<TParameters>>,
) -> i32 {
    match run::<
        TParameters,
        DefaultTestApplicationLocator<
            TParameters,
            DefaultResolveComponentsStrategy,
            DefaultResolveComponentScheduleStrategy,
            DefaultResolveFormatterStrategy,
            DefaultResolveRunnerStrategy,
        >,
    >(components, parameters)
    .await
    {
        ComponentResult::Pass(_) => 0,
        ComponentResult::Fail(_) => 1,
        ComponentResult::DidNotRun(_) => 3,
    }
}

pub async fn run<
    TParameters: TestParameters + Clone + Sync + Send + UnwindSafe + 'static + std::fmt::Debug,
    Locator: TestApplicationLocator<TParameters> + Sync + Send + 'static,
>(
    auto_detect_components: Vec<ComponentDecoration<TParameters>>,
    parameters: TParameters,
) -> ComponentResult {
    let components =
        resolve_components::<TParameters, Locator>(&parameters, auto_detect_components);
    let schedule = resolve_component_schedule::<TParameters, Locator>(&parameters, components);

    let max_concurrency = std::cmp::min(parameters.max_concurrency(), schedule.max_concurrency());

    let sink = resolve_results_sink::<TParameters, Locator>(&parameters);
    let (sender, receiver) = ResultsChannel::new(sink, max_concurrency);

    let runner_task = integra8_async_runtime::spawn(async move {
        DefaultScheduleRunner::new(RunProgressChannelNotify::new(sender))
            .run(parameters, schedule)
            .await;
    });

    let receiver_task = receiver.start_listening();
    let (_, run_summary) = join!(runner_task, receiver_task);
    run_summary.run_result()
}

pub fn resolve_results_sink<
    TParameters: TestParameters,
    Locator: TestApplicationLocator<TParameters>,
>(
    parameters: &TParameters,
) -> ResultsOutputWriterSink {
    let formatter = match parameters.is_child_process() {
        false => Locator::resolve_formatter_strategy(&parameters).resolve_formatter(&parameters),
        true => Box::new(NoOutputFormatter::new()),
    };

    ResultsOutputWriterSink::new(formatter)
}

pub fn resolve_components<
    TParameters: TestParameters,
    Locator: TestApplicationLocator<TParameters>,
>(
    parameters: &TParameters,
    auto_detect_components: Vec<ComponentDecoration<TParameters>>,
) -> Vec<ComponentDecoration<TParameters>> {
    let components = Locator::resolve_components_strategy(&parameters)
        .resolve_components(parameters, auto_detect_components);

    match parameters.is_child_process() {
        true => {
            // Child process is used to run a single component in a separate process
            components
                .into_iter()
                .find(|c| !parameters.exclude_component_predicate(c.path()))
                .into_iter()
                .collect()
        }
        false => components,
    }
}

pub fn resolve_component_schedule<
    TParameters: TestParameters,
    Locator: TestApplicationLocator<TParameters>,
>(
    parameters: &TParameters,
    components: Vec<ComponentDecoration<TParameters>>,
) -> TaskStateMachineNode<ScheduledComponent<TParameters>> {
    Locator::resolve_component_schedule_strategy(&parameters)
        .resolve_schedule(&parameters, components)
}

#[cfg(test)]
//type ExecutionContext  = crate::runner::context::ExecutionContext<MockParameters>;
#[cfg(test)]
#[derive(Clone, Debug, crate::structopt::StructOpt)]
#[structopt()]
pub struct MockParameters {}

#[cfg(test)]
type Parameters = MockParameters;

#[cfg(test)]
#[linkme::distributed_slice]
pub static REGISTERED_COMPONENTS: [fn() -> ComponentDecoration<Parameters>] = [..];