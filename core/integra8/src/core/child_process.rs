
use std::panic::UnwindSafe;

use crate::core::TestApplicationLocator;
use crate::scheduling::ScheduledComponent;

use crate::results::ComponentResult;
use crate::components::TestParameters;
use crate::decorations::ComponentDecoration;

pub async fn run<
    TParameters: TestParameters + Clone + Sync + Send + UnwindSafe + 'static + std::fmt::Debug,
    Locator: TestApplicationLocator<TParameters> + Sync + Send + 'static,
>(
    auto_detect_components: Vec<ComponentDecoration<TParameters>>,
    parameters: TParameters,
) -> ComponentResult {

    // 1: Resolve all decorations 
    let decorations = Locator::resolve_decorations_strategy(&parameters)
        .resolve_decorations(&parameters, auto_detect_components);
  
    // 2: Build component hierarchy  
    let root_component = Locator::resolve_component_hierarchy_strategy(&parameters)
        .resolve_component_hierarchy(&parameters, decorations);

    // 3: Run component
    Locator::resolve_runner_strategy(&parameters)
        .run_component(
            parameters, 
            ScheduledComponent::from(root_component)
        )
        .await
        .result
}

