---
title: "StyleFn"
kind: type
path: "iced_widget::pane_grid::StyleFn"
crate: "iced_widget"
module: "iced_widget::pane_grid"
file: "iced_widget/pane_grid/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/pane_grid/type.StyleFn.html"
source_file: "src/iced_widget/pane_grid.rs"
source_lines: "1265"
---

# `iced_widget::pane_grid::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`PaneGrid`](struct.PaneGrid.md).

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.

**Source:** [pane_grid.rs](../../src/iced_widget/pane_grid.rs.md) (lines 1265)
