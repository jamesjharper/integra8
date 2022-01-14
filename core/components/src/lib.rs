pub mod context;
pub use context::{ExecutionContext, ExecutionStrategy, TestParameters};

pub mod delegates;
pub use delegates::Delegate;

pub mod test;
pub use test::{Test, TestAttributes};

pub mod bookends;
pub use bookends::{BookEnd, BookEndAttributes, BookEnds};

mod suite;
pub use suite::{Suite, SuiteAttributes};

mod acceptance_criteria;
pub use acceptance_criteria::{AcceptanceCriteria, TimingAcceptanceCriteria};

mod meta;
pub use meta::{
    ComponentDescription, ComponentGeneratorId, ComponentId, ComponentLocation, ComponentPath,
    ComponentType, ConcurrencyMode,
};

//use integra8_decorations::ComponentDecoration;

/*
pub struct RootSuite();

impl RootSuite {
    pub fn from_decorated_components<ComponentsIterator, TParameters: TestParameters>(
        components: ComponentsIterator,
        parameters: &TParameters,
    ) -> Suite<TParameters>
    where
        ComponentsIterator: IntoIterator<Item = ComponentDecoration<TParameters>>,
    {
        Suite::<TParameters>::from_decorated_components(components, parameters)
    }
}*/
