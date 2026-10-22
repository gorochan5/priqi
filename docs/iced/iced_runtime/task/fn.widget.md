---
title: "widget"
kind: function
path: "iced_runtime::task::widget"
crate: "iced_runtime"
module: "iced_runtime::task"
file: "iced_runtime/task/fn.widget.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/fn.widget.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "399-410"
---

# `iced_runtime::task::widget`

```rust
pub fn widget<T>(operation: impl Operation<T> + 'static) -> Task<T> where
    T: Send + 'static,
```

Creates a new [`Task`](struct.Task.md) that runs the given [`widget::Operation`] and produces its output.

**Source:** [task.rs](../../src/iced_runtime/task.rs.md) (lines 399-410)
