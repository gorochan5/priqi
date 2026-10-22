---
title: "frames"
kind: function
path: "iced::window::frames"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.frames.md"
docs_url: "https://docs.iced.rs/iced/window/fn.frames.html"
---

# `iced::window::frames`

```rust
pub fn frames() -> Subscription<Instant>
```

Subscribes to the frames of the window of the running application.

The resulting [`Subscription`](../struct.Subscription.md) will produce items at a rate equal to the refresh rate of the first application window. Note that this rate may be variable, as it is normally managed by the graphics driver and/or the OS.

In any case, this [`Subscription`](../struct.Subscription.md) is useful to smoothly draw application-driven animations without missing any frames.
