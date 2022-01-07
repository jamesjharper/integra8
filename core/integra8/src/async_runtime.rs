#[cfg(feature = "tokio-runtime")]
mod channel_impl {
    pub use tokio::sync::mpsc::channel;
    pub use tokio::sync::mpsc::{Receiver, Sender};
}

#[cfg(feature = "async-std-runtime")]
mod channel_impl {
    pub use async_std::sync::channel;
    pub use async_std::sync::{Receiver, Sender};
}

#[cfg(not(any(feature = "tokio-runtime", feature = "async-std-runtime")))]
mod channel_impl {
    pub use std::sync::mpsc::channel;
    pub use std::sync::mpsc::{Receiver, Sender};
}

pub use channel_impl::channel;
pub use channel_impl::{Receiver, Sender};

#[cfg(feature = "tokio-runtime")]
pub fn timeout<T: futures::Future>(
    duration: std::time::Duration,
    future: T,
) -> tokio::time::Timeout<T> {
    tokio::time::timeout(duration, future)
}

#[cfg(feature = "tokio-runtime")]
pub fn spawn<T>(task: T) -> tokio::task::JoinHandle<T::Output>
where
    T: futures::Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::spawn(task)
}

#[cfg(feature = "async-std-runtime")]
pub async fn timeout<F, T>(
    duration: std::time::Duration,
    future: F,
) -> Result<T, async_std::future::TimeoutError>
where
    F: futures::Future<Output = T>,
{
    async_std::future::timeout(duration, future).await
}

#[cfg(feature = "async-std-runtime")]
pub fn spawn<F, T>(future: F) -> async_std::task::JoinHandle<T>
where
    F: futures::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    async_std::task::spawn(future)
}
