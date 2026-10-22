---
title: "src/iced_core/mouse.rs"
kind: source
path: "src/iced_core/mouse.rs"
crate: "iced_core"
file: "src/iced_core/mouse.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/mouse.rs.html"
source_file: "src/iced_core/mouse.rs"
---

# `src/iced_core/mouse.rs`

```rust
//! Handle mouse events.
pub mod click;

mod button;
mod cursor;
mod event;
mod interaction;

pub use button::Button;
pub use click::Click;
pub use cursor::Cursor;
pub use event::{Event, ScrollDelta};
pub use interaction::Interaction;
```
