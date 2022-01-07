use std::cmp;

pub trait TaskStream {
    type Payload;
    fn try_poll(&mut self) -> PollTaskResult<Self::Payload>;
    fn max_concurrency(&self) -> usize;
    fn complete_task(&mut self, path: TaskNodePath) -> bool;
    fn len(&self) -> usize;  
}

pub enum PollTaskResult<Payload> {
    Next(Payload, TaskNodePath),
    None,
    Busy, 
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskNodePath {
    elements: Vec<usize>
}

impl TaskNodePath {
    pub fn new() -> Self {
        Self {
            elements : Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.elements.pop()
    }

    pub fn append(mut self, idx: usize) -> Self {
        self.elements.push(idx);
        self
    }
}

#[derive(Debug)]
pub enum TaskStateMachineNode<Payload> {
    Single(TaskNode<Payload>),
    Serial(SerialTaskNode<Payload>),
    Parallel(ParallelTaskNode<Payload>),
}


impl<Payload> From<TaskNode<Payload>> for TaskStateMachineNode<Payload> {
    fn from(n: TaskNode<Payload>) -> TaskStateMachineNode<Payload> {
        TaskStateMachineNode::Single(n)
    }
}

impl<Payload> From<Payload> for TaskStateMachineNode<Payload> {
    fn from(p: Payload) -> TaskStateMachineNode<Payload> {
        TaskStateMachineNode::Single(TaskNode::new(p))
    }
}

impl<Payload> From<SerialTaskNode<Payload>> for TaskStateMachineNode<Payload> {
    fn from(n: SerialTaskNode<Payload>) -> TaskStateMachineNode<Payload> {
        TaskStateMachineNode::Serial(n)
    }
}

impl<Payload> From<ParallelTaskNode<Payload>> for TaskStateMachineNode<Payload> {
    fn from(n: ParallelTaskNode<Payload>) -> TaskStateMachineNode<Payload> {
        TaskStateMachineNode::Parallel(n)
    }
}

impl<Payload> TaskStream for TaskStateMachineNode<Payload> {
    
    type Payload = Payload;
    fn try_poll(&mut self) -> PollTaskResult<Self::Payload> {
        match self {
            Self::Single(node) => node.try_poll(),
            Self::Serial(node) => node.try_poll(),
            Self::Parallel(node) => node.try_poll(),
        }
    }
    
    fn max_concurrency(&self) -> usize {
        match self {
            Self::Single(node) => node.max_concurrency(),
            Self::Serial(node) => node.max_concurrency(),
            Self::Parallel(node) => node.max_concurrency(),
        }
    }

    fn complete_task(&mut self, path: TaskNodePath) -> bool {
        match self {
            Self::Single(node) => node.complete_task(path),
            Self::Serial(node) => node.complete_task(path),
            Self::Parallel(node) => node.complete_task(path),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Serial(node) => node.len(),
            Self::Parallel(node) => node.len(),
        }
    }  
}

// TaskNode

#[derive(Debug)]
pub enum TaskNode<Payload> {
    NotStarted(Option<Payload>),
    InProgress,
    IsDone, 
}

impl<Payload> TaskNode<Payload> {
    pub fn new(payload: Payload) -> Self {
        Self::NotStarted(Some(payload))
    }

    pub fn start(&mut self) -> Payload {
        let payload = match self {
            Self::NotStarted(ref mut p) => std::mem::take(p),
            _ => None
        };

        *self = Self::InProgress;
        payload.unwrap()
    }
}

impl<Payload> TaskStream for TaskNode<Payload> {
    type Payload = Payload;

    fn try_poll(&mut self) -> PollTaskResult<Self::Payload> {
        match self {
            Self::NotStarted(_) => { },
            Self::InProgress => {
                return PollTaskResult::Busy;
            },
            Self::IsDone => {
                return PollTaskResult::None;
            }
        };

        PollTaskResult::Next(self.start(), TaskNodePath::new())
    }

    fn complete_task(&mut self, path: TaskNodePath) -> bool {
        match self {
            Self::IsDone => {
                // Already done,
                false
            },
            _ => {
                *self = Self::IsDone;
                // The path should be empty at this point, otherwise
                // this path didn't resolve to a node correctly.
                // should probably raise some kind of error here 
                path.is_empty()
            }
        }
    }

    fn max_concurrency(&self) -> usize {
        1
    }

    fn len(&self) -> usize {
        1
    }
}
// Parallel Task Node

#[derive(Debug)]
pub struct ParallelTaskNode<Payload> {
    nodes: Vec<TaskStateMachineNode<Payload>>,
    done: usize,
    total: usize,
}

impl<Payload> ParallelTaskNode<Payload> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            done: 0,
            total: 0
        }
    }

    pub fn append(&mut self, node: impl Into<TaskStateMachineNode<Payload>>) {
        match node.into() {
            TaskStateMachineNode::Single(node) => self.append_task(node),
            TaskStateMachineNode::Serial(node) => self.append_serial(node),
            TaskStateMachineNode::Parallel(node) => self.append_parallel(node),
        }
    }

    pub fn append_all<I, IntoNode>(&mut self, iter : I)
    where
        I: IntoIterator<Item = IntoNode>,
        IntoNode: Into<TaskStateMachineNode<Payload>>
    {
        for node in iter.into_iter() {
            self.append(node)
        }
    }

    pub fn append_task(&mut self, task: TaskNode<Payload>) {
        self.total = self.total.saturating_add(1);
        self.nodes.push(TaskStateMachineNode::Single(task));
    }

    pub fn append_parallel(&mut self, mut parallel: ParallelTaskNode<Payload>) {
        if !parallel.is_empty() {
            self.total = self.total.saturating_add(parallel.total);
            self.nodes.append(&mut parallel.nodes);
        }
    }

    pub fn append_serial(&mut self, serial: SerialTaskNode<Payload>) {
        if !serial.is_empty() {
            self.total = self.total.saturating_add(serial.total);
            self.nodes.push(TaskStateMachineNode::Serial(serial));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
}


impl<Payload> TaskStream for ParallelTaskNode<Payload> {
    type Payload = Payload;

    fn try_poll(&mut self) -> PollTaskResult<Self::Payload> {
        if self.len() == 0 {
            return PollTaskResult::None;
        }

        let mut is_busy = false;

        self.nodes
            .iter_mut()
            .enumerate() 
            .find_map(|(idx, node)| {
                match node.try_poll() {
                    PollTaskResult::None => None,
                    PollTaskResult::Busy => {
                        is_busy = true;
                        // keep looking
                        None
                    },
                    PollTaskResult::Next(payload, path) => {
                        // Break on first ready
                        Some(PollTaskResult::Next(payload, path.append(idx)))
                    }
                }
            })
            .unwrap_or_else(|| {
                match is_busy {
                    true => PollTaskResult::Busy, 
                    false => PollTaskResult::None
                }
            })
    }

    fn complete_task(&mut self, mut path: TaskNodePath) -> bool {
        match path.pop() {
            None => {
                // This should be some kind of error
                false
            },
            Some(idx) => {
                if let Some(n) = self.nodes.get_mut(idx) {
                    if n.complete_task(path) {
                        self.done += 1;
                        return true;
                    }
                }
                return false;
            }
        }
    }

    fn max_concurrency(&self) -> usize {
        self.nodes
            .iter()
            .fold(0, |total , x| {
                    x.max_concurrency() + total
                }
            )
    }

    fn len(&self) -> usize {
        self.total.saturating_sub(self.done)
    }
}

// Serial Task Node

#[derive(Debug)]
pub struct SerialTaskNode<Payload> {
    nodes: Vec<TaskStateMachineNode<Payload>>,
    done: usize,
    total: usize,
    current_idx: usize,
}

impl<Payload> SerialTaskNode<Payload> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            done: 0,
            total: 0,
            current_idx: 0,
        }
    }

    pub fn enqueue(&mut self, node: impl Into<TaskStateMachineNode<Payload>>) {
        match node.into() {
            TaskStateMachineNode::Single(node) => self.enqueue_task(node),
            TaskStateMachineNode::Serial(node) => self.enqueue_serial(node),
            TaskStateMachineNode::Parallel(node) => self.enqueue_parallel(node),
        }
    }

    pub fn enqueue_all<I, IntoNode>(&mut self, iter : I)
    where
        I: IntoIterator<Item = IntoNode>,
        IntoNode: Into<TaskStateMachineNode<Payload>>
    {
        for node in iter.into_iter() {
            self.enqueue(node)
        }
    }

    pub fn enqueue_task(&mut self, single: TaskNode<Payload>) {
        self.total = self.total.saturating_add(1);
        self.nodes.push(TaskStateMachineNode::Single(single));
    }

    pub fn enqueue_parallel(&mut self, parallel: ParallelTaskNode<Payload>) {
        if !parallel.is_empty() {
            self.total = self.total.saturating_add(parallel.total);
            self.nodes.push(TaskStateMachineNode::Parallel(parallel));
        }
    }

    pub fn enqueue_serial(&mut self, mut serial: SerialTaskNode<Payload>) {
        if !serial.is_empty() {
            self.total = self.total.saturating_add(serial.total);
            self.nodes.append(&mut serial.nodes);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
}

impl<Payload> TaskStream for SerialTaskNode<Payload> {
    type Payload = Payload;

    fn try_poll(&mut self) -> PollTaskResult<Self::Payload> {
        loop {
            match self.nodes.get_mut(self.current_idx).map(|node| node.try_poll()) {
                Some(PollTaskResult::Next(payload, path)) => {
                    return PollTaskResult::Next(payload, path.append(self.current_idx))
                },
                Some(PollTaskResult::None) => {
                    // Move to the next node, and get the next task
                    self.current_idx += 1;
                },
                Some(PollTaskResult::Busy) => {
                    return PollTaskResult::Busy;
                },
                None => {
                    return PollTaskResult::None;
                },
            }
        }
    }

    fn complete_task(&mut self, mut path: TaskNodePath) -> bool {
        match path.pop() {
            None => {
                // This should be some kind of error
                false
            },
            Some(idx) => {
                if let Some(n) = self.nodes.get_mut(idx) {
                    if n.complete_task(path) {
                        self.done += 1;
                        return true;
                    }
                }
                return false;
            }
        }
    }

    fn max_concurrency(&self) -> usize {
        self.nodes
            .iter()
            .fold(1 /*min value */, 
                |max, x| {
                    cmp::max(max, x.max_concurrency())
                }
            )
    }

    fn len(&self) -> usize {
        self.total.saturating_sub(self.done)
    }
}


