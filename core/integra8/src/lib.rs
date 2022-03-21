#[cfg(all(feature = "async-std-runtime", feature = "tokio-runtime"))]
compile_error!(
"feature \"tokio-runtime\" and feature \"async-std-runtime\" cannot be enabled at the same time!
To configure `tokio` use `integra8 = {version = \"{VERSION}\", features = [\"core\", \"tokio-runtime\"], default-features = false }`
To configure `async-std` use `integra8 = {version = \"{VERSION}\", features = [\"core\", \"async-std-runtime\"], default-features = false }`
Otherwise using `integra8 = {version = \"{VERSION}\" }` will enable tokio by default"
);

#[cfg(all(not(feature = "async-std-runtime"), not(feature = "tokio-runtime")))]
compile_error!(
"No async runtime configured!
To configure `tokio` use `integra8 = {version = \"{VERSION}\", features = [\"core\", \"tokio-runtime\"], default-features = false }`
To configure `async-std` use `integra8 = {version = \"{VERSION}\", features = [\"core\", \"async-std-runtime\"], default-features = false }`
Otherwise using `integra8 = {version = \"{VERSION}\" }` will enable tokio by default "
);


#[cfg(feature = "core")]
pub mod strategy;

#[cfg(feature = "core")]
pub mod macros;

#[cfg(feature = "core")]
pub mod core;

#[cfg(feature = "formatters")]
pub mod formatters;

#[cfg(feature = "results")]
pub mod results;

#[cfg(feature = "scheduling")]
pub mod scheduling;

#[cfg(feature = "decorations")]
pub mod decorations;

#[cfg(feature = "components")]
pub mod components;

#[cfg(feature = "runner")]
pub mod runner;

#[cfg(feature = "async_runtime")]
pub mod async_runtime;

pub use integra8_impl::*;
pub use integra8_decorations_impl::*;

#[cfg(feature = "linkme")]
#[doc(hidden)]
pub mod linkme {
    pub use linkme::*;
}

#[cfg(feature = "structopt")]
#[doc(hidden)]
pub mod structopt {
    pub use structopt::*;
}

#[cfg(feature = "humantime")]
pub mod humantime {
    pub use humantime::parse_duration;
}

// Test rigging to replicate what main_test!() does,
// to allow decorations to to be used in unit tests.
// must be in root!
#[cfg(feature = "decorations")]
#[cfg(test)]
pub use crate::decorations::test_rigging::*;