
mod test;
pub use test::{TestAttributesDecoration, TestDecoration};

mod bookends;
pub use bookends::{BookEndAttributesDecoration, BookEndDecoration, BookEndDecorationPair};

mod suite;
pub use suite::SuiteAttributesDecoration;

mod hierarchy;
pub use hierarchy::{ComponentGroup, ComponentHierarchy};

#[derive(Debug)]
pub enum ComponentDecoration<TParameters> {
    IntegrationTest(TestDecoration<TParameters>),
    Suite(SuiteAttributesDecoration),
    TearDown(BookEndDecoration<TParameters>),
    Setup(BookEndDecoration<TParameters>),
}

impl<TParameters> ComponentDecoration<TParameters> {
    pub fn path(&self) -> &'static str {
        match self {
            ComponentDecoration::IntegrationTest(c) => c.desc.path,
            ComponentDecoration::Suite(c) => c.path,
            ComponentDecoration::TearDown(c) => c.desc.path,
            ComponentDecoration::Setup(c) => c.desc.path,
        }
    }
}
