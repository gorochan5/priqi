---
title: "src/iced_core/window/user_attention.rs"
kind: source
path: "src/iced_core/window/user_attention.rs"
crate: "iced_core"
file: "src/iced_core/window/user_attention.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/window/user_attention.rs.html"
source_file: "src/iced_core/window/user_attention.rs"
---

# `src/iced_core/window/user_attention.rs`

```rust
/// The type of user attention to request.
///
/// ## Platform-specific
///
/// - **X11:** Sets the WM's `XUrgencyHint`. No distinction between [`Critical`] and [`Informational`].
///
/// [`Critical`]: Self::Critical
/// [`Informational`]: Self::Informational
#[derive(Debug, Clone, Copy)]
pub enum UserAttention {
    /// ## Platform-specific
    ///
    /// - **macOS:** Bounces the dock icon until the application is in focus.
    /// - **Windows:** Flashes both the window and the taskbar button until the application is in focus.
    Critical,
    /// ## Platform-specific
    ///
    /// - **macOS:** Bounces the dock icon once.
    /// - **Windows:** Flashes the taskbar button until the application is in focus.
    Informational,
}
```
