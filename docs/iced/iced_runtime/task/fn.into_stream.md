---
title: "into_stream"
kind: function
path: "iced_runtime::task::into_stream"
crate: "iced_runtime"
module: "iced_runtime::task"
file: "iced_runtime/task/fn.into_stream.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/fn.into_stream.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "466-468"
---

# `iced_runtime::task::into_stream`

```rust
pub fn into_stream<T>(task: Task<T>) -> Option<BoxStream<Action<T>>>
```

Returns the underlying [`Stream`] of the [`Task`](struct.Task.md).

**Source:** [task.rs](../../src/iced_runtime/task.rs.md) (lines 466-468)
