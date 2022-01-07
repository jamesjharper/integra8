use std::time::Duration;

use crate::{BookEndAttributes, SuiteAttributes, TestAttributes};

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
    pub warn_threshold: Option<Duration>,
    pub critical_threshold: Option<Duration>,
}

impl TimingAcceptanceCriteria {
    pub fn for_test(attributes: &TestAttributes) -> Self {
        Self {
            warn_threshold: Some(attributes.warn_threshold.clone()),
            critical_threshold: Some(attributes.critical_threshold.clone()),
        }
    }

    pub fn for_bookend(attributes: &BookEndAttributes) -> Self {
        Self {
            warn_threshold: attributes.critical_threshold.clone(),
            critical_threshold: attributes.critical_threshold.clone(),
        }
    }

    pub fn for_suite(_attributes: &SuiteAttributes) -> Self {
        Self {
            warn_threshold: None,
            critical_threshold: None,
        }
    }
}
