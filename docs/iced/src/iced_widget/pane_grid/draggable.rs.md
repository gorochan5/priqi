---
title: "src/iced_widget/pane_grid/draggable.rs"
kind: source
path: "src/iced_widget/pane_grid/draggable.rs"
crate: "iced_widget"
file: "src/iced_widget/pane_grid/draggable.rs.md"
docs_url: "https://docs.iced.rs/src/iced_widget/pane_grid/draggable.rs.html"
source_file: "src/iced_widget/pane_grid/draggable.rs"
---

# `src/iced_widget/pane_grid/draggable.rs`

```rust
use crate::core::widget::Tree;
use crate::core::{Layout, Point};

/// A pane that can be dragged.
pub trait Draggable {
    /// Returns whether the [`Draggable`] with the given [`Layout`] can be picked
    /// at the provided cursor position.
    fn can_be_dragged_at(&self, tree: &Tree, layout: Layout, cursor: Point) -> bool;
}
```
