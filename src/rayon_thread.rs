use rayon;
use super::ThreadPool;
pub struct RayonThreadPool {
    pool: rayon::ThreadPool
}

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<Self, ()> {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads as usize).build().unwrap();
        Ok(Self{pool})
    }

    fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static {
        self.pool.spawn(job);
    }
}

impl Drop for RayonThreadPool {
    fn drop(&mut self) {
        drop(&mut self.pool);
    }
}