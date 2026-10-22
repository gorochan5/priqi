---
title: "src/iced_futures/backend/native/thread_pool.rs"
kind: source
path: "src/iced_futures/backend/native/thread_pool.rs"
crate: "iced_futures"
file: "src/iced_futures/backend/native/thread_pool.rs.md"
docs_url: "https://docs.iced.rs/src/iced_futures/backend/native/thread_pool.rs.html"
source_file: "src/iced_futures/backend/native/thread_pool.rs"
---

# `src/iced_futures/backend/native/thread_pool.rs`

```rust
//! A `ThreadPool` backend.

/// A thread pool executor for futures.
pub type Executor = futures::executor::ThreadPool;

impl crate::Executor for Executor {
    fn new() -> Result<Self, futures::io::Error> {
        futures::executor::ThreadPool::new()
    }

    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        self.spawn_ok(future);
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        futures::executor::block_on(future)
    }
}

pub mod time {
    //! Listen and react to time.
}
```
