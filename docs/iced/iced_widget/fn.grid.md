---
title: "grid"
kind: function
path: "iced_widget::grid"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.grid.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.grid.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "510-515"
---

# `iced_widget::grid`

```rust
pub fn grid<W>(children: impl IntoIterator<Item = W>) -> Grid<W> where
    W: Meta,
```

Creates a new [`Grid`](grid/struct.Grid.md) from an iterator.

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 510-515)
