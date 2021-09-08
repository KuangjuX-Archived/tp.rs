use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread::JoinHandle;

enum ThreadPoolMessage {
    RunJon(Box<dyn FnOnce() + Send + 'static>),
    Shutdown
}

pub struct SharedQueueThreadPool {
    receiver: Mutex<Receiver<ThreadPoolMessage>>,
    size: usize
}

impl SharedQueueThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = channel::<ThreadPoolMessage>();
        Self {
            receiver: Mutex::new(receiver),
            size
        }
    }
}