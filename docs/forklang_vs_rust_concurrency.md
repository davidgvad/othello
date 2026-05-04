# ForkLang vs Rust: Concurrency

| Course ForkLang | Rust |
| --- | --- |
| `(fork exp exp')` | `thread::spawn(move || { ... })` |
| `(lock var)` | `mutex.lock()` |
| `(unlock var)` | automatic when the `MutexGuard` is dropped |
| shared heap location | shared data wrapped in `Arc<Mutex<T>>` |
| language feature | standard-library feature |
| programmer manually reasons about locks | compiler enforces ownership and thread-safety rules |

In ForkLang, concurrency is built directly into the language syntax. In Rust, concurrency is mostly provided through the standard library, but Rust's ownership and type system make those library tools safer.

```rust
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
```

Presentation script:

> In our course, ForkLang introduced concurrency with explicit language constructs: `fork`, `lock`, and `unlock`. Rust does not use those exact expressions. Instead, it uses `std::thread::spawn` to create threads, which is similar to `fork`, and `Mutex` to protect shared data, which is similar to `lock`. The major difference is that Rust does not usually require an explicit `unlock`; the lock is released automatically when the guard goes out of scope. Shared mutable data must be wrapped in types like `Arc<Mutex<T>>`, so Rust's type system makes unsafe sharing harder to write.

Run the demo with:

```bash
cargo run --example concurrency_demo
```
