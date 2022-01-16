use crate::summary::counts::ResultReasonCounter;
use crate::summary::{
    DidNotRunResultsCountSummary, FailResultsCountSummary, PassResultsCountSummary,
    WarningResultsCountSummary,
};

use crate::report::ComponentRunReport;
use crate::ComponentResult;
use crate::{DidNotRunReason, FailureReason, PassReason, WarningReason};

pub use std::slice::Iter;

/// A struct for interrogating *pass* results.
/// Implements `Iterator` and can be reduced to a filtered results set using its accompanying  `due_to...` methods
pub struct PassedResults<'a> {
    iter: ChainedResultsIter<'a, PassResultsCountSummary>,
}

impl<'a> PassedResults<'a> {
    pub fn from_many(
        many: Vec<(Iter<'a, ComponentRunReport>, &'a PassResultsCountSummary)>,
    ) -> Self {
        Self {
            iter: ChainedResultsIter::from_many(many),
        }
    }

    pub fn from(iter: Iter<'a, ComponentRunReport>, counts: &'a PassResultsCountSummary) -> Self {
        Self {
            iter: ChainedResultsIter::from_single(iter, counts),
        }
    }

    /// Returns `true` if there are no *pass* results.
    pub fn has_none(&self) -> bool {
        self.count() == 0
    }

    /// Returns `true` if there are any *pass* results.
    pub fn has_some(&self) -> bool {
        self.count() != 0
    }

    /// Returns the total count of *pass* results
    pub fn count(&self) -> usize {
        (&self.iter).count()
    }

    /// Returns the total count of *pass* results for a given reason.
    ///
    /// # Arguments
    ///
    /// * `reason` - The `PassReason` enum to count.
    ///
    pub fn count_due_to_reason(&self, reason: &PassReason) -> usize {
        (&self.iter).count_due_to_reason(reason)
    }

    /// Returns a iterator of only the pass results with an *accepted* reason.
    ///
    /// # Examples of tests with this result and reason:
    ///
    /// ```rust,ignore
    /// #[integration_test]
    /// fn this_test_will_fail_but_will_be_accepted() {
    ///    assert_eq!(true, true);
    /// }
    ///```
    pub fn due_to_acceptance(self) -> PassReasonResults<'a> {
        self.due_to_reason(PassReason::Accepted)
    }

    /// Returns a iterator for only the pass results which matches the give pass reason
    ///
    /// # Arguments
    ///
    /// * `reason` - The `PassReason` enum to filter by.
    ///
    pub fn due_to_reason(self, reason: PassReason) -> PassReasonResults<'a> {
        PassReasonResults {
            count: self.count_due_to_reason(&reason),
            iter: self.iter,
            filter_by_reason: reason,
        }
    }
}

impl<'a> Iterator for PassedResults<'a> {
    type Item = &'a ComponentRunReport;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.count();
        (len, Some(len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|report| match report.result {
            ComponentResult::Pass(_) => Some(report),
            _ => None,
        })
    }
}

/// A struct for iterating *pass* result reasons.
pub struct PassReasonResults<'a> {
    iter: ChainedResultsIter<'a, PassResultsCountSummary>,
    filter_by_reason: PassReason,
    count: usize,
}

impl<'a> PassReasonResults<'a> {
    pub fn has_none(&self) -> bool {
        self.count == 0
    }

    pub fn has_some(&self) -> bool {
        self.count != 0
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl<'a> Iterator for PassReasonResults<'a> {
    type Item = &'a ComponentRunReport;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let filter_by_reason = self.filter_by_reason.clone();
        self.iter.find_map(|report| match &report.result {
            ComponentResult::Pass(reason) if reason == &filter_by_reason => Some(report),
            _ => None,
        })
    }
}

/// A struct for interrogating *pass* results.
/// Implements `Iterator` and can be reduced to a filtered results set using its accompanying  `due_to...` methods
pub struct WarningResults<'a> {
    iter: ChainedResultsIter<'a, WarningResultsCountSummary>,
}

impl<'a> WarningResults<'a> {
    pub fn from_many(
        many: Vec<(Iter<'a, ComponentRunReport>, &'a WarningResultsCountSummary)>,
    ) -> Self {
        Self {
            iter: ChainedResultsIter::from_many(many),
        }
    }

    pub fn from(
        iter: Iter<'a, ComponentRunReport>,
        counts: &'a WarningResultsCountSummary,
    ) -> Self {
        Self {
            iter: ChainedResultsIter::from_single(iter, counts),
        }
    }

    /// Returns `true` if there are no *pass* results.
    pub fn has_none(&self) -> bool {
        self.count() == 0
    }

    /// Returns `true` if there are any *pass* results.
    pub fn has_some(&self) -> bool {
        self.count() != 0
    }

    /// Returns the total count of *pass* results
    pub fn count(&self) -> usize {
        (&self.iter).count()
    }

    /// Returns the total count of *warning* results for a given reason.
    ///
    /// # Arguments
    ///
    /// * `reason` - The `WarningReason` enum to count.
    ///
    pub fn count_due_to_reason(&self, reason: &WarningReason) -> usize {
        (&self.iter).count_due_to_reason(reason)
    }

    /// Returns a iterator of only the waring results with an *failure allowed* reason.
    ///
    /// # Example of test with this result and reason:
    ///
    /// ```rust,ignore
    /// #[integration_test]
    /// #[allow_fail]
    /// fn this_test_will_fail_but_will_be_accepted() {
    ///    assert_eq!(true, false);
    /// }
    ///```
    pub fn due_to_allowed_failure(self) -> WarningReasonResults<'a> {
        self.due_to_reason(WarningReason::FailureAllowed)
    }

    /// Returns a iterator of only the waring results with an *overtime warning* reason.
    ///
    /// # Example of test with this result and reason:
    ///
    /// ```rust,ignore
    /// #[integration_test]
    /// #[allow_fail]
    /// #[warning_critical_threshold_seconds(1)]
    /// fn this_test_will_be_aborted_after_1_second_but_will_still_be_accepted() {
    ///    std::thread::sleep(std::time::Duration::from_millis(1100));
    /// }
    ///```
    pub fn due_to_overtime_warning(self) -> WarningReasonResults<'a> {
        self.due_to_reason(WarningReason::OvertimeWarning)
    }

    /// Returns a iterator of only the waring results with an *overtime warning* reason.
    ///
    /// # Example of test with this result and reason:
    ///
    /// ```rust,ignore
    /// // This suite will have ChildWarning as its WarningReason
    /// #[integration_suite]
    /// mod test_suite {
    ///
    ///     #[integration_test]
    ///     #[allow_fail]
    ///     fn this_test_will_fail_but_will_be_accepted() {
    ///         assert_eq!(true, false);
    ///     }
    /// }
    ///```
    pub fn due_to_child_warning(self) -> WarningReasonResults<'a> {
        self.due_to_reason(WarningReason::ChildWarning)
    }

    /// Returns a iterator for only the warning results which matches the give warning reason
    ///
    /// # Arguments
    ///
    /// * `reason` - The `WarningReason` enum to filter by.
    ///
    pub fn due_to_reason(self, reason: WarningReason) -> WarningReasonResults<'a> {
        WarningReasonResults {
            count: self.count_due_to_reason(&reason),
            iter: self.iter,
            filter_by_reason: reason,
        }
    }
}

impl<'a> Iterator for WarningResults<'a> {
    type Item = &'a ComponentRunReport;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.count();
        (len, Some(len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|report| match report.result {
            ComponentResult::Warning(_) => Some(report),
            _ => None,
        })
    }
}

/// A struct for iterating *warning* result reasons.
pub struct WarningReasonResults<'a> {
    iter: ChainedResultsIter<'a, WarningResultsCountSummary>,
    filter_by_reason: WarningReason,
    count: usize,
}

impl<'a> WarningReasonResults<'a> {
    pub fn has_none(&self) -> bool {
        self.count == 0
    }

    pub fn has_some(&self) -> bool {
        self.count != 0
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl<'a> Iterator for WarningReasonResults<'a> {
    type Item = &'a ComponentRunReport;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let filter_by_reason = self.filter_by_reason.clone();
        self.iter.find_map(|report| match &report.result {
            ComponentResult::Warning(reason) if reason == &filter_by_reason => Some(report),
            _ => None,
        })
    }
}

/// A struct for interrogating *failed* results.
/// Implements `Iterator` and can be reduced to a filtered results set using its accompanying  `due_to...` methods
pub struct FailedResults<'a> {
    iter: ChainedResultsIter<'a, FailResultsCountSummary>,
}

impl<'a> FailedResults<'a> {
    pub fn from_many(
        many: Vec<(Iter<'a, ComponentRunReport>, &'a FailResultsCountSummary)>,
    ) -> Self {
        Self {
            iter: ChainedResultsIter::from_many(many),
        }
    }

    pub fn from(iter: Iter<'a, ComponentRunReport>, counts: &'a FailResultsCountSummary) -> Self {
        Self {
            iter: ChainedResultsIter::from_single(iter, counts),
        }
    }

    /// Returns `true` if there are no *fail* results.
    pub fn has_none(&self) -> bool {
        self.count() == 0
    }

    /// Returns `true` if there are any *fail* results.
    pub fn has_some(&self) -> bool {
        self.count() != 0
    }

    /// Returns the total count of *fail* results
    pub fn count(&self) -> usize {
        (&self.iter).count()
    }

    /// Returns the total count of *fail* results for a given reason.
    ///
    /// # Arguments
    ///
    /// * `reason` - The `FailureReason` enum to count.
    ///
    pub fn count_due_to_reason(&self, reason: &FailureReason) -> usize {
        (&self.iter).count_due_to_reason(reason)
    }

    /// Returns a iterator of only the fail results with a *rejected* reason.
    ///
    /// # Examples of tests with this result and reason:
    ///
    /// ```rust,ignore
    /// #[integration_test]
    /// fn this_test_will_fail() {
    ///    assert_eq!(true, false);
    /// }
    ///```
    ///
    pub fn due_to_rejection(self) -> FailedReasonResults<'a> {
        self.due_to_reason(FailureReason::Rejected)
    }

    /// Returns a iterator of only the fail results with an *overtime* reason.
    ///
    /// # Examples of tests with this result and reason:
    ///
    /// ```rust,ignore
    /// #[integration_test]
    /// #[critical_threshold_seconds(1)]
    /// fn this_test_will_be_aborted_after_1_second() {
    ///    std::thread::sleep(std::time::Duration::from_millis(1100));
    /// }
    ///```
    pub fn due_to_timing_out(self) -> FailedReasonResults<'a> {
        self.due_to_reason(FailureReason::Overtime)
    }

    /// Returns a iterator for only the pass results which matches the give fail reason
    ///
    /// # Arguments
    ///
    /// * `reason` - The `FailureReason` enum to filter by.
    ///
    pub fn due_to_reason(self, reason: FailureReason) -> FailedReasonResults<'a> {
        FailedReasonResults {
            count: self.count_due_to_reason(&reason),
            iter: self.iter,
            filter_by_reason: reason,
        }
    }
}

impl<'a> Iterator for FailedResults<'a> {
    type Item = &'a ComponentRunReport;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.count();
        (count, Some(count))
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|report| match report.result {
            ComponentResult::Fail(_) => Some(report),
            _ => None,
        })
    }
}

/// A struct for iterating *failed* result reasons.
pub struct FailedReasonResults<'a> {
    iter: ChainedResultsIter<'a, FailResultsCountSummary>,
    filter_by_reason: FailureReason,
    count: usize,
}

impl<'a> FailedReasonResults<'a> {
    pub fn has_none(&self) -> bool {
        self.count == 0
    }

    pub fn has_some(&self) -> bool {
        self.count != 0
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl<'a> Iterator for FailedReasonResults<'a> {
    type Item = &'a ComponentRunReport;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let filter_by_reason = self.filter_by_reason.clone();
        self.iter.find_map(|report| match &report.result {
            ComponentResult::Fail(reason) if reason == &filter_by_reason => Some(report),
            _ => None,
        })
    }
}

// NotRun Results Iterators

/// A struct for interrogating *not run* results.
/// Implements `Iterator` and can be reduced to a filtered results set using its accompanying  `due_to...` methods
pub struct NotRunResults<'a> {
    iter: ChainedResultsIter<'a, DidNotRunResultsCountSummary>,
}

impl<'a> NotRunResults<'a> {
    pub fn from_many(
        many: Vec<(
            Iter<'a, ComponentRunReport>,
            &'a DidNotRunResultsCountSummary,
        )>,
    ) -> Self {
        Self {
            iter: ChainedResultsIter::from_many(many),
        }
    }

    pub fn from(
        iter: Iter<'a, ComponentRunReport>,
        counts: &'a DidNotRunResultsCountSummary,
    ) -> Self {
        Self {
            iter: ChainedResultsIter::from_single(iter, counts),
        }
    }

    /// Returns `true` if there are no *not run* results.
    pub fn has_none(&self) -> bool {
        self.count() == 0
    }

    /// Returns `true` if there are any *not run* results.
    pub fn has_some(&self) -> bool {
        self.count() != 0
    }

    /// Returns the total count of *not run* results
    pub fn count(&self) -> usize {
        (&self.iter).count()
    }

    /// Returns the total count of *not run* results for a given reason.
    ///
    /// # Arguments
    ///
    /// * `reason` - The `DidNotRunReason` enum to count.
    ///
    pub fn count_due_to_reason(&self, reason: &DidNotRunReason) -> usize {
        (&self.iter).count_due_to_reason(reason)
    }

    /// Returns a iterator of only the not run results with an *filtered* reason.
    /// Tests which are filter is determined by the command line parameters given
    /// at the time of execution
    pub fn due_to_filtered(self) -> NotRunReasonResults<'a> {
        self.due_to_reason(DidNotRunReason::Filtered)
    }

    /// Returns a iterator of only the not run results with an *ignored* reason.
    ///
    /// # Examples of tests with this result and reason:
    ///
    /// ```rust,ignore
    /// #[integration_test]
    /// #[ignore]
    /// fn this_test_will_never_be_run() {
    ///    
    /// }
    ///```
    pub fn due_to_ignored(self) -> NotRunReasonResults<'a> {
        self.due_to_reason(DidNotRunReason::Ignored)
    }

    /// Returns a iterator of only the not run results with a *undetermined* reason.
    /// This result should never be returned in normal operation and will only be assigned
    /// if a component was scheduled but never run.
    pub fn due_to_undetermined(self) -> NotRunReasonResults<'a> {
        self.due_to_reason(DidNotRunReason::Undetermined)
    }

    /// Returns a iterator for only the pass results which matches the give not run reason
    ///
    /// # Arguments
    ///
    /// * `reason` - The `DidNotRunReason` enum to filter by.
    ///
    pub fn due_to_reason(self, reason: DidNotRunReason) -> NotRunReasonResults<'a> {
        NotRunReasonResults {
            count: self.count_due_to_reason(&reason),
            iter: self.iter,
            filter_by_reason: reason,
        }
    }
}

impl<'a> Iterator for NotRunResults<'a> {
    type Item = &'a ComponentRunReport;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.count();
        (count, Some(count))
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|report| match report.result {
            ComponentResult::DidNotRun(_) => Some(report),
            _ => None,
        })
    }
}

/// A struct for iterating *not run* result reasons.
pub struct NotRunReasonResults<'a> {
    iter: ChainedResultsIter<'a, DidNotRunResultsCountSummary>,
    filter_by_reason: DidNotRunReason,
    count: usize,
}

impl<'a> NotRunReasonResults<'a> {
    pub fn has_none(&self) -> bool {
        self.count == 0
    }

    pub fn has_some(&self) -> bool {
        self.count != 0
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl<'a> Iterator for NotRunReasonResults<'a> {
    type Item = &'a ComponentRunReport;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let filter_by_reason = self.filter_by_reason.clone();
        self.iter.find_map(|report| match &report.result {
            ComponentResult::DidNotRun(reason) if reason == &filter_by_reason => Some(report),
            _ => None,
        })
    }
}

/// Iterator for aggregating multiple results into a single iteration.
struct ChainedResultsIter<'a, ResultsCountSummary> {
    iters: Vec<(Iter<'a, ComponentRunReport>, &'a ResultsCountSummary)>,
}

impl<'a, ResultsCountSummary: ResultReasonCounter> ChainedResultsIter<'a, ResultsCountSummary> {
    pub fn from_many(many: Vec<(Iter<'a, ComponentRunReport>, &'a ResultsCountSummary)>) -> Self {
        Self { iters: many }
    }

    pub fn from_single(
        iter: Iter<'a, ComponentRunReport>,
        result: &'a ResultsCountSummary,
    ) -> Self {
        Self {
            iters: vec![(iter, result)],
        }
    }

    pub fn count(&self) -> usize {
        self.iters
            .iter()
            .fold(0, |acc, (_, counts)| counts.total() + acc)
    }

    pub fn count_due_to_reason(
        &self,
        reason: &<ResultsCountSummary as ResultReasonCounter>::ReasonType,
    ) -> usize {
        self.iters
            .iter()
            .fold(0, |acc, (_, counts)| counts.by_reason(&reason) + acc)
    }
}

impl<'a, ResultsCountSummary> Iterator for ChainedResultsIter<'a, ResultsCountSummary> {
    type Item = &'a ComponentRunReport;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((iter, _)) = self.iters.first_mut() {
            match iter.next() {
                Some(val) => {
                    return Some(val);
                }
                None => {
                    self.iters.pop();
                }
            }
        }
        return None;
    }
}
