use std::sync::mpsc::*;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
pub enum ThreadPoolMessage {
    Task(Box<dyn FnOnce() + Send + 'static>),
    Shutdown
}

// type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct SharedQueueThreadPool {
    receiver: Arc<Mutex<Receiver<ThreadPoolMessage>>>,
    sender: Arc<Sender<ThreadPoolMessage>>,
    pool: Vec<thread::JoinHandle<()>>,
    capacity: u32
}

impl SharedQueueThreadPool {
    pub fn new(threads: u32) -> Result<SharedQueueThreadPool, ()> {
        let (sender, receiver) = channel::<ThreadPoolMessage>();
        let receiver = Arc::new(Mutex::new(receiver));
        Ok(Self {
            receiver,
            sender: Arc::new(sender),
            capacity: threads,
            pool: vec![]
        })
    }

    pub fn run(&mut self) {
        for _ in 0..self.capacity {
            let receiver = self.receiver.clone();
            let join_handle = thread::spawn(move || {
                run_task(receiver);
            });
            self.pool.push(join_handle);
        }
    }

    pub fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static {
        let task = ThreadPoolMessage::Task(Box::new(job));
        println!("[Debug] 发送任务");
        self.sender.send(task).unwrap();
    }

    pub fn shutdown(&self) {
        println!("[Debug] 关闭线程池");
        for _ in 0..self.capacity {
            self.sender.send(ThreadPoolMessage::Shutdown).unwrap();
        }
    }

}

/// 执行任务方法
pub fn run_task(receiver: Arc<Mutex<Receiver<ThreadPoolMessage>>>) {
    println!("[Debug] 开始运行任务");
    loop {
        let recv = receiver.lock().unwrap();
        match recv.recv().unwrap() {
            ThreadPoolMessage::Task(task) => {
                task();
            },

            ThreadPoolMessage::Shutdown => {
                break;
            }
        }
    }
}
