use std::cmp;

use crate::scheduling::state_machine::TaskStream;
use crate::scheduling::{PollTaskResult, TaskNodePath};

use std::future::Future;
use std::pin::Pin;

use futures::stream::StreamExt;
use futures::task::Context;
use futures::task::Poll;

use crate::async_runtime::channel;
use crate::async_runtime::{Receiver, Sender};

struct TaskCompleteEvent(TaskNodePath);

struct TaskContext<Payload> {
    pub payload: Payload,
    pub tx: Sender<TaskCompleteEvent>,
    pub path: TaskNodePath,
}

pub struct TaskScheduler<Stream> {
    state_machine: Stream,
    max_concurrency: usize,
    rx: Receiver<TaskCompleteEvent>,
    tx: Sender<TaskCompleteEvent>,
}

impl<Stream: TaskStream> TaskScheduler<Stream> {
    pub fn new(state_machine: Stream, max_concurrency: usize) -> Self {
        // Don't waste resources, if we know the max pool size needed, then we shouldn't exceed it
        let actual_max_concurrency = cmp::min(max_concurrency, state_machine.max_concurrency());
        let (tx, rx) = channel::<TaskCompleteEvent>(actual_max_concurrency);

        Self {
            rx: rx,
            tx: tx,
            max_concurrency: actual_max_concurrency,
            state_machine: state_machine,
        }
    }

    pub async fn for_each_concurrent<InvokeFn, Fut>(self, invoke: InvokeFn)
    where
        InvokeFn: Fn(<Stream as TaskStream>::Payload) -> Fut + Copy,
        Fut: Future<Output = ()>,
    {
        let max_concurrency = self.max_concurrency;
        self.into_future_stream()
            .for_each_concurrent(max_concurrency, |ctx| async move {
                (invoke)(ctx.payload).await;
                // TODO: what todo?
                let _ = ctx.tx.send(TaskCompleteEvent(ctx.path)).await;
            })
            .await
    }

    fn into_future_stream(self) -> TaskSchedulerStream<Stream> {
        TaskSchedulerStream {
            task_scheduler: self,
        }
    }

    pub fn queue_length(&self) -> usize {
        self.state_machine.len()
    }

    fn try_poll(&mut self) -> PollTaskResult<TaskContext<<Stream as TaskStream>::Payload>> {
        self.update_completed_tasks();

        match self.state_machine.try_poll() {
            PollTaskResult::Next(payload, path) => PollTaskResult::Next(
                TaskContext {
                    payload: payload,
                    tx: self.tx.clone(),
                    path: path.clone(),
                },
                path,
            ),
            PollTaskResult::Busy => PollTaskResult::Busy,
            PollTaskResult::None => PollTaskResult::None,
        }
    }

    fn update_completed_tasks(&mut self) {
        loop {
            match self.rx.try_recv() {
                Ok(event) => {
                    self.state_machine.complete_task(event.0);
                }
                _ => {
                    return; // TODO: handle disconnect and empty correctly
                }
            }
        }
    }
}

struct TaskSchedulerStream<Stream> {
    task_scheduler: TaskScheduler<Stream>,
}

impl<Stream: TaskStream> Unpin for TaskSchedulerStream<Stream> {}

impl<Stream: TaskStream> futures::stream::Stream for TaskSchedulerStream<Stream> {
    type Item = TaskContext<<Stream as TaskStream>::Payload>;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.task_scheduler.try_poll() {
            PollTaskResult::Next(payload, _) => Poll::Ready(Some(payload)),
            PollTaskResult::Busy => Poll::Pending,
            PollTaskResult::None => Poll::Ready(None),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.task_scheduler.queue_length();
        (len, Some(len))
    }
}
