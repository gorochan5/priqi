---
title: "src/iced_core/widget/id.rs"
kind: source
path: "src/iced_core/widget/id.rs"
crate: "iced_core"
file: "src/iced_core/widget/id.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/widget/id.rs.html"
source_file: "src/iced_core/widget/id.rs"
---

# `src/iced_core/widget/id.rs`

```rust
use std::borrow;
use std::sync::atomic::{self, AtomicUsize};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// The identifier of a generic widget.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(Internal);

impl Id {
    /// Creates a new [`Id`] from a static `str`.
    pub const fn new(id: &'static str) -> Self {
        Self(Internal::Custom(borrow::Cow::Borrowed(id)))
    }

    /// Creates a unique [`Id`].
    ///
    /// This function produces a different [`Id`] every time it is called.
    pub fn unique() -> Self {
        let id = NEXT_ID.fetch_add(1, atomic::Ordering::Relaxed);

        Self(Internal::Unique(id))
    }
}

impl From<&'static str> for Id {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self(Internal::Custom(borrow::Cow::Owned(value)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Internal {
    Unique(usize),
    Custom(borrow::Cow<'static, str>),
}

#[cfg(test)]
mod tests {
    use super::Id;

    #[test]
    fn unique_generates_different_ids() {
        let a = Id::unique();
        let b = Id::unique();

        assert_ne!(a, b);
    }
}
```
