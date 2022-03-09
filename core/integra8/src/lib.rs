pub mod core;
pub mod strategy;

pub use integra8_impl::*;

pub mod formatters {
    pub use integra8_formatters::*;
}

pub mod results {
    pub use integra8_results::*;
}

pub mod scheduling {
    pub use integra8_scheduling::*;
}

pub mod decorations {
    pub use integra8_decorations::*;
}

pub use integra8_decorations_impl::*;

pub mod components {
    pub use integra8_components::*;
}

pub mod runner {
    pub use integra8_runner::*;
}

pub mod async_runtime {
    pub use integra8_async_runtime::*;
}

#[doc(hidden)]
pub mod linkme {
    pub use linkme::*;
}

#[doc(hidden)]
pub mod structopt {
    pub use structopt::*;
}

pub mod humantime {
    pub use humantime::parse_duration;
}

#[macro_export]
macro_rules! run_tests {
    ($parameters:expr) => {
        $crate::core::run_test(
            $parameters,
            REGISTERED_COMPONENTS.into_iter().map(|f| (f)()).collect(),
        )
        .await
    };
}

#[cfg(test)]
#[derive(Clone, Debug, crate::structopt::StructOpt)]
#[structopt()]
pub struct MockParameters {}

#[cfg(test)]
type Parameters = MockParameters;

#[cfg(test)]
#[linkme::distributed_slice]
pub static REGISTERED_COMPONENTS: [fn() -> crate::decorations::ComponentDecoration<Parameters>] =
    [..];
