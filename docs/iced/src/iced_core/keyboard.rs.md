---
title: "src/iced_core/keyboard.rs"
kind: source
path: "src/iced_core/keyboard.rs"
crate: "iced_core"
file: "src/iced_core/keyboard.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/keyboard.rs.html"
source_file: "src/iced_core/keyboard.rs"
---

# `src/iced_core/keyboard.rs`

```rust
//! Listen to keyboard events.
pub mod key;

mod event;
mod location;
mod modifiers;

pub use event::Event;
pub use key::Key;
pub use location::Location;
pub use modifiers::Modifiers;
```
