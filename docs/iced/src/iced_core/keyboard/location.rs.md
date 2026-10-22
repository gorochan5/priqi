---
title: "src/iced_core/keyboard/location.rs"
kind: source
path: "src/iced_core/keyboard/location.rs"
crate: "iced_core"
file: "src/iced_core/keyboard/location.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/keyboard/location.rs.html"
source_file: "src/iced_core/keyboard/location.rs"
---

# `src/iced_core/keyboard/location.rs`

```rust
/// The location of a key on the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// The standard group of keys on the keyboard.
    Standard,
    /// The left side of the keyboard.
    Left,
    /// The right side of the keyboard.
    Right,
    /// The numpad of the keyboard.
    Numpad,
}
```
