---
title: "center_y"
kind: function
path: "iced_widget::center_y"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.center_y.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.center_y.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "296-301"
---

# `iced_widget::center_y`

```rust
pub fn center_y<'a, W, Theme>(content: W) -> Container<'a, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Container`](container/struct.Container.md) that fills all the available space vertically and centers its contents inside.

This is equivalent to:

```rust
let center_y = container("Vertical Center!").center_y(Fill);
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 296-301)
