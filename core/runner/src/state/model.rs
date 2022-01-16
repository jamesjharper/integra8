use integra8_results::ComponentResult;
use std::time::Duration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentState {
    Undetermined,
    Tentative(ComponentResult),
    Finalized(ComponentResult),
}

impl ComponentState {
    pub fn is_undetermined(&self) -> bool {
        match self {
            Self::Undetermined => true,
            _ => false,
        }
    }

    pub fn is_failed(&self) -> bool {
        self.result().map(|r| r.has_failed()).unwrap_or(false)
    }

    pub fn is_success(&self) -> bool {
        self.result().map(|r| r.has_passed()).unwrap_or(false)
    }

    pub fn is_warn(&self) -> bool {
        self.result().map(|r| r.has_warn()).unwrap_or(false)
    }

    pub fn is_skipped(&self) -> bool {
        self.result().map(|r| r.has_not_run()).unwrap_or(false)
    }

    pub fn result(&self) -> Option<ComponentResult> {
        match self {
            Self::Tentative(r) | Self::Finalized(r) => Some(r.clone()),
            _ => None,
        }
    }
}

pub struct ComponentResultsModel {
    pub state: ComponentState,
    pub time_taken: Duration,
}

impl ComponentResultsModel {
    pub fn undetermined_state() -> Self {
        ComponentResultsModel {
            state: ComponentState::Undetermined,
            time_taken: Duration::new(0, 0),
        }
    }
}
