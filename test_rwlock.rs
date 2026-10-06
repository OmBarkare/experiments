use std::{hint::black_box, sync::{Arc, Barrier, RwLock}, thread};
const ITERS: u64 = 20_000_000;
fn main() {
    let thread_count = 8;
    let data = Arc::new(RwLock::new(7u64));
    let mut handles = Vec::with_capacity(thread_count);
    let barrier = Arc::new(Barrier::new(thread_count));
    for _ in 0..thread_count {
        let d = Arc::clone(&data);
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            b.wait();
            for _ in 0..ITERS { let g = d.read().unwrap(); black_box(*g); }
        }));
    }
    for h in handles { h.join().unwrap(); }
}
