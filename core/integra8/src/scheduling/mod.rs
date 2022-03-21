pub mod iter;

pub mod state_machine;
pub use state_machine::{PollTaskResult, TaskNodePath, TaskStateMachineNode, TaskStream};

pub mod components;
pub use components::{IntoTaskStateMachine, ScheduledComponent};

mod scheduler;
pub use scheduler::TaskScheduler;

//use num_cpus;

pub fn recommended_max_concurrency() -> usize {
    std::cmp::max(1, num_cpus::get().saturating_sub(1))
}
/*
### example:

```
 ● root
 ├── ● parallel tests (run at same time)
 │   ├── ■ test 3
 │   └── ■ test 4
 │
 ├── ● Sequential tests (run in order)
 │   ├── ■ test 1
 │   └── ■ test 2
 │
 ├── ● parallel suites (run at same time)
 │   ├── ● Suite 1
 │   │   └── ... recursive behavior
 │   └── ● Suite 2
 │       └── ... recursive behavior
 ├── ● Sequential suites (run in order)
 │   ├── ● Suite 3
 │   │   └── ... recursive behavior
 │   └── ● Suite 4
 │       └── ... recursive behavior

```
*/
