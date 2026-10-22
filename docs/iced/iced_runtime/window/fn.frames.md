---
title: "frames"
kind: function
path: "iced_runtime::window::frames"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.frames.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.frames.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "196-201"
---

# `iced_runtime::window::frames`

```rust
pub fn frames() -> Subscription<Instant>
```

Subscribes to the frames of the window of the running application.

The resulting [`Subscription`] will produce items at a rate equal to the refresh rate of the first application window. Note that this rate may be variable, as it is normally managed by the graphics driver and/or the OS.

In any case, this [`Subscription`] is useful to smoothly draw application-driven animations without missing any frames.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 196-201)
