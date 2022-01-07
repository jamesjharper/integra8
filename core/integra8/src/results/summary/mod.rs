mod summary;

#[doc(inline)]
pub use summary::{
    ComponentResultSummary, SuiteSummary, RunSummary
};

mod counts;

#[doc(inline)]
pub use counts::{
    DidNotRunResultsCountSummary, FailResultsCountSummary, PassResultsCountSummary,
    ResultsCountSummary,
};

mod results_iter;

#[doc(inline)]
pub use results_iter::{
    FailedReasonResults, FailedResults, NotRunReasonResults, NotRunResults, PassReasonResults,
    PassedResults,
};
