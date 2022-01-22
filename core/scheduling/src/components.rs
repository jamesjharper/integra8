use std::iter::Peekable;

use crate::state_machine::{ParallelTaskNode, SerialTaskNode, TaskStateMachineNode};

use integra8_components::{
    BookEnd, ComponentDescription, ComponentType, ConcurrencyMode, Suite, SuiteAttributes, Test,
    TestParameters,
};

#[derive(Clone, Debug)]
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

        // Schedule Suite Component before we start running this suites components. 
        // Suites don't have anything to execute, however the runner will still publish a start when it 
        // encountered this component, as well as checking to see if is this suite and its children should be
        // aborted due to its parent failing.
        root_node.enqueue(ScheduledComponent::Suite(
            self.description.clone(),
            self.attributes.clone(),
        ));

        // 1: Run all setup components in the order they appear
        root_node.enqueue(self.setups.into_task_state_machine());

        // 2: Run all test components in the order they appear
        root_node.enqueue(self.tests.into_task_state_machine());

        // 3: Run all child suites of this suite.
        // Queue in groups, depending on the child suites concurrency mode
        let mut parallel_suites = ParallelTaskNode::new();
        let mut serial_suites = SerialTaskNode::new();

        self.suites
            .drain(..)
            .for_each(|suite| match suite.attributes.suite_concurrency_mode {
                ConcurrencyMode::Sequential => {
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

        // 4: run all teardown components,
        root_node.enqueue(self.tear_downs.into_task_state_machine());

        // Schedule Component for the second time, so that the runner can finalize the suites results
        root_node.enqueue(ScheduledComponent::Suite(
            self.description, 
            self.attributes
        ));

        root_node.into()
    }
}

impl<TParameters: TestParameters> IntoTaskStateMachine<ScheduledComponent<TParameters>>
    for Vec<Test<TParameters>>
{
    fn into_task_state_machine(self) -> TaskStateMachineNode<ScheduledComponent<TParameters>> {
        IntoComponentTaskStepIterator::from(self.into_iter().map(|x| TaskStepComponent::Test(x)))
            .fold(SerialTaskNode::new(), |mut seq, node| {
                seq.enqueue(node);
                seq
            })
            .into()
    }
}

impl<TParameters: TestParameters> IntoTaskStateMachine<ScheduledComponent<TParameters>>
    for Vec<BookEnd<TParameters>>
{
    fn into_task_state_machine(self) -> TaskStateMachineNode<ScheduledComponent<TParameters>> {
        IntoComponentTaskStepIterator::from(self.into_iter().map(|x| {
            if x.description.component_type() == &ComponentType::Setup {
                TaskStepComponent::Setup(x)
            } else {
                TaskStepComponent::TearDown(x)
            }
        }))
        .fold(SerialTaskNode::new(), |mut seq, node| {
            seq.enqueue(node);
            seq
        })
        .into()
    }
}

enum TaskStepComponent<TParameters> {
    Test(Test<TParameters>),
    Setup(BookEnd<TParameters>),
    TearDown(BookEnd<TParameters>),
}

impl<TParameters> TaskStepComponent<TParameters> {
    pub fn concurrency_mode(&self) -> &'_ ConcurrencyMode {
        match self {
            Self::Test(test) => &test.attributes.concurrency_mode,
            Self::Setup(setup) => &setup.attributes.concurrency_mode,
            Self::TearDown(tear_down) => &tear_down.attributes.concurrency_mode,
        }
    }

    pub fn into_scheduled_component(self) -> ScheduledComponent<TParameters> {
        match self {
            Self::Test(test) => ScheduledComponent::Test(test),
            Self::Setup(setup) => ScheduledComponent::Setup(setup),
            Self::TearDown(tear_down) => ScheduledComponent::TearDown(tear_down),
        }
    }
}

struct IntoComponentTaskStepIterator<TParameters, I>
where
    I: Iterator<Item = TaskStepComponent<TParameters>>,
{
    iter: Peekable<I>,
}

impl<TParameters: TestParameters, I> IntoComponentTaskStepIterator<TParameters, I>
where
    I: Iterator<Item = TaskStepComponent<TParameters>>,
{
    pub fn from(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

impl<TParameters: TestParameters, I> Iterator for IntoComponentTaskStepIterator<TParameters, I>
where
    I: Iterator<Item = TaskStepComponent<TParameters>>,
{
    type Item = TaskStateMachineNode<ScheduledComponent<TParameters>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;

        // If this test isn't parallel then,
        // yield an array of a single test
        if next.concurrency_mode() == &ConcurrencyMode::Sequential {
            return Some(next.into_scheduled_component().into());
        }

        // Yield sequences of tests which can be executed in parallel
        let mut parallel_group = ParallelTaskNode::new();
        parallel_group.append(next.into_scheduled_component());

        while let Some(next) = self
            .iter
            .next_if(|x| x.concurrency_mode() == &ConcurrencyMode::Parallel)
        {
            parallel_group.append(next.into_scheduled_component());
        }
        Some(parallel_group.into())
    }
}
