use std::panic::UnwindSafe;
use std::sync::Arc;

use crate::runner::executor::execute;
use crate::runner::{
    ComponentFixture, ComponentProgressNotify, ComponentState, ComponentStateToken,
    RunProgressNotify, RunStateModel,
};

use crate::components::TestParameters;
use crate::scheduling::iter::TaskStreamMap;
use crate::scheduling::state_machine::TaskStateMachineNode;
use crate::scheduling::{ScheduledComponent, TaskScheduler};

use crate::results::report::{ComponentReportBuilder, ComponentRunReport};
use crate::results::summary::ComponentTypeCountSummary;

pub struct ScheduleRunner<RunProgressNotify> {
    sender: RunProgressNotify,
    status: RunStateModel,
}

impl<ProgressNotify: RunProgressNotify + Sync + Send + Clone + 'static>
    ScheduleRunner<ProgressNotify>
{
    pub fn new(sender: ProgressNotify) -> Self {
        Self {
            sender: sender,
            status: RunStateModel::new(),
        }
    }

    /// Runs the given schedule
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameter type as defined by the test author.
    ///
    /// * `schedule` - A ordered tree of components to be executed in order
    ///
    /// * `summary` - Count summary of all the components within the schedule
    ///
    pub async fn run<TParameters: TestParameters + Sync + Send + UnwindSafe + 'static>(
        mut self,
        parameters: TParameters,
        schedule: TaskStateMachineNode<ScheduledComponent<TParameters>>,
        summary: ComponentTypeCountSummary,
    ) {
        self.sender.notify_run_start(summary).await;
        let sender = self.sender.clone();

        let parameters = Arc::new(parameters);

        let scheduled_component_runs =
            schedule.map(|component| self.prepare_component_run(parameters.clone(), component));

        TaskScheduler::new(scheduled_component_runs, parameters.max_concurrency())
            .for_each_concurrent(|runner| async {
                if let ComponentRunResult::Ready(report) = runner.run().await {
                    // Only publish reports as they are ready.
                    // In the case of a suite, they run twice.
                    // First time is to update child components to reflect their
                    // parents suites status
                    // Second is to complete the suite and publish its result
                    sender.notify_component_report_complete(report).await;
                }
            })
            .await;

        self.sender.notify_run_complete().await;
    }

    fn prepare_component_run<TParameters: TestParameters + Sync + Send + UnwindSafe + 'static>(
        &mut self,
        parameters: Arc<TParameters>,
        component: ScheduledComponent<TParameters>,
    ) -> ComponentRunner<TParameters, <ProgressNotify as RunProgressNotify>::ComponentProgressNotify>
    {
        let fixture = ComponentFixture::from_scheduled_component(component, parameters);

        ComponentRunner {
            component_state: self.status.get_status_token(fixture.description()),
            progress_notify: self
                .sender
                .component_process_notify(fixture.description().clone()),
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
    > ComponentRunner<TParameters, ProgressNotify>
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
            ComponentRunResult::WaitingOnChildren => {
                // A Suite will wait on children before its results can be
                // determined, if all the children are ignored or there are no
                // children then the assume a pass results. So the suite is
                // automatically given a tentative pass result
                component_state.tentative_pass();
                ComponentRunResult::WaitingOnChildren
            }
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
        ComponentRunResult::Ready(execute(self.progress_notify, self.fixture, self.report).await)
    }
}
