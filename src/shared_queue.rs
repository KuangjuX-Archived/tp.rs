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
    pool: Vec<Option<thread::JoinHandle<()>>>,
    capacity: u32
}

impl SharedQueueThreadPool {
    /// 生成线程池
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

    /// 开始运行线程池
    pub fn run(&mut self) {
        for id in 0..self.capacity {
            let receiver = self.receiver.clone();
            let join_handle = thread::spawn(move || {
                run_task(id, receiver);
            });
            self.pool.push(Some(join_handle));
        }
    }

    /// 向线程池中传递执行方法
    pub fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static {
        let task = ThreadPoolMessage::Task(Box::new(job));
        println!("[Debug] 发送任务");
        self.sender.send(task).unwrap();
    }

    /// 销毁线程池
    pub fn shutdown(&mut self) {
        println!("[Debug] 关闭线程池");
        for _ in 0..self.capacity {
            self.sender.send(ThreadPoolMessage::Shutdown).unwrap();
        }

        for thread in &mut self.pool {
            if let Some(thread) = thread.take() {
                thread.join().unwrap();
            }
        }
    }

}

/// 执行任务方法
pub fn run_task(id: u32, receiver: Arc<Mutex<Receiver<ThreadPoolMessage>>>) {
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
    println!("[Debug] Thread {} exit", id);
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}
