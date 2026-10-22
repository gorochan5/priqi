---
title: "src/iced_runtime/widget/selector.rs"
kind: source
path: "src/iced_runtime/widget/selector.rs"
crate: "iced_runtime"
file: "src/iced_runtime/widget/selector.rs.md"
docs_url: "https://docs.iced.rs/src/iced_runtime/widget/selector.rs.html"
source_file: "src/iced_runtime/widget/selector.rs"
---

# `src/iced_runtime/widget/selector.rs`

```rust
//! Find and query widgets in your applications.
pub use iced_selector::{Bounded, Candidate, Selector, Target, Text, id, is_focused};

use crate::Task;
use crate::task;

/// Finds a widget matching the given [`Selector`].
pub fn find<S>(selector: S) -> Task<Option<S::Output>>
where
    S: Selector + Send + 'static,
    S::Output: Send + Clone + 'static,
{
    task::widget(selector.find())
}

/// Finds all widgets matching the given [`Selector`].
pub fn find_all<S>(selector: S) -> Task<Vec<S::Output>>
where
    S: Selector + Send + 'static,
    S::Output: Send + Clone + 'static,
{
    task::widget(selector.find_all())
}
```
