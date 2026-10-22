---
title: "center"
kind: function
path: "iced_widget::center"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.center.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.center.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "258-263"
---

# `iced_widget::center`

```rust
pub fn center<'a, W, Theme>(content: W) -> Container<'a, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Container`](container/struct.Container.md) that fills all the available space and centers its contents inside.

This is equivalent to:

```rust
let center = container("Center!").center(Fill);
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 258-263)
