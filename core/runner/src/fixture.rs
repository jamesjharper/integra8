
use integra8_components::{TestParameters, ExecutionStrategy, ExecutionContext, AcceptanceCriteria, BookEnd, SuiteAttributes, Test, ComponentDescription, ComponentPath};

use std::sync::Arc;

pub enum ComponentFixture<TParameters> {
    Test {
        test: Test<TParameters>,
        parameters: Arc<TParameters>,
    },
    BookEnd {
        bookend: BookEnd<TParameters>,
        parameters: Arc<TParameters>,
    },
    Suite {
        description: ComponentDescription,
        attributes: SuiteAttributes,
        parameters: Arc<TParameters>,
    },
}

impl<TParameters: TestParameters> ComponentFixture<TParameters> {
    pub fn for_test(test: Test<TParameters>, parameters: Arc<TParameters>) -> Self {
        Self::Test {
            test: test,
            parameters: parameters,
        }
    }

    pub fn for_bookend(bookend: BookEnd<TParameters>, parameters: Arc<TParameters>) -> Self {
        Self::BookEnd {
            bookend: bookend,
            parameters: parameters,
        }
    }

    pub fn for_suite(
        description: ComponentDescription,
        attributes: SuiteAttributes,
        parameters: Arc<TParameters>,
    ) -> Self {
        Self::Suite {
            description: description,
            attributes: attributes,
            parameters: parameters,
        }
    }

    pub async fn run(&self) {
        match self {
            Self::Test {
                test, parameters, ..
            } => {
                let ctx = ExecutionContext {
                    parameters: parameters.as_ref(),
                    description : self.description()
                };
                test.test_fn.run_async(ctx).await
            }
            Self::BookEnd {
                bookend,
                parameters,
                ..
            } => {
                let ctx = ExecutionContext {
                    parameters: parameters.as_ref(),
                    description : self.description()
                };
                bookend.bookend_fn.run_async(ctx).await
            }
            Self::Suite { .. } => {
                // Can not run
            }
        }
    }

    pub fn acceptance_criteria(&self) -> AcceptanceCriteria {
        match self {
            Self::Test { test, .. } => AcceptanceCriteria::for_test(&test.attributes),
            Self::BookEnd { bookend, .. } => AcceptanceCriteria::for_bookend(&bookend.attributes),
            Self::Suite { attributes, .. } => AcceptanceCriteria::for_suite(&attributes),
        }
    }

    pub fn description<'a>(&'a self) -> &'a ComponentDescription {
        match self {
            Self::Test { test, .. } => {
                return &test.description;
            }
            Self::BookEnd { bookend, .. } => {
                return &bookend.description;
            }
            Self::Suite { description, .. } => {
                return &description;
            }
        }
    }

    pub fn execution_strategy(&self) -> ExecutionStrategy {
        match self {
            Self::Test { parameters, .. } => parameters.execution_strategy(),
            Self::BookEnd { parameters, .. } => parameters.execution_strategy(),
            Self::Suite { parameters, .. } => parameters.execution_strategy(),
        }
    }

    pub fn component_path(&self) -> ComponentPath {
        match self {
            Self::Test { test, .. } => {
                test.description.path.clone()
            }
            Self::BookEnd { bookend, .. } => {
                bookend.description.path.clone()
            }
            Self::Suite { description, .. } => {
                description.path.clone()
            }
        }
    }

    pub fn ignore(&self) -> bool {
        match self {
            Self::Test { test, .. } => {
                return test.attributes.ignore;
            }
            Self::BookEnd { bookend, .. } => {
                return bookend.attributes.ignore;
            }
            Self::Suite { attributes, .. } => {
                return attributes.ignore;
            }
        }
    }

    pub fn is_suite(&self) -> bool {
        match self {
            Self::Suite { .. } => true,
            _ => false,
        }
    }
}
