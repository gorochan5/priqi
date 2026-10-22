---
title: "src/iced_core/window/settings/linux.rs"
kind: source
path: "src/iced_core/window/settings/linux.rs"
crate: "iced_core"
file: "src/iced_core/window/settings/linux.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/window/settings/linux.rs.html"
source_file: "src/iced_core/window/settings/linux.rs"
---

# `src/iced_core/window/settings/linux.rs`

```rust
//! Platform specific settings for Linux.

/// The platform specific window settings of an application.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlatformSpecific {
    /// Sets the application id of the window.
    ///
    /// As a best practice, it is suggested to select an application id that match
    /// the basename of the application’s .desktop file.
    pub application_id: String,

    /// Whether bypass the window manager mapping for x11 windows
    ///
    /// This flag is particularly useful for creating UI elements that need precise
    /// positioning and immediate display without window manager interference.
    pub override_redirect: bool,
}
```
