---
title: "src/iced_core/touch.rs"
kind: source
path: "src/iced_core/touch.rs"
crate: "iced_core"
file: "src/iced_core/touch.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/touch.rs.html"
source_file: "src/iced_core/touch.rs"
---

# `src/iced_core/touch.rs`

```rust
//! Build touch events.
use crate::Point;

/// A touch interaction.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum Event {
    /// A touch interaction was started.
    FingerPressed { id: Finger, position: Point },

    /// An on-going touch interaction was moved.
    FingerMoved { id: Finger, position: Point },

    /// A touch interaction was ended.
    FingerLifted { id: Finger, position: Point },

    /// A touch interaction was canceled.
    FingerLost { id: Finger, position: Point },
}

/// A unique identifier representing a finger on a touch interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Finger(pub u64);
```
