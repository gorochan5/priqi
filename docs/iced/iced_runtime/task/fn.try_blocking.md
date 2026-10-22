---
title: "try_blocking"
kind: function
path: "iced_runtime::task::try_blocking"
crate: "iced_runtime"
module: "iced_runtime::task"
file: "iced_runtime/task/fn.try_blocking.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/fn.try_blocking.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "492-512"
---

# `iced_runtime::task::try_blocking`

```rust
pub fn try_blocking<T, E>(
    f: impl FnOnce(Sender<T>) -> Result<(), E> + Send + 'static,
) -> Task<Result<T, E>> where
    T: Send + 'static,
    E: Send + 'static,
```

Creates a new [`Task`](struct.Task.md) that will run the given closure that can fail in a new thread.

Any data sent by the closure through the [`mpsc::Sender`] will be produced by the [`Task`](struct.Task.md).

**Source:** [task.rs](../../src/iced_runtime/task.rs.md) (lines 492-512)
