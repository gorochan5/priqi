---
title: "src/iced_core/window/mode.rs"
kind: source
path: "src/iced_core/window/mode.rs"
crate: "iced_core"
file: "src/iced_core/window/mode.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/window/mode.rs.html"
source_file: "src/iced_core/window/mode.rs"
---

# `src/iced_core/window/mode.rs`

```rust
/// The mode of a window-based application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// The application appears in its own window.
    Windowed,

    /// The application takes the whole screen of its current monitor.
    Fullscreen,

    /// The application is hidden
    Hidden,
}
```
