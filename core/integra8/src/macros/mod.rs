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

#[macro_export]
macro_rules! sleep {
    ($duration:expr) => {
        $crate::async_runtime::sleep($duration).await
    };
}