---
title: "effect"
kind: function
path: "iced_runtime::task::effect"
crate: "iced_runtime"
module: "iced_runtime::task"
file: "iced_runtime/task/fn.effect.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/fn.effect.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "454-463"
---

# `iced_runtime::task::effect`

```rust
pub fn effect<T>(action: impl Into<Action<Infallible>>) -> Task<T>
```

Creates a new [`Task`](struct.Task.md) that executes the given [`Action`](../enum.Action.md) and produces no output.

**Source:** [task.rs](../../src/iced_runtime/task.rs.md) (lines 454-463)
