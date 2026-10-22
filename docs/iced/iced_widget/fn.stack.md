---
title: "stack"
kind: function
path: "iced_widget::stack"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.stack.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.stack.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "520-525"
---

# `iced_widget::stack`

```rust
pub fn stack<W>(children: impl IntoIterator<Item = W>) -> Stack<W> where
    W: Meta,
```

Creates a new `Stack` with the given children.

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 520-525)
