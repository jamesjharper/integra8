
use futures::join;
use std::panic::UnwindSafe;

use crate::core::channel::notify::RunProgressChannelNotify;
use crate::core::channel::{ResultsChannel, ResultsOutputWriterSink};
use crate::core::TestApplicationLocator;

use integra8_results::summary::ComponentTypeCountSummary;
use integra8_results::ComponentResult;

use integra8_components::TestParameters;
use integra8_decorations::ComponentDecoration;

use integra8_scheduling::state_machine::{TaskStateMachineNode, TaskStream};

pub async fn run<
    TParameters: TestParameters + Clone + Sync + Send + UnwindSafe + 'static + std::fmt::Debug,
    Locator: TestApplicationLocator<TParameters> + Sync + Send + 'static,
>(
    auto_detect_components: Vec<ComponentDecoration<TParameters>>,
    parameters: TParameters,
) -> ComponentResult {

    // 1: Resolve all decorations 
    let decorations = Locator::resolve_decorations_strategy(&parameters)
        .resolve_decorations(&parameters, auto_detect_components);

    // 2: Count all the component types found in the decorations 
    let component_summary = decorations
        .iter()
        .fold(ComponentTypeCountSummary::new(), |mut count, c| {
            count.increment(&c.component_type());
            count
        });

    // 3: Build component hierarchy  
    let root_component = Locator::resolve_component_hierarchy_strategy(&parameters)
        .resolve_component_hierarchy(&parameters, decorations);

    // 4: Calculate component schedule
    let schedule = Locator::resolve_component_schedule_strategy(&parameters)
        .resolve_schedule(&parameters, root_component);

  
    // 5: Calculate the max needed concurrency based on the schedule, or limit to the 
    // max max concurrency parameter if smaller 
    let max_concurrency = schedule.max_concurrency_or_limit(parameters.max_concurrency());

    // 6: Setup results channels for publishing tests results
    let (sender, receiver) = ResultsChannel::new(
        ResultsOutputWriterSink::new(
            Locator::resolve_formatter_strategy(&parameters).resolve_formatter(&parameters)
        ), 
        max_concurrency
    );

    // 7: Run Tests using schedule
    let runner_task = integra8_async_runtime::spawn(async move {
        let mut runner = Locator::resolve_runner_strategy(&parameters);
        runner.run_schedule(parameters, RunProgressChannelNotify::new(sender), schedule, component_summary)
            .await;
    });

    // 8: Wait for results
    let receiver_task = receiver.start_listening();
    let (_, run_summary) = join!(runner_task, receiver_task);
    run_summary.run_result()
}