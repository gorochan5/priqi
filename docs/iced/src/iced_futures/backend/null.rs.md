---
title: "src/iced_futures/backend/null.rs"
kind: source
path: "src/iced_futures/backend/null.rs"
crate: "iced_futures"
file: "src/iced_futures/backend/null.rs.md"
docs_url: "https://docs.iced.rs/src/iced_futures/backend/null.rs.html"
source_file: "src/iced_futures/backend/null.rs"
---

# `src/iced_futures/backend/null.rs`

```rust
//! A backend that does nothing!
use crate::MaybeSend;

/// An executor that drops all the futures, instead of spawning them.
#[derive(Debug)]
pub struct Executor;

impl crate::Executor for Executor {
    fn new() -> Result<Self, futures::io::Error> {
        Ok(Self)
    }

    fn spawn(&self, _future: impl Future<Output = ()> + MaybeSend + 'static) {}

    #[cfg(not(target_arch = "wasm32"))]
    fn block_on<T>(&self, _future: impl Future<Output = T>) -> T {
        unimplemented!()
    }
}

pub mod time {
    //! Listen and react to time.
}
```
