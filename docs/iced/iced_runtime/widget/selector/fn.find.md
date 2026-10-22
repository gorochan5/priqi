---
title: "find"
kind: function
path: "iced_runtime::widget::selector::find"
crate: "iced_runtime"
module: "iced_runtime::widget::selector"
file: "iced_runtime/widget/selector/fn.find.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/selector/fn.find.html"
source_file: "src/iced_runtime/widget/selector.rs"
source_lines: "8-14"
---

# `iced_runtime::widget::selector::find`

```rust
pub fn find<S>(selector: S) -> Task<Option<S::Output>> where
    S: Selector + Send + 'static,
    S::Output: Send + Clone + 'static,
```

Finds a widget matching the given [`Selector`](trait.Selector.md).

**Source:** [selector.rs](../../../src/iced_runtime/widget/selector.rs.md) (lines 8-14)
