---
title: "blocking"
kind: function
path: "iced_runtime::task::blocking"
crate: "iced_runtime"
module: "iced_runtime::task"
file: "iced_runtime/task/fn.blocking.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/fn.blocking.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "474-485"
---

# `iced_runtime::task::blocking`

```rust
pub fn blocking<T>(f: impl FnOnce(Sender<T>) + Send + 'static) -> Task<T> where
    T: Send + 'static,
```

Creates a new [`Task`](struct.Task.md) that will run the given closure in a new thread.

Any data sent by the closure through the [`mpsc::Sender`] will be produced by the [`Task`](struct.Task.md).

**Source:** [task.rs](../../src/iced_runtime/task.rs.md) (lines 474-485)
