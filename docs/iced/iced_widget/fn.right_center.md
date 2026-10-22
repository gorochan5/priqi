---
title: "right_center"
kind: function
path: "iced_widget::right_center"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.right_center.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.right_center.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "334-341"
---

# `iced_widget::right_center`

```rust
pub fn right_center<'a, W, Theme>(content: W) -> Container<'a, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Container`](container/struct.Container.md) that fills all the available space and aligns its contents inside to the right center.

This is equivalent to:

```rust
let right_center = container("Bottom Center!").align_right(Fill).center_y(Fill);
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 334-341)
