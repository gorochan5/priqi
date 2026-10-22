---
title: "bottom"
kind: function
path: "iced_widget::bottom"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.bottom.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.bottom.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "355-360"
---

# `iced_widget::bottom`

```rust
pub fn bottom<'a, W, Theme>(content: W) -> Container<'a, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Container`](container/struct.Container.md) that fills all the available space vertically and bottom-aligns its contents inside.

This is equivalent to:

```rust
let bottom = container("Bottom!").align_bottom(Fill);
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 355-360)
