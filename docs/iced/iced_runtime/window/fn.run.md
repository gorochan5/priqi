---
title: "run"
kind: function
path: "iced_runtime::window::run"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.run.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.run.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "432-444"
---

# `iced_runtime::window::run`

```rust
pub fn run<T>(
    id: Id,
    f: impl FnOnce(&dyn Window) -> T + Send + 'static,
) -> Task<T> where
    T: Send + 'static,
```

Runs the given callback with a reference to the [`Window`] with the given [`Id`].

Note that if the window closes before this call is processed the callback will not be run.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 432-444)
