---
title: "close_events"
kind: function
path: "iced_runtime::window::close_events"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.close_events.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.close_events.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "226-234"
---

# `iced_runtime::window::close_events`

```rust
pub fn close_events() -> Subscription<Id>
```

Subscribes to all [`Event::Closed`] occurrences in the running application.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 226-234)
