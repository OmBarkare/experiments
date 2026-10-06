use std::{
    hint::black_box,
    sync::{Arc, Barrier, Mutex, RwLock},
    thread,
    time::Instant,
};

const ITERS_PER_THREAD: u64 = 20_000_000;
fn bench_rwlock_reads(thread_count: usize) {
    let data = Arc::new(RwLock::new(7u64));
    let mut handles = Vec::with_capacity(thread_count);
    let barrier = Arc::new(Barrier::new(thread_count));

    let start = Instant::now();
    for _ in 0..thread_count {
        let shared_lock = Arc::clone(&data);
        let shared_barrier = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            shared_barrier.wait();

            for _ in 0..ITERS_PER_THREAD {
                let guard = shared_lock.read().unwrap();
                black_box(*guard);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "RwLock (reads) | {:2} threads | Total Time: {:>8.2?}",
        thread_count, elapsed
    );
}

fn bench_mutex(thread_count: usize) {
    let data = Arc::new(Mutex::new(11u64));
    let barrier = Arc::new(Barrier::new(thread_count));
    let mut handles = Vec::with_capacity(thread_count);

    let start = Instant::now();
    for _ in 0..thread_count {
        let shared_data = Arc::clone(&data);
        let shared_barrier = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            shared_barrier.wait();

            for _ in 0..ITERS_PER_THREAD {
                let guard = shared_data.lock().unwrap();
                black_box(*guard);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    let elapsed = start.elapsed();
    println!(
        "Mutex          | {:2} threads | Total Time: {:>8.2?}",
        thread_count, elapsed
    );
}

fn main() {
    let thread_counts = [1, 2, 4, 8, 16];

    println!("RwLock---");
    for t in thread_counts {
        bench_rwlock_reads(t);
    }

    println!("Mutex---");
    for t in thread_counts {
        bench_mutex(t);
    }
}
