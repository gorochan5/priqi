---
title: "src/iced_core/window/position.rs"
kind: source
path: "src/iced_core/window/position.rs"
crate: "iced_core"
file: "src/iced_core/window/position.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/window/position.rs.html"
source_file: "src/iced_core/window/position.rs"
---

# `src/iced_core/window/position.rs`

```rust
use crate::{Point, Size};

/// The position of a window in a given screen.
#[derive(Debug, Clone, Copy, Default)]
pub enum Position {
    /// The platform-specific default position for a new window.
    #[default]
    Default,
    /// The window is completely centered on the screen.
    Centered,
    /// The window is positioned with specific coordinates: `(X, Y)`.
    ///
    /// When the decorations of the window are enabled, Windows 10 will add some
    /// invisible padding to the window. This padding gets included in the
    /// position. So if you have decorations enabled and want the window to be
    /// at (0, 0) you would have to set the position to
    /// `(PADDING_X, PADDING_Y)`.
    Specific(Point),
    /// Like [`Specific`], but the window is positioned with the specific coordinates returned by the function.
    ///
    /// The function receives the window size and the monitor's resolution as input.
    ///
    /// [`Specific`]: Self::Specific
    SpecificWith(fn(Size, Size) -> Point),
}
```
