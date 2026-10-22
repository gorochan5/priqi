---
title: "src/iced_core/window/id.rs"
kind: source
path: "src/iced_core/window/id.rs"
crate: "iced_core"
file: "src/iced_core/window/id.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/window/id.rs.html"
source_file: "src/iced_core/window/id.rs"
---

# `src/iced_core/window/id.rs`

```rust
use std::fmt;
use std::hash::Hash;
use std::sync::atomic::{self, AtomicU64};

/// The id of the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id(u64);

static COUNT: AtomicU64 = AtomicU64::new(1);

impl Id {
    /// Creates a new unique window [`Id`].
    pub fn unique() -> Id {
        Id(COUNT.fetch_add(1, atomic::Ordering::Relaxed))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
```
