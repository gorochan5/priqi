---
title: "find_all"
kind: function
path: "iced_runtime::widget::selector::find_all"
crate: "iced_runtime"
module: "iced_runtime::widget::selector"
file: "iced_runtime/widget/selector/fn.find_all.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/selector/fn.find_all.html"
source_file: "src/iced_runtime/widget/selector.rs"
source_lines: "17-23"
---

# `iced_runtime::widget::selector::find_all`

```rust
pub fn find_all<S>(selector: S) -> Task<Vec<S::Output>> where
    S: Selector + Send + 'static,
    S::Output: Send + Clone + 'static,
```

Finds all widgets matching the given [`Selector`](trait.Selector.md).

**Source:** [selector.rs](../../../src/iced_runtime/widget/selector.rs.md) (lines 17-23)
