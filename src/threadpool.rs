use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread::*;

enum ThreadPoolMessage {
    RunJon(Box<dyn FnOnce() + Send + 'static>),
    Shutdown
}

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct SharedQueueThreadPool {
    receiver: Mutex<Receiver<ThreadPoolMessage>>,
    pool: Vec<JoinHandle<Task>>,
    capacity: usize
}

impl SharedQueueThreadPool {
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = channel::<ThreadPoolMessage>();
        Self {
            receiver: Mutex::new(receiver),
            pool: vec![],
            capacity
        }
    }
}