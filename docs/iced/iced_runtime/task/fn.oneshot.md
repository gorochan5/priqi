---
title: "oneshot"
kind: function
path: "iced_runtime::task::oneshot"
crate: "iced_runtime"
module: "iced_runtime::task"
file: "iced_runtime/task/fn.oneshot.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/fn.oneshot.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "414-432"
---

# `iced_runtime::task::oneshot`

```rust
pub fn oneshot<T>(f: impl FnOnce(Sender<T>) -> Action<T>) -> Task<T> where
    T: MaybeSend + 'static,
```

Creates a new [`Task`](struct.Task.md) that executes the [`Action`](../enum.Action.md) returned by the closure and produces the value fed to the [`oneshot::Sender`].

**Source:** [task.rs](../../src/iced_runtime/task.rs.md) (lines 414-432)
