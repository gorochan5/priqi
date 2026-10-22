---
title: "channel"
kind: function
path: "iced_runtime::task::channel"
crate: "iced_runtime"
module: "iced_runtime::task"
file: "iced_runtime/task/fn.channel.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/fn.channel.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "436-451"
---

# `iced_runtime::task::channel`

```rust
pub fn channel<T>(f: impl FnOnce(Sender<T>) -> Action<T>) -> Task<T> where
    T: MaybeSend + 'static,
```

Creates a new [`Task`](struct.Task.md) that executes the [`Action`](../enum.Action.md) returned by the closure and produces the values fed to the [`mpsc::Sender`].

**Source:** [task.rs](../../src/iced_runtime/task.rs.md) (lines 436-451)
