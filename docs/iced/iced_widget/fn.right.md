---
title: "right"
kind: function
path: "iced_widget::right"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.right.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.right.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "315-320"
---

# `iced_widget::right`

```rust
pub fn right<'a, W, Theme>(content: W) -> Container<'a, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Container`](container/struct.Container.md) that fills all the available space horizontally and right-aligns its contents inside.

This is equivalent to:

```rust
let right = container("Right!").align_right(Fill);
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 315-320)
