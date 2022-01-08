use std::iter::Peekable;
use std::vec::IntoIter;

use crate::state_machine::{ParallelTaskNode, SerialTaskNode, TaskStateMachineNode};

use integra8_components::{
    BookEnd, ComponentDescription, ConcurrencyMode, Suite, SuiteAttributes, Test,
};
use integra8_context::parameters::TestParameters;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScheduledComponent<TParameters> {
    Suite(ComponentDescription, SuiteAttributes),
    Test(Test<TParameters>),
    Setup(BookEnd<TParameters>),
    TearDown(BookEnd<TParameters>),
}

pub trait IntoTaskStateMachine<Payload> {
    fn into_task_state_machine(self) -> TaskStateMachineNode<Payload>;
}

impl<TParameters: TestParameters> IntoTaskStateMachine<ScheduledComponent<TParameters>>
    for Suite<TParameters>
{
    fn into_task_state_machine(mut self) -> TaskStateMachineNode<ScheduledComponent<TParameters>> {
        let mut root_node = SerialTaskNode::new();

        root_node.enqueue(ScheduledComponent::Suite(
            self.description.clone(),
            self.attributes.clone(),
        ));

        // 1: Run all setup components in sequence
        root_node.enqueue_all(
            self.bookends
                .iter_mut()
                .filter_map(|b| std::mem::take(&mut b.setup))
                .map(|setup| ScheduledComponent::Setup(setup)),
        );

        // 2: Run all test belonging to this suite
        root_node.enqueue(self.tests.into_task_state_machine());

        // 3: Run all child suites of this suite, and queue, depending on
        // each child suites concurrency mode

        let mut parallel_suites = ParallelTaskNode::new();
        let mut serial_suites = SerialTaskNode::new();

        self.suites
            .drain(..)
            .for_each(|suite| match suite.attributes.suite_concurrency_mode {
                ConcurrencyMode::Serial => {
                    serial_suites.enqueue(suite.into_task_state_machine());
                }
                ConcurrencyMode::Parallel => {
                    parallel_suites.append(suite.into_task_state_machine());
                }
            });

        // Favor running concurrent suites over serial onces,
        // by running as many tests upfront as posable we can
        // increases the chances we fail sooner, rather then later
        root_node.enqueue(parallel_suites);
        root_node.enqueue(serial_suites);

        // 4: run all teardown components, in reverse order to
        // the setup components
        root_node.enqueue_all(
            self.bookends
                .iter_mut()
                .rev()
                .filter_map(|b| std::mem::take(&mut b.tear_down))
                .map(|tear_down| ScheduledComponent::TearDown(tear_down)),
        );

        root_node.enqueue(ScheduledComponent::Suite(self.description, self.attributes));

        root_node.into()
    }
}

impl<TParameters: TestParameters> IntoTaskStateMachine<ScheduledComponent<TParameters>>
    for Vec<Test<TParameters>>
{
    fn into_task_state_machine(self) -> TaskStateMachineNode<ScheduledComponent<TParameters>> {
        TestIntoComponentTaskStepIterator::new(self)
            .fold(SerialTaskNode::new(), |mut seq, node| {
                seq.enqueue(node);
                seq
            })
            .into()
    }
}

pub struct TestIntoComponentTaskStepIterator<TParameters> {
    test_iter: Peekable<IntoIter<Test<TParameters>>>,
}

impl<TParameters: TestParameters> TestIntoComponentTaskStepIterator<TParameters> {
    pub fn new(tests: Vec<Test<TParameters>>) -> Self {
        Self {
            test_iter: tests.into_iter().peekable(),
        }
    }
}

impl<TParameters: TestParameters> Iterator for TestIntoComponentTaskStepIterator<TParameters> {
    type Item = TaskStateMachineNode<ScheduledComponent<TParameters>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.test_iter.next()?;

        // If this test isn't parallelizable then,
        // yield an array of a single test
        if next.attributes.concurrency_mode == ConcurrencyMode::Serial {
            return Some(ScheduledComponent::Test(next).into());
        }

        // Yield sequences of tests which can be executed in parallel
        let mut parallelizable_group = ParallelTaskNode::new();
        parallelizable_group.append(ScheduledComponent::Test(next));

        while let Some(next) = self
            .test_iter
            .next_if(|x| x.attributes.concurrency_mode == ConcurrencyMode::Parallel)
        {
            parallelizable_group.append(ScheduledComponent::Test(next));
        }
        Some(parallelizable_group.into())
    }
}
