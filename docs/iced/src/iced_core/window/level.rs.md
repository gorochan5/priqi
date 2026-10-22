---
title: "src/iced_core/window/level.rs"
kind: source
path: "src/iced_core/window/level.rs"
crate: "iced_core"
file: "src/iced_core/window/level.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/window/level.rs.html"
source_file: "src/iced_core/window/level.rs"
---

# `src/iced_core/window/level.rs`

```rust
/// A window level groups windows with respect to their z-position.
///
/// The relative ordering between windows in different window levels is fixed.
/// The z-order of a window within the same window level may change dynamically
/// on user interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Level {
    /// The default behavior.
    #[default]
    Normal,

    /// The window will always be below normal windows.
    ///
    /// This is useful for a widget-based app.
    AlwaysOnBottom,

    /// The window will always be on top of normal windows.
    AlwaysOnTop,
}
```
