use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, thread, time::Duration};

use once_cell::sync::OnceCell;
use sharded_slab::Pool;

static STOP: AtomicBool = AtomicBool::new(false);

fn get_pool() -> Arc<Pool<Vec<&'static str>>> {
    static POOL: OnceCell<Arc<Pool<Vec<&'static str>>>> = OnceCell::new();
    POOL.get_or_init(|| Arc::new(Pool::new())).clone()
}

fn main() {
    let mut handles = Vec::new();
    for _ in 0..16 {
        let handle = thread::spawn(run_pooler);
        handles.push(handle);
    }

    thread::sleep(Duration::from_secs(3600));
    STOP.store(true, Ordering::Release);

    for handle in handles.drain(..) {
        handle.join().expect("thread should have completed cleanly");
    }
}

fn run_pooler() {
    loop {
        if STOP.load(Ordering::Acquire) {
            break
        }

        let pool = get_pool();
        let mut vector = pool.clone().create_owned().expect("should not run out");
        vector.push("foo");
        pool.clear(vector.key());
        drop(vector);
    }
}
