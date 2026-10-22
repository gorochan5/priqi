---
title: "center_x"
kind: function
path: "iced_widget::center_x"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.center_x.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.center_x.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "277-282"
---

# `iced_widget::center_x`

```rust
pub fn center_x<'a, W, Theme>(content: W) -> Container<'a, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Container`](container/struct.Container.md) that fills all the available space horizontally and centers its contents inside.

This is equivalent to:

```rust
let center_x = container("Horizontal Center!").center_x(Fill);
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 277-282)
