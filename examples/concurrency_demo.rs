use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared_score = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..2 {
        let score = Arc::clone(&shared_score);

        let handle = thread::spawn(move || {
            let mut value = score.lock().unwrap();
            *value += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Shared score after two threads: {}",
        *shared_score.lock().unwrap()
    );
}
