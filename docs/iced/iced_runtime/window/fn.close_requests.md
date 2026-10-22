---
title: "close_requests"
kind: function
path: "iced_runtime::window::close_requests"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.close_requests.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.close_requests.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "248-256"
---

# `iced_runtime::window::close_requests`

```rust
pub fn close_requests() -> Subscription<Id>
```

Subscribes to all [`Event::CloseRequested`] occurrences in the running application.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 248-256)
