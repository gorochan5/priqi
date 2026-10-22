---
title: "bottom_center"
kind: function
path: "iced_widget::bottom_center"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.bottom_center.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.bottom_center.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "374-381"
---

# `iced_widget::bottom_center`

```rust
pub fn bottom_center<'a, W, Theme>(content: W) -> Container<'a, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Container`](container/struct.Container.md) that fills all the available space and aligns its contents inside to the bottom center.

This is equivalent to:

```rust
let bottom_center = container("Bottom Center!").center_x(Fill).align_bottom(Fill);
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 374-381)
