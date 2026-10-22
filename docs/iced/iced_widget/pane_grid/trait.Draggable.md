---
title: "Draggable"
kind: trait
path: "iced_widget::pane_grid::Draggable"
crate: "iced_widget"
module: "iced_widget::pane_grid"
file: "iced_widget/pane_grid/trait.Draggable.md"
docs_url: "https://docs.iced.rs/iced_widget/pane_grid/trait.Draggable.html"
source_file: "src/iced_widget/pane_grid/draggable.rs"
source_lines: "5-9"
---

# `iced_widget::pane_grid::Draggable`

```rust
pub trait Draggable {
    // Required method
    fn can_be_dragged_at(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: Point,
    ) -> bool;
}
```

A pane that can be dragged.

**Source:** [draggable.rs](../../src/iced_widget/pane_grid/draggable.rs.md) (lines 5-9)

## Required Methods

### `can_be_dragged_at`

```rust
fn can_be_dragged_at(&self, tree: &Tree, layout: Layout, cursor: Point) -> bool
```

Returns whether the [`Draggable`](trait.Draggable.md) with the given [`Layout`] can be picked at the provided cursor position.

## Implementors

- `impl<Message, T, W, Theme, Renderer> Draggable for &Content<'_, Message, T, W, Theme, Renderer> where Theme: Catalog, Renderer: Renderer, T: Widget<Message, Theme, Renderer>,`
