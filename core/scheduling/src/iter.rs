use crate::TaskStream;

pub trait TaskStreamMap<In, Out> {
    fn map<F>(self, f: F) -> map::TaskStreamMap<Self, F>
    where
        Self: Sized + TaskStream<Payload = In>,
        F: FnMut(In) -> Out;
}

impl<In, Out, Stream> TaskStreamMap<In, Out> for Stream
where
    Stream: TaskStream<Payload = In>,
{
    fn map<F>(self, f: F) -> map::TaskStreamMap<Self, F>
    where
        Self: Sized + TaskStream<Payload = In>,
        F: FnMut(In) -> Out,
    {
        map::TaskStreamMap::new(self, f)
    }
}

mod map {
    use crate::{PollTaskResult, TaskNodePath, TaskStream};

    pub struct TaskStreamMap<Stream, F> {
        stream: Stream,
        f: F,
    }

    impl<Stream, F> TaskStreamMap<Stream, F> {
        pub fn new(stream: Stream, f: F) -> TaskStreamMap<Stream, F> {
            Self {
                stream: stream,
                f: f,
            }
        }
    }

    impl<Payload, Stream: TaskStream, F> TaskStream for TaskStreamMap<Stream, F>
    where
        F: FnMut(Stream::Payload) -> Payload,
    {
        type Payload = Payload;

        fn try_poll(&mut self) -> PollTaskResult<Self::Payload> {
            match self.stream.try_poll() {
                PollTaskResult::Next(payload, path) => {
                    PollTaskResult::Next((self.f)(payload), path)
                }
                PollTaskResult::Busy => PollTaskResult::Busy,
                PollTaskResult::None => PollTaskResult::None,
            }
        }

        fn max_concurrency(&self) -> usize {
            self.stream.max_concurrency()
        }

        fn complete_task(&mut self, path: TaskNodePath) -> bool {
            self.stream.complete_task(path)
        }

        fn len(&self) -> usize {
            self.stream.len()
        }
    }
}
