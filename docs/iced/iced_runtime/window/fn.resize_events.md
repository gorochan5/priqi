---
title: "resize_events"
kind: function
path: "iced_runtime::window::resize_events"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.resize_events.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.resize_events.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "237-245"
---

# `iced_runtime::window::resize_events`

```rust
pub fn resize_events() -> Subscription<(Id, Size)>
```

Subscribes to all [`Event::Resized`] occurrences in the running application.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 237-245)
