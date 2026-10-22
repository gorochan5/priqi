---
title: "open"
kind: function
path: "iced_runtime::window::open"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.open.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.open.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "260-267"
---

# `iced_runtime::window::open`

```rust
pub fn open(settings: Settings) -> (Id, Task<Id>)
```

Opens a new window with the given [`Settings`]; producing the [`Id`] of the new window on completion.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 260-267)
