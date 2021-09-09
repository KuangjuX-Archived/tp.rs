use threadpool::*;

fn main() {
    let mut tp = SharedQueueThreadPool::new(4).unwrap();
    tp.run();
    for _ in 0..16 {
        tp.spawn(|| {
            println!("Hello World");
        });
    }
    // tp.shutdown();
    // loop{}
}