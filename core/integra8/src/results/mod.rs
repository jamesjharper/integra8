use std::time::Duration;

pub mod artifacts;
pub mod report;
pub mod summary;

#[cfg(feature = "enable_serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WarningReason {
    FailureAllowed,
    OvertimeWarning,
    ChildWarning,
}

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PassReason {
    Accepted,
}

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FailureReason {
    Rejected,
    Overtime,
    ChildFailure,
}

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DidNotRunReason {
    Ignored,
    Filtered,
    ParentFailure,
    Undetermined,
}

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentResult {
    Pass(PassReason),
    Warning(WarningReason),
    Fail(FailureReason),
    DidNotRun(DidNotRunReason),
}

impl ComponentResult {
    pub fn passed() -> Self {
        Self::Pass(PassReason::Accepted)
    }

    pub fn rejection_exempt() -> Self {
        Self::Warning(WarningReason::FailureAllowed)
    }

    pub fn child_failure() -> Self {
        Self::Fail(FailureReason::ChildFailure)
    }

    pub fn child_warning() -> Self {
        Self::Warning(WarningReason::ChildWarning)
    }

    pub fn time_out_warning() -> Self {
        Self::Warning(WarningReason::OvertimeWarning)
    }

    pub fn timed_out() -> Self {
        Self::Fail(FailureReason::Overtime)
    }

    pub fn rejected() -> Self {
        Self::Fail(FailureReason::Rejected)
    }

    pub fn ignored() -> Self {
        Self::DidNotRun(DidNotRunReason::Ignored)
    }

    pub fn filtered() -> Self {
        Self::DidNotRun(DidNotRunReason::Filtered)
    }

    pub fn parent_failure() -> Self {
        Self::DidNotRun(DidNotRunReason::ParentFailure)
    }

    pub fn undetermined() -> Self {
        Self::DidNotRun(DidNotRunReason::Undetermined)
    }

    pub fn has_failed(&self) -> bool {
        match self {
            Self::Fail(_) => true,
            _ => false,
        }
    }

    pub fn has_passed(&self) -> bool {
        match self {
            Self::Pass(_) => true,
            _ => false,
        }
    }

    pub fn has_warn(&self) -> bool {
        match self {
            Self::Warning(_) => true,
            _ => false,
        }
    }

    pub fn has_not_run(&self) -> bool {
        match self {
            Self::DidNotRun(_) => true,
            _ => false,
        }
    }

    pub fn to_status_code(&self) -> i32 {
        match self {
            ComponentResult::Pass(PassReason::Accepted) => 0,
            // Any unexpected panic will return => 1,
            ComponentResult::Warning(WarningReason::FailureAllowed) => 2,
            ComponentResult::Warning(WarningReason::OvertimeWarning) => 3,
            ComponentResult::Warning(WarningReason::ChildWarning) => 4,

            ComponentResult::Fail(FailureReason::ChildFailure) => 10,
            ComponentResult::Fail(FailureReason::Rejected) => 11,
            ComponentResult::Fail(FailureReason::Overtime) => 12,

            ComponentResult::DidNotRun(DidNotRunReason::Undetermined) => 20,
            ComponentResult::DidNotRun(DidNotRunReason::Filtered) => 21,
            ComponentResult::DidNotRun(DidNotRunReason::Ignored) => 22,
            ComponentResult::DidNotRun(DidNotRunReason::ParentFailure) => 23,
        }
    }

    pub fn from_status_code(status_code: i32) -> Self {
        match status_code {
            0 => ComponentResult::Pass(PassReason::Accepted),
            //1 => Any unexpected internal panic will return 1,
            2 => ComponentResult::Warning(WarningReason::FailureAllowed),
            3 => ComponentResult::Warning(WarningReason::OvertimeWarning),
            4 => ComponentResult::Warning(WarningReason::ChildWarning),

            10 => ComponentResult::Fail(FailureReason::ChildFailure),
            11 => ComponentResult::Fail(FailureReason::Rejected),
            13 => ComponentResult::Fail(FailureReason::Overtime),

            20 => ComponentResult::DidNotRun(DidNotRunReason::Undetermined),
            21 => ComponentResult::DidNotRun(DidNotRunReason::Filtered),
            22 => ComponentResult::DidNotRun(DidNotRunReason::Ignored),
            23 => ComponentResult::DidNotRun(DidNotRunReason::ParentFailure),
            _ => ComponentResult::DidNotRun(DidNotRunReason::Undetermined),
        }
    }
}

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub struct ComponentTimeResult {
    pub time_taken: Duration,
    #[cfg_attr(
        feature = "enable_serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub warning_time_limit: Option<Duration>,
    #[cfg_attr(
        feature = "enable_serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub time_limit: Option<Duration>,
}

impl ComponentTimeResult {
    pub fn zero() -> Self {
        Self {
            time_taken: Duration::from_secs(0),
            warning_time_limit: None,
            time_limit: None,
        }
    }

    pub fn from_time(t: Duration) -> Self {
        Self {
            time_taken: t,
            warning_time_limit: None,
            time_limit: None,
        }
    }

    pub fn new(
        t: Duration,
        warning_time_limit: Option<Duration>,
        time_limit: Option<Duration>,
    ) -> Self {
        Self {
            time_taken: t,
            warning_time_limit: warning_time_limit,
            time_limit: time_limit,
        }
    }

    pub fn is_warn(&self) -> bool {
        match self.warning_time_limit {
            Some(warning_time_limit) => warning_time_limit < self.duration(),
            None => false,
        }
    }

    pub fn is_critical(&self) -> bool {
        match self.time_limit {
            Some(time_limit) => time_limit < self.duration(),
            None => false,
        }
    }

    pub fn duration(&self) -> Duration {
        self.time_taken
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::report::ComponentReportBuilder;
    use crate::results::summary::RunSummary;
    use crate::components::{
        AcceptanceCriteria, ComponentDescription, ComponentId, ComponentLocation, ComponentPath,
        ComponentType, ExecutionArtifacts, TimingAcceptanceCriteria,
    };

    // Component Report Tests

    fn root_suite_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("root"),
                /* id */ ComponentId::from(1),
                /* parent_id */ ComponentId::from(1),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from("main.rs"),
                    column: 0,
                    line: 0,
                    path: ComponentPath::from("crate::results"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from("main.rs"),
                    column: 0,
                    line: 0,
                    path: ComponentPath::from("crate::results"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Suite,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn test_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("test_1"),
                /* id */ ComponentId::from(2),
                /* parent_id */ ComponentId::from(1),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::test_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from("main.rs"),
                    column: 0,
                    line: 0,
                    path: ComponentPath::from("crate::results"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Test,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn setup_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("setup_1"),
                /* id */ ComponentId::from(3),
                /* parent_id */ ComponentId::from(1),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::setup_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from("main.rs"),
                    column: 0,
                    line: 0,
                    path: ComponentPath::from("crate::results"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Setup,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn tear_down_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("tear_down_1"),
                /* id */ ComponentId::from(4),
                /* parent_id */ ComponentId::from(1),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::tear_down_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from("main.rs"),
                    column: 0,
                    line: 0,
                    path: ComponentPath::from("crate::results"),
                },
                /* description */ None,
                /* component_type */ ComponentType::TearDown,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn test_2_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("test_2"),
                /* id */ ComponentId::from(5),
                /* parent_id */ ComponentId::from(1),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::test_2"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from("main.rs"),
                    column: 0,
                    line: 0,
                    path: ComponentPath::from("crate::results"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Test,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: Some(Duration::from_secs(1000)),
                    time_limit: Some(Duration::from_secs(2000)),
                },
            },
        )
    }

    fn test_3_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("test_3"),
                /* id */ ComponentId::from(6),
                /* parent_id */ ComponentId::from(1),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::test_3"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from("main.rs"),
                    column: 0,
                    line: 0,
                    path: ComponentPath::from("crate::results"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Test,
            ),
            AcceptanceCriteria {
                allowed_fail: true,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: Some(Duration::from_secs(1000)),
                    time_limit: Some(Duration::from_secs(2000)),
                },
            },
        )
    }

    fn suite_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("suite_1"),
                /* id */ ComponentId::from(7),
                /* parent_id */ ComponentId::from(1),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: 1,
                    line: 369,
                    path: ComponentPath::from("crate::results::test::suite_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from("main.rs"),
                    column: 0,
                    line: 0,
                    path: ComponentPath::from("crate::results"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Suite,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn suite_1_test_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("test_1"),
                /* id */ ComponentId::from(8),
                /* parent_id */ ComponentId::from(7),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::suite_1::test_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: 1,
                    line: 369,
                    path: ComponentPath::from("crate::results::test::suite_1"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Test,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn suite_1_setup_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("setup_1"),
                /* id */ ComponentId::from(9),
                /* parent_id */ ComponentId::from(7),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::suite_1::setup_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: 1,
                    line: 369,
                    path: ComponentPath::from("crate::results::test::suite_1"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Setup,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn suite_1_tear_down_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("tear_down_1"),
                /* id */ ComponentId::from(10),
                /* parent_id */ ComponentId::from(7),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::suite_1::tear_down_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: 1,
                    line: 369,
                    path: ComponentPath::from("crate::results::test::suite_1"),
                },
                /* description */ None,
                /* component_type */ ComponentType::TearDown,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn suite_2_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("suite_2"),
                /* id */ ComponentId::from(11),
                /* parent_id */ ComponentId::from(1),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: 1,
                    line: 516,
                    path: ComponentPath::from("crate::results::test::suite_2"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from("main.rs"),
                    column: 0,
                    line: 0,
                    path: ComponentPath::from("crate::results"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Suite,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn suite_2_test_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("test_1"),
                /* id */ ComponentId::from(12),
                /* parent_id */ ComponentId::from(11),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::suite_2::test_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: 1,
                    line: 516,
                    path: ComponentPath::from("crate::results::test::suite_2"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Test,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn suite_2_setup_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("setup_1"),
                /* id */ ComponentId::from(13),
                /* parent_id */ ComponentId::from(11),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::suite_2::setup_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: 1,
                    line: 516,
                    path: ComponentPath::from("crate::results::test::suite_2"),
                },
                /* description */ None,
                /* component_type */ ComponentType::Setup,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    fn suite_2_tear_down_1_report_builder() -> ComponentReportBuilder {
        ComponentReportBuilder::new(
            ComponentDescription::new(
                /* name */ Some("tear_down_1"),
                /* id */ ComponentId::from(14),
                /* parent_id */ ComponentId::from(11),
                /* location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: column!(),
                    line: line!(),
                    path: ComponentPath::from("crate::results::test::suite_2::tear_down_1"),
                },
                /* parent_location */
                ComponentLocation {
                    file_name: std::borrow::Cow::from(file!()),
                    column: 1,
                    line: 516,
                    path: ComponentPath::from("crate::results::test::suite_2"),
                },
                /* description */ None,
                /* component_type */ ComponentType::TearDown,
            ),
            AcceptanceCriteria {
                allowed_fail: false,
                timing: TimingAcceptanceCriteria {
                    warning_time_limit: None,
                    time_limit: None,
                },
            },
        )
    }

    #[test]
    fn can_report_test_1_success() {
        // Arrange
        let mut builder = test_1_report_builder();

        // Act
        builder.passed_result();
        let report = builder.build();

        // Assert
        assert!(report.result.has_passed(), "Expected Test to have passed");
        assert_eq!(
            report.result,
            ComponentResult::Pass(PassReason::Accepted),
            "Expected Test to have passed, with PassReason::Accepted"
        );
    }

    #[test]
    fn can_report_test_3_success() {
        // Arrange
        // Test 3 has failure allowed flag enabled, which shouldn't
        // prevent its reporting as passed if it does in fact pass
        let mut builder = test_3_report_builder();

        // Act
        builder.passed_result();
        let report = builder.build();

        // Assert
        assert!(report.result.has_passed(), "Expected Test to have passed");
        assert_eq!(
            report.result,
            ComponentResult::Pass(PassReason::Accepted),
            "Expected Test to have passed, with PassReason::Accepted"
        );
    }

    #[test]
    fn should_report_test_3_warning_when_failed() {
        // Arrange
        // Test 3 has failure allowed flag enabled
        let mut builder = test_3_report_builder();

        // Act
        builder.rejected_result();
        let report = builder.build();

        // Assert
        assert!(report.result.has_warn(), "Expected Test to have warning");
        assert_eq!(
            report.result,
            ComponentResult::Warning(WarningReason::FailureAllowed),
            "Expected Test to have warning, with WarningReason::FailureAllowed"
        );
    }

    #[test]
    fn can_report_test_1_failure() {
        // Arrange
        let mut builder = test_1_report_builder();

        // Act
        builder.rejected_result();
        let report = builder.build();

        // Assert
        assert!(report.result.has_failed(), "Expected Test to have failed");
        assert_eq!(
            report.result,
            ComponentResult::Fail(FailureReason::Rejected),
            "Expected Test to have failed, with FailureReason::Rejected"
        );
    }

    #[test]
    fn can_report_test_1_failed_due_to_child_failure() {
        // Arrange
        let mut builder = test_1_report_builder();

        // Act
        builder.with_result(ComponentResult::child_failure());
        let report = builder.build();

        // Assert
        assert!(report.result.has_failed(), "Expected Test to have failed");
        assert_eq!(
            report.result,
            ComponentResult::Fail(FailureReason::ChildFailure),
            "Expected Test to have failed, with FailureReason::ChildFailure"
        );
    }

    #[test]
    fn can_report_test_1_ignored() {
        // Arrange
        let mut builder = test_1_report_builder();

        // Act
        builder.ignored_result();
        let report = builder.build();

        // Assert
        assert!(
            report.result.has_not_run(),
            "Expected Test to have not be run"
        );
        assert_eq!(
            report.result,
            ComponentResult::DidNotRun(DidNotRunReason::Ignored),
            "Expected Test to have failed, with DidNotRunReason::Ignored"
        );
    }

    #[test]
    fn can_report_test_1_undetermined() {
        // Arrange
        let mut builder = test_1_report_builder();

        // Act
        builder.with_result(ComponentResult::undetermined());
        let report = builder.build();

        // Assert
        assert!(
            report.result.has_not_run(),
            "Expected Test to have not be run"
        );
        assert_eq!(
            report.result,
            ComponentResult::DidNotRun(DidNotRunReason::Undetermined),
            "Expected Test to have failed, with DidNotRunReason::Undetermined"
        );
    }

    #[test]
    fn can_report_test_1_filtered_out() {
        // Arrange
        let mut builder = test_1_report_builder();

        // Act
        builder.filtered_result();
        let report = builder.build();

        // Assert
        assert!(
            report.result.has_not_run(),
            "Expected Test to have not be run"
        );
        assert_eq!(
            report.result,
            ComponentResult::DidNotRun(DidNotRunReason::Filtered),
            "Expected Test to have failed, with DidNotRunReason::Undetermined"
        );
    }

    #[test]
    fn can_report_test_1_ignored_due_to_parent_failure() {
        // Arrange
        let mut builder = test_1_report_builder();

        // Act
        builder.with_result(ComponentResult::parent_failure());
        let report = builder.build();

        // Assert
        assert!(
            report.result.has_not_run(),
            "Expected Test to have not be run"
        );
        assert_eq!(
            report.result,
            ComponentResult::DidNotRun(DidNotRunReason::ParentFailure),
            "Expected Test to have failed, with DidNotRunReason::Undetermined"
        );
    }

    // Time result behavior

    #[test]
    fn should_report_test_1_timing_results_as_ok_when_no_timing_acceptance_criteria() {
        // Arrange
        let mut builder = test_1_report_builder();

        // Act
        builder.passed_result();
        builder.time_taken(Duration::from_secs(1000));

        let report = builder.build();

        // Assert
        assert_eq!(
            report.timing.is_warn(),
            false,
            "Expected no warning on test 1 timing results"
        );
        assert_eq!(
            report.timing.is_critical(),
            false,
            "Expected no critical error on test 1 timing results"
        );
        assert_eq!(report.timing.duration(), Duration::from_secs(1000));
    }

    #[test]
    fn should_report_test_2_timing_results_as_ok_when_within_timing_acceptance_criteria() {
        // Arrange
        let mut builder = test_2_report_builder();

        // Act
        builder.passed_result();
        builder.time_taken(Duration::from_secs(500));

        let report = builder.build();

        // Assert
        assert_eq!(
            report.result,
            ComponentResult::Pass(PassReason::Accepted),
            "Expected Test to have passed, with PassReason::Accepted"
        );
        assert_eq!(
            report.timing.is_warn(),
            false,
            "Expected no warning on test 1 timing results"
        );
        assert_eq!(
            report.timing.is_critical(),
            false,
            "Expected no critical error on test 1 timing results"
        );
        assert_eq!(report.timing.duration(), Duration::from_secs(500));
    }

    #[test]
    fn should_report_test_2_timing_results_as_warn_when_exceeding_warn_timing_acceptance_criteria()
    {
        // Arrange
        let mut builder = test_2_report_builder();

        // Act
        builder.passed_result();
        builder.time_taken(Duration::from_secs(1500));

        let report = builder.build();

        // Assert
        assert_eq!(
            report.result,
            ComponentResult::Warning(WarningReason::OvertimeWarning),
            "Expected Test to have warning, with WarningReason::OvertimeWarning"
        );
        assert_eq!(
            report.timing.is_warn(),
            true,
            "Expected warning on test 2 timing results"
        );
        assert_eq!(
            report.timing.is_critical(),
            false,
            "Expected no critical error on test 2 timing results"
        );
        assert_eq!(report.timing.duration(), Duration::from_secs(1500));
    }

    #[test]
    fn should_report_test_2_timing_results_as_fail_when_exceeding_timing_acceptance_criteria() {
        // Arrange
        let mut builder = test_2_report_builder();

        // Act
        builder.passed_result();
        builder.time_taken(Duration::from_secs(2500));

        let report = builder.build();

        // Assert
        assert_eq!(
            report.result,
            ComponentResult::Fail(FailureReason::Overtime),
            "Expected Test to have warning, with WarningReason::OvertimeWarning"
        );
        assert_eq!(
            report.timing.is_warn(),
            true,
            "Expected warning on test 2 timing results"
        );
        assert_eq!(
            report.timing.is_critical(),
            true,
            "Expected critical error on test 2 timing results"
        );
        assert_eq!(report.timing.duration(), Duration::from_secs(2500));
    }

    #[test]
    fn should_report_test_2_results_as_rejected_when_rejected_and_exceeding_timing_acceptance_criteria(
    ) {
        // Because async framework doesn't always resume tasks immediately, we can have a test timeout and fail, and panic
        // at the same time. There is no one correct solution to what error should show in this case. The framework will
        // favor the panic rather then the time out, as it should in theory be more descriptive.

        // Arrange
        let mut builder = test_2_report_builder();

        // Act
        builder.rejected_result();
        builder.time_taken(Duration::from_secs(2500));

        let report = builder.build();

        // Assert
        assert_eq!(
            report.result,
            ComponentResult::Fail(FailureReason::Rejected),
            "Expected Test to have warning, with FailureReason::Rejected"
        );
        assert_eq!(
            report.timing.is_warn(),
            true,
            "Expected warning on test 2 timing results"
        );
        assert_eq!(
            report.timing.is_critical(),
            true,
            "Expected critical error on test 2 timing results"
        );
        assert_eq!(report.timing.duration(), Duration::from_secs(2500));
    }

    #[test]
    fn should_report_test_3_results_as_timed_out_when_rejected_and_exceeding_timing_acceptance_criteria(
    ) {
        // Arrange
        // Test 3 has failure allowed flag enabled
        let mut builder = test_3_report_builder();

        // Act
        builder.rejected_result();
        builder.time_taken(Duration::from_secs(2500));

        let report = builder.build();

        // Assert
        assert_eq!(
            report.result,
            ComponentResult::Fail(FailureReason::Overtime),
            "Expected Test to have warning, with WarningReason::OvertimeWarning"
        );
        assert_eq!(
            report.timing.is_warn(),
            true,
            "Expected warning on test 3 timing results"
        );
        assert_eq!(
            report.timing.is_critical(),
            true,
            "Expected critical error on test 3 timing results"
        );
        assert_eq!(report.timing.duration(), Duration::from_secs(2500));
    }

    #[test]
    fn should_return_remaining_time_to_deadline_based_on_timing_acceptance_criteria() {
        // Arrange
        let builder = test_2_report_builder();

        // Act
        let time_to_deadline = builder.time_until_deadline(Duration::from_secs(500));

        // Assert
        assert_eq!(time_to_deadline, Some(Duration::from_secs(1500)));
    }

    #[test]
    fn should_return_zero_time_to_deadline_when_exceeding_timing_acceptance_criteria() {
        // Arrange
        let builder = test_2_report_builder();

        // Act
        let time_to_deadline = builder.time_until_deadline(Duration::from_secs(10000));

        // Assert
        assert_eq!(time_to_deadline, Some(Duration::from_secs(0)));
    }

    #[test]
    fn should_return_none_as_deadline_when_no_timing_acceptance_criteria_is_defined() {
        // Arrange
        let builder = test_1_report_builder();

        // Act
        let time_to_deadline = builder.time_until_deadline(Duration::from_secs(10000));

        // Assert
        assert_eq!(time_to_deadline, None);
    }

    // Artifacts

    #[test]
    fn should_return_no_artifacts_when_none_are_defined() {
        // Arrange
        let mut builder = test_1_report_builder();

        // Act
        builder.rejected_result();
        let report = builder.build();

        // Assert
        assert!(report.artifacts.map.is_empty());
    }

    #[test]
    fn should_return_include_text_artifact_when_defined() {
        // Arrange
        let mut builder = test_1_report_builder();

        let artifacts = ExecutionArtifacts::new();
        artifacts.include_text("sample", "This is a sample artifact");

        // Act
        builder.rejected_result();
        builder.with_artifacts(&artifacts);
        let report = builder.build();

        // Assert
        assert_eq!(
            report.artifacts.map["sample"].as_string().unwrap(),
            "This is a sample artifact"
        );
    }

    #[test]
    fn should_return_include_text_buff_artifact_when_defined() {
        // Arrange
        let mut builder = test_1_report_builder();

        let artifacts = ExecutionArtifacts::new();
        artifacts.include_utf8_text_buffer("sample", b"This is a sample artifact".as_slice());

        // Act
        builder.rejected_result();
        builder.with_artifacts(&artifacts);
        let report = builder.build();

        // Assert
        assert_eq!(
            report.artifacts.map["sample"].as_string().unwrap(),
            "This is a sample artifact"
        );
    }

    #[test]
    fn should_return_include_text_cursor_artifact_when_defined() {
        use std::io::Write;

        // Arrange
        let artifacts = ExecutionArtifacts::new();
        let mut writer = artifacts.writer("sample");
        write!(writer, "This is a sample artifact").unwrap();
        drop(writer); // Drop is required for the stream to write to artifacts

        // Act
        let mut builder = test_1_report_builder();
        builder.rejected_result();
        builder.with_artifacts(&artifacts);
        let report = builder.build();

        // Assert
        assert_eq!(
            report.artifacts.map["sample"].as_string().unwrap(),
            "This is a sample artifact"
        );
    }

    #[test]
    fn should_preserve_artifact_order_defined() {
        // Arrange
        let mut builder = test_1_report_builder();

        let artifacts = ExecutionArtifacts::new();
        artifacts.include_text("sampleZ", "z");
        artifacts.include_text("2", "2");
        artifacts.include_text("sampleA", "a");
        artifacts.include_text("1", "1");

        // Act
        builder.rejected_result();
        builder.with_artifacts(&artifacts);
        let report = builder.build();

        // Assert

        let mut iter = report.artifacts.map.iter();

        let (k1, v1) = iter.next().unwrap();
        assert_eq!("sampleZ", k1);
        assert_eq!("z", v1.as_string().unwrap());

        let (k2, v2) = iter.next().unwrap();
        assert_eq!("2", k2);
        assert_eq!("2", v2.as_string().unwrap());

        let (k3, v3) = iter.next().unwrap();
        assert_eq!("sampleA", k3);
        assert_eq!("a", v3.as_string().unwrap());

        let (k4, v4) = iter.next().unwrap();
        assert_eq!("1", k4);
        assert_eq!("1", v4.as_string().unwrap());

        assert!(iter.next().is_none());
    }

    // Summary Tests

    #[test]
    fn can_create_empty_summary_report() {
        // Arrange
        let mut root_suite_builder = root_suite_report_builder();
        root_suite_builder.passed_result();

        // Act
        let mut summary = RunSummary::new();
        summary.push_report(root_suite_builder.build());

        // Assert
        assert_eq!(
            summary.run_result(),
            ComponentResult::Pass(PassReason::Accepted)
        );
        assert_eq!(summary.test_passed().count(), 0);
        assert_eq!(summary.test_warning().count(), 0);
        assert_eq!(summary.test_failed().count(), 0);
        assert_eq!(summary.test_not_run().count(), 0);

        assert_eq!(summary.setup_passed().count(), 0);
        assert_eq!(summary.setup_warning().count(), 0);
        assert_eq!(summary.setup_failed().count(), 0);
        assert_eq!(summary.setup_not_run().count(), 0);

        assert_eq!(summary.tear_down_passed().count(), 0);
        assert_eq!(summary.tear_down_warning().count(), 0);
        assert_eq!(summary.tear_down_failed().count(), 0);
        assert_eq!(summary.tear_down_not_run().count(), 0);
    }

    #[test]
    fn can_create_summary_report_with_passed_components_at_root() {
        // Arrange
        let mut root_suite_builder = root_suite_report_builder();
        let mut test_1_builder = test_1_report_builder();
        let mut setup_1_builder = setup_1_report_builder();
        let mut tear_down_1_builder = tear_down_1_report_builder();

        root_suite_builder.passed_result();
        test_1_builder.passed_result();
        setup_1_builder.passed_result();
        tear_down_1_builder.passed_result();

        // Act
        let mut summary = RunSummary::new();
        summary.push_report(root_suite_builder.build());
        summary.push_report(test_1_builder.build());
        summary.push_report(setup_1_builder.build());
        summary.push_report(tear_down_1_builder.build());

        // Assert
        assert_eq!(
            summary.run_result(),
            ComponentResult::Pass(PassReason::Accepted)
        );
        assert_eq!(summary.test_passed().count(), 1);
        assert_eq!(summary.test_warning().count(), 0);
        assert_eq!(summary.test_failed().count(), 0);
        assert_eq!(summary.test_not_run().count(), 0);

        assert_eq!(summary.setup_passed().count(), 1);
        assert_eq!(summary.setup_warning().count(), 0);
        assert_eq!(summary.setup_failed().count(), 0);
        assert_eq!(summary.setup_not_run().count(), 0);

        assert_eq!(summary.tear_down_passed().count(), 1);
        assert_eq!(summary.tear_down_warning().count(), 0);
        assert_eq!(summary.tear_down_failed().count(), 0);
        assert_eq!(summary.tear_down_not_run().count(), 0);
    }

    #[test]
    fn can_create_summary_report_with_failed_components_at_root() {
        // Arrange
        let mut root_suite_builder = root_suite_report_builder();
        let mut test_1_builder = test_1_report_builder();
        let mut setup_1_builder = setup_1_report_builder();
        let mut tear_down_1_builder = tear_down_1_report_builder();

        root_suite_builder.rejected_result();
        test_1_builder.rejected_result();
        setup_1_builder.rejected_result();
        tear_down_1_builder.rejected_result();

        // Act
        let mut summary = RunSummary::new();
        summary.push_report(root_suite_builder.build());
        summary.push_report(test_1_builder.build());
        summary.push_report(setup_1_builder.build());
        summary.push_report(tear_down_1_builder.build());

        // Assert
        assert_eq!(
            summary.run_result(),
            ComponentResult::Fail(FailureReason::Rejected)
        );
        assert_eq!(summary.test_passed().count(), 0);
        assert_eq!(summary.test_warning().count(), 0);
        assert_eq!(summary.test_failed().count(), 1);
        assert_eq!(summary.test_not_run().count(), 0);

        assert_eq!(summary.setup_passed().count(), 0);
        assert_eq!(summary.setup_warning().count(), 0);
        assert_eq!(summary.setup_failed().count(), 1);
        assert_eq!(summary.setup_not_run().count(), 0);

        assert_eq!(summary.tear_down_passed().count(), 0);
        assert_eq!(summary.tear_down_warning().count(), 0);
        assert_eq!(summary.tear_down_failed().count(), 1);
        assert_eq!(summary.tear_down_not_run().count(), 0);
    }

    #[test]
    fn can_create_summary_report_with_passed_components_at_nested_suite() {
        // Arrange
        let mut root_suite_builder = root_suite_report_builder();
        let mut suite_1_builder = suite_1_report_builder();
        let mut suite_1_test_1_builder = suite_1_test_1_report_builder();
        let mut suite_1_setup_1_builder = suite_1_setup_1_report_builder();
        let mut suite_1_tear_down_1_builder = suite_1_tear_down_1_report_builder();

        root_suite_builder.passed_result();
        suite_1_builder.passed_result();
        suite_1_test_1_builder.passed_result();
        suite_1_setup_1_builder.passed_result();
        suite_1_tear_down_1_builder.passed_result();

        // Act
        let mut summary = RunSummary::new();
        summary.push_report(root_suite_builder.build());
        summary.push_report(suite_1_test_1_builder.build());
        summary.push_report(suite_1_setup_1_builder.build());
        summary.push_report(suite_1_tear_down_1_builder.build());
        summary.push_report(suite_1_builder.build());

        // Assert
        assert_eq!(
            summary.run_result(),
            ComponentResult::Pass(PassReason::Accepted)
        );
        assert_eq!(summary.test_passed().total_count(), 1);
        assert_eq!(summary.test_warning().total_count(), 0);
        assert_eq!(summary.test_failed().total_count(), 0);
        assert_eq!(summary.test_not_run().total_count(), 0);

        assert_eq!(summary.setup_passed().total_count(), 1);
        assert_eq!(summary.setup_warning().total_count(), 0);
        assert_eq!(summary.setup_failed().total_count(), 0);
        assert_eq!(summary.setup_not_run().total_count(), 0);

        assert_eq!(summary.tear_down_passed().total_count(), 1);
        assert_eq!(summary.tear_down_warning().total_count(), 0);
        assert_eq!(summary.tear_down_failed().total_count(), 0);
        assert_eq!(summary.tear_down_not_run().total_count(), 0);
    }

    #[test]
    fn can_create_summary_report_with_failed_components_at_nested_suite() {
        // Arrange
        let mut root_suite_builder = root_suite_report_builder();
        let mut suite_1_builder = suite_1_report_builder();
        let mut suite_1_test_1_builder = suite_1_test_1_report_builder();
        let mut suite_1_setup_1_builder = suite_1_setup_1_report_builder();
        let mut suite_1_tear_down_1_builder = suite_1_tear_down_1_report_builder();

        let mut suite_2_builder = suite_2_report_builder();
        let mut suite_2_test_1_builder = suite_2_test_1_report_builder();
        let mut suite_2_setup_1_builder = suite_2_setup_1_report_builder();
        let mut suite_2_tear_down_1_builder = suite_2_tear_down_1_report_builder();

        root_suite_builder.rejected_result();
        suite_1_builder.rejected_result();
        suite_1_test_1_builder.rejected_result();
        suite_1_setup_1_builder.rejected_result();
        suite_1_tear_down_1_builder.rejected_result();

        suite_2_builder.rejected_result();
        suite_2_test_1_builder.rejected_result();
        suite_2_setup_1_builder.rejected_result();
        suite_2_tear_down_1_builder.rejected_result();

        // Act
        let mut summary = RunSummary::new();
        summary.push_report(root_suite_builder.build());

        summary.push_report(suite_1_test_1_builder.build());
        summary.push_report(suite_1_setup_1_builder.build());
        summary.push_report(suite_1_tear_down_1_builder.build());
        summary.push_report(suite_1_builder.build());

        summary.push_report(suite_2_test_1_builder.build());
        summary.push_report(suite_2_setup_1_builder.build());
        summary.push_report(suite_2_tear_down_1_builder.build());
        summary.push_report(suite_2_builder.build());

        // Assert
        assert_eq!(
            summary.run_result(),
            ComponentResult::Fail(FailureReason::Rejected)
        );
        assert_eq!(summary.test_passed().total_count(), 0);
        assert_eq!(summary.test_warning().total_count(), 0);
        assert_eq!(summary.test_failed().total_count(), 2);
        assert_eq!(summary.test_failed().due_to_rejection().total_count(), 2);
        assert_eq!(summary.test_not_run().total_count(), 0);

        assert_eq!(summary.setup_passed().total_count(), 0);
        assert_eq!(summary.setup_warning().total_count(), 0);
        assert_eq!(summary.setup_failed().total_count(), 2);
        assert_eq!(summary.setup_failed().due_to_rejection().total_count(), 2);
        assert_eq!(summary.setup_not_run().total_count(), 0);

        assert_eq!(summary.tear_down_passed().total_count(), 0);
        assert_eq!(summary.tear_down_warning().total_count(), 0);
        assert_eq!(summary.tear_down_failed().total_count(), 2);
        assert_eq!(
            summary.tear_down_failed().due_to_rejection().total_count(),
            2
        );
        assert_eq!(summary.tear_down_not_run().total_count(), 0);
    }
}
