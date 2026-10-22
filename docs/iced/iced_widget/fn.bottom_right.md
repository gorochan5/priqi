---
title: "bottom_right"
kind: function
path: "iced_widget::bottom_right"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.bottom_right.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.bottom_right.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "395-402"
---

# `iced_widget::bottom_right`

```rust
pub fn bottom_right<'a, W, Theme>(content: W) -> Container<'a, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Container`](container/struct.Container.md) that fills all the available space and aligns its contents inside to the bottom right corner.

This is equivalent to:

```rust
let bottom_right = container("Bottom!").align_right(Fill).align_bottom(Fill);
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 395-402)
