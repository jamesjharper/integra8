use std::sync::{Arc, RwLock};

mod model;
pub use model::{ComponentResultsModel, ComponentState};

use integra8_components::{ComponentDescription, ComponentPath, ComponentType};
use integra8_results::ComponentResult;
use std::collections::HashMap;
use std::time::Duration;

pub struct RunStateModel {
    component_result_states: HashMap<ComponentPath, Arc<RwLock<ComponentResultsModel>>>,
}

impl RunStateModel {
    pub fn new() -> Self {
        Self {
            component_result_states: HashMap::new(),
        }
    }

    /// Returns a global state token for the given component
    ///
    /// # Arguments
    ///
    /// * `description` - The description of the component for which the returned token will represent
    ///
    pub fn get_status_token(&mut self, description: &ComponentDescription) -> ComponentStateToken {
        ComponentStateToken {
            component_type: description.component_type.clone(),
            self_token: self.get_token(&description.path),
            parent_token: self.get_token(&description.parent_path),
        }
    }

    fn get_token(&mut self, path: &ComponentPath) -> Arc<RwLock<ComponentResultsModel>> {
        self.component_result_states
            .entry(path.clone())
            .or_insert(Arc::new(RwLock::new(
                ComponentResultsModel::undetermined_state(),
            )))
            .clone()
    }
}

#[derive(Clone)]
pub struct ComponentStateToken {
    component_type: ComponentType,
    self_token: Arc<RwLock<ComponentResultsModel>>,
    parent_token: Arc<RwLock<ComponentResultsModel>>,
}

impl ComponentStateToken {
    /// Returns the total execution time of this component
    pub fn time_taken(&self) -> Duration {
        self.self_token.read().unwrap().time_taken.clone()
    }

    /// Returns the type of component this token belongs to
    pub fn component_type(&self) -> ComponentType {
        self.component_type.clone()
    }

    /// Returns the current global state of this component
    /// If the state has yet to be be determined, then, it my be inherited from its parent component as per the table bellow
    ///
    /// | State                 | Parent State       | Inferred State                                             |
    /// |-----------------------|--------------------|------------------------------------------------------------|
    /// | Pass, Fail or Skipped | Any                | Pass, Fail or Skipped respectively                         |
    /// | Undetermined          | Skipped            | Skipped                                                    |
    /// | Undetermined          | Failed             | Failed, or Undetermined if component type  is tear down    |
    /// | Undetermined          | Undetermined       | Undetermined                                               |
    ///
    /// *Tear down is always run, even if the parent is in a failed state. This is to ensure to the best our abilities a clean environment after running the test.*
    ///
    pub fn state(&self) -> ComponentState {
        match self.self_token.read().unwrap().state.clone() {
            ComponentState::Undetermined => {
                let state_parent = &self.parent_token.read().unwrap().state;

                if state_parent.is_skipped() {
                    return ComponentState::Tentative(state_parent.result().unwrap());
                }

                if state_parent.is_failed() && !self.component_type.is_tear_down() {
                    return ComponentState::Tentative(ComponentResult::parent_failure());
                }

                return ComponentState::Undetermined;
            }
            other => other,
        }
    }

    /// Finalizes the global published status for this component and propagates the status value to this components parent.
    ///
    /// Parents state is determined by the following rules:
    /// - If a child has pass result, the parents own status is also passed.
    /// - If **any** children are failed, the parents own status is also failed with the reason `child_failure`
    /// - If **all** children are skipped, the parents status is also skipped.   
    ///
    /// # Arguments
    ///
    /// * `result` - The result of this components execution
    ///
    /// * `time_taken` - The execution time of this component. Can be zero if the component was skipped.
    ///
    pub fn finalize_result(&self, result: ComponentResult, time_taken: Duration) {
        self.set_result(ComponentState::Finalized(result), time_taken)
    }

    fn set_result(&self, result: ComponentState, time_taken: Duration) {
        // Set child result
        let mut child_model = self.self_token.write().unwrap();

        child_model.state = result;
        child_model.time_taken = match self.component_type {
            // tear down and setup does not contribute
            // to their parents suites test time
            ComponentType::Setup | ComponentType::TearDown => Duration::new(0, 0),
            _ => time_taken,
        };

        if self.is_root() {
            return;
        }

        // Update child's parent result if needed
        let mut parent_model = self.parent_token.write().unwrap();
        parent_model.time_taken += child_model.time_taken;

        if child_model.state.is_failed() {
            // If the child has failed, then we implicitly failed
            parent_model.state = ComponentState::Tentative(ComponentResult::child_failure());
            return;
        }

        if child_model.state.is_success() && !parent_model.state.is_failed() {
            // If no children failed, the then we implicitly succeeded
            parent_model.state = ComponentState::Tentative(ComponentResult::passed());
            return;
        }

        if child_model.state.is_skipped() && parent_model.state.is_undetermined() {
            // if all our children are skipped, then we are implicitly skipped
            parent_model.state = child_model.state.clone();
        }
    }

    /// Returns if the component is root or not.
    /// *Root component will always have its self as its parent*
    fn is_root(&self) -> bool {
        Arc::ptr_eq(&self.self_token, &self.parent_token)
    }
}
