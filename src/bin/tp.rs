use threadpool::*;

fn main() {
    let mut tp = RayonThreadPool::new(4).unwrap();
    for _ in 0..16 {
        tp.spawn(|| {
            println!("Hello World");
        });
    }
    drop(tp);
    // tp.shutdown();
    // loop{}
}