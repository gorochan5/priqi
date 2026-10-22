---
title: "src/iced_futures/keyboard.rs"
kind: source
path: "src/iced_futures/keyboard.rs"
crate: "iced_futures"
file: "src/iced_futures/keyboard.rs.md"
docs_url: "https://docs.iced.rs/src/iced_futures/keyboard.rs.html"
source_file: "src/iced_futures/keyboard.rs"
---

# `src/iced_futures/keyboard.rs`

```rust
//! Listen to keyboard events.
use crate::core;
use crate::core::keyboard::Event;
use crate::subscription::{self, Subscription};

/// Returns a [`Subscription`] that listens to ignored keyboard events.
pub fn listen() -> Subscription<Event> {
    #[derive(Hash)]
    struct Listen;

    subscription::filter_map(Listen, move |event| match event {
        subscription::Event::Interaction {
            event: core::Event::Keyboard(event),
            status: core::event::Status::Ignored,
            ..
        } => Some(event),
        _ => None,
    })
}
```
