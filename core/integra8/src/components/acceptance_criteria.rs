use std::time::Duration;

use crate::components::{BookEndAttributes, SuiteAttributes, TestAttributes};

pub struct AcceptanceCriteria {
    pub allowed_fail: bool,
    pub timing: TimingAcceptanceCriteria,
}

impl AcceptanceCriteria {
    pub fn for_test(attributes: &TestAttributes) -> Self {
        Self {
            timing: TimingAcceptanceCriteria::for_test(attributes),
            allowed_fail: attributes.allow_fail,
        }
    }

    pub fn for_bookend(attributes: &BookEndAttributes) -> Self {
        Self {
            timing: TimingAcceptanceCriteria::for_bookend(attributes),
            allowed_fail: false,
        }
    }

    pub fn for_suite(attributes: &SuiteAttributes) -> Self {
        Self {
            timing: TimingAcceptanceCriteria::for_suite(attributes),
            allowed_fail: attributes.allow_suite_fail,
        }
    }
}

pub struct TimingAcceptanceCriteria {
    pub warning_time_limit: Option<Duration>,
    pub time_limit: Option<Duration>,
}

impl TimingAcceptanceCriteria {
    pub fn for_test(attributes: &TestAttributes) -> Self {
        Self {
            warning_time_limit: Some(attributes.warning_time_limit.clone()),
            time_limit: Some(attributes.time_limit.clone()),
        }
    }

    pub fn for_bookend(attributes: &BookEndAttributes) -> Self {
        Self {
            warning_time_limit: Some(attributes.time_limit.clone()),
            time_limit: Some(attributes.time_limit.clone()),
        }
    }

    pub fn for_suite(_attributes: &SuiteAttributes) -> Self {
        Self {
            warning_time_limit: None,
            time_limit: None,
        }
    }
}
