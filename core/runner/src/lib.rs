pub mod executor;

mod state;
pub use state::{ComponentState, ComponentStateToken, RunStateModel};

pub mod notify;
pub use notify::{RunProgressNotify, ComponentProgressNotify};

mod fixture;
pub use fixture::ComponentFixture;

use std::panic::UnwindSafe;
use std::sync::Arc;

use integra8_context::parameters::TestParameters;
use integra8_context::ExecutionStrategy;
use integra8_scheduling::iter::TaskStreamMap;
use integra8_scheduling::state_machine::TaskStateMachineNode;
use integra8_scheduling::{TaskScheduler, ScheduledComponent};

use integra8_results::report::{ComponentReportBuilder, ComponentRunReport};
use crate::executor::{process_external_executor, process_internal_executor, Executor};

use std::future::Future;
use std::pin::Pin;

pub trait ScheduleRunner<TParameters> {
    fn run(
        self,
        parameters: TParameters,
        schedule: TaskStateMachineNode<ScheduledComponent<TParameters>>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

pub struct DefaultScheduleRunner<RunProgressNotify> {
    pub sender: RunProgressNotify,
    pub status: RunStateModel,
}

impl<
    TParameters: TestParameters + Sync + Send + UnwindSafe + 'static,
    ProgressNotify: RunProgressNotify + Sync + Send + Clone + 'static,
    > ScheduleRunner<TParameters>
    for DefaultScheduleRunner<ProgressNotify>
{
    fn run(
        self,
        parameters: TParameters,
        schedule: TaskStateMachineNode<ScheduledComponent<TParameters>>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        async fn run<
            TInnerParameters: TestParameters + Sync + Send + UnwindSafe + 'static,
            InnerProgressNotify: RunProgressNotify + Clone + Send + Sync + 'static,
        >(
            mut runner: DefaultScheduleRunner<InnerProgressNotify>,
            parameters: TInnerParameters,
            schedule: TaskStateMachineNode<ScheduledComponent<TInnerParameters>>,
        ) {
            let sender = runner.sender.clone();

            let parameters = Arc::new(parameters);

            let scheduled_component_runs = schedule
                .map(|component| runner.prepare_component_run(parameters.clone(), component));

            TaskScheduler::new(scheduled_component_runs, parameters.max_concurrency())
                .for_each_concurrent(|runner| async {
                    if let ComponentRunResult::Ready(report) = runner.run().await {
                        sender.notify_component_complete(report).await;
                    }
                })
                .await;

            runner.sender.notify_run_complete().await;
        }

        Box::pin(run(self, parameters, schedule))
    }
}

impl<
    ProgressNotify: RunProgressNotify + Sync + Send + Clone + 'static,
> DefaultScheduleRunner<ProgressNotify> {
    pub fn new(sender: ProgressNotify) -> Self {
        Self {
            sender: sender,
            status: RunStateModel::new(),
        }
    }

    fn prepare_component_run<TParameters: TestParameters + Sync + Send + UnwindSafe + 'static>(
        &mut self,
        parameters: Arc<TParameters>,
        component: ScheduledComponent<TParameters>,
    ) -> ComponentRunner<TParameters, <ProgressNotify as RunProgressNotify>::ComponentProgressNotify> {
        let fixture = match component {
            ScheduledComponent::Test(c) => ComponentFixture::for_test(c, parameters),
            ScheduledComponent::Setup(c) | ScheduledComponent::TearDown(c) => {
                ComponentFixture::for_bookend(c, parameters)
            }
            ScheduledComponent::Suite(description, attributes) => {
                ComponentFixture::for_suite(description, attributes, parameters)
            }
        };

        ComponentRunner {
            component_state: self.status.get_status_token(fixture.description()),
            progress_notify: self.sender.component_process_notify(fixture.description().clone()),
            report: ComponentReportBuilder::new(
                fixture.description().clone(),
                fixture.acceptance_criteria(),
            ),
            fixture: fixture,
        }
    }
}

pub enum ComponentRunResult<Report> {
    Ready(Report),
    AlreadyPublished(Report),
    WaitingOnChildren,
}

pub struct ComponentRunner<
    TParameters: TestParameters + Send + Sync + UnwindSafe + 'static,
    ProgressNotify: ComponentProgressNotify + Send + Sync + 'static,
> {
    pub component_state: ComponentStateToken,
    pub progress_notify: ProgressNotify,
    pub report: ComponentReportBuilder,
    pub fixture: ComponentFixture<TParameters>,
}

impl<
    TParameters: TestParameters + Send + Sync + UnwindSafe + 'static,
    ProgressNotify: ComponentProgressNotify + Send + Sync + 'static,
>
    ComponentRunner<TParameters, ProgressNotify>
{
    pub async fn run(self) -> ComponentRunResult<ComponentRunReport> {
        let component_state = self.component_state.clone();

        match self.evaluate().await {
            ComponentRunResult::Ready(report_builder) => {
                let report = report_builder.build();
                component_state.finalize_result(report.result.clone(), report.timing.duration());

                ComponentRunResult::Ready(report)
            }
            ComponentRunResult::AlreadyPublished(report_builder) => {
                ComponentRunResult::AlreadyPublished(report_builder.build())
            }
            ComponentRunResult::WaitingOnChildren => ComponentRunResult::WaitingOnChildren,
        }
    }

    async fn evaluate(mut self) -> ComponentRunResult<ComponentReportBuilder> {
        match self.component_state.state() {
            ComponentState::Undetermined => self.execute().await,
            ComponentState::Tentative(result) => {
                self.report.time_taken(self.component_state.time_taken());
                self.report.with_result(result.clone());
                ComponentRunResult::Ready(self.report)
            }
            ComponentState::Finalized(result) => {
                self.report.time_taken(self.component_state.time_taken());
                self.report.with_result(result);
                ComponentRunResult::AlreadyPublished(self.report)
            }
        }
    }

    async fn execute(mut self) -> ComponentRunResult<ComponentReportBuilder> {
        if self.fixture.ignore() {
            self.report.ignored_result();
            return ComponentRunResult::Ready(self.report);
        }

        if self.fixture.is_suite() {
            // Suites cant "run", they are only a projection of their children's results.
            return ComponentRunResult::WaitingOnChildren;
        }

        // execute to determine the components state
        match self.fixture.execution_strategy() {
            ExecutionStrategy::ProcessInternal => ComponentRunResult::Ready(
                process_internal_executor()
                    .execute(self.progress_notify, self.fixture, self.report)
                    .await,
            ),
            ExecutionStrategy::ProcessExternal => ComponentRunResult::Ready(
                process_external_executor()
                    .execute(self.progress_notify, self.fixture, self.report)
                    .await,
            ),
        }
    }
}
