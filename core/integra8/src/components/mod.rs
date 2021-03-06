pub mod context;
pub use context::{
    ChildProcessComponentArgs, ChildProcessComponentMetaArgs, ExecutionArtifact,
    ExecutionArtifacts, ExecutionContext, ExecutionStrategy, TestParameters,
};

pub mod delegates;
pub use delegates::Delegate;

pub mod test;
pub use test::{Test, TestAttributes};

pub mod bookends;
pub use bookends::{BookEnd, BookEndAttributes};

mod suite;
pub use suite::{Suite, SuiteAttributes};

mod acceptance_criteria;
pub use acceptance_criteria::{AcceptanceCriteria, TimingAcceptanceCriteria};

pub mod macros;

mod meta;
pub use meta::{
    ComponentDescription, ComponentGeneratorId, ComponentId, ComponentLocation, ComponentPath,
    ComponentType, ConcurrencyMode,
};

#[derive(Clone, Debug)]
pub enum Component<TParameters> {
    Suite(Suite<TParameters>),
    Test(Test<TParameters>),
    Setup(BookEnd<TParameters>),
    TearDown(BookEnd<TParameters>),
}
