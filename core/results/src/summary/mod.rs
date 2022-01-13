mod summary;

#[doc(inline)]
pub use summary::RunSummary;

#[doc(inline)]
pub use summary::SuiteSummary;

#[doc(inline)]
pub use summary::ComponentResultSummary;

mod results_iter;

#[doc(inline)]
pub use results_iter::{
    FailedReasonResults, FailedResults, NotRunReasonResults, NotRunResults, PassReasonResults,
    PassedResults,
};

mod counts;

#[doc(inline)]
pub use counts::{
    DidNotRunResultsCountSummary, FailResultsCountSummary, PassResultsCountSummary,
    ResultsCountSummary, ComponentTypeCountSummary
};