mod shared_queue;
mod rayon_thread;

pub use shared_queue::*;
pub use rayon_thread::*;

pub trait ThreadPool {
    fn new(threads: u32) -> Result<Self, ()>
    where
        Self: Sized;
    
    fn spawn<F>(&self, job: F)
    where 
        F: FnOnce() + Send + 'static;
}