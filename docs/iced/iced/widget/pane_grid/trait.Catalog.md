---
title: "Catalog"
kind: trait
path: "iced::widget::pane_grid::Catalog"
crate: "iced"
module: "iced::widget::pane_grid"
file: "iced/widget/pane_grid/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced/widget/pane_grid/trait.Catalog.html"
source_file: "src/iced_widget/pane_grid.rs"
source_lines: "1251"
---

# `iced::widget::pane_grid::Catalog`

```rust
pub trait Catalog: Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>) -> Style;
}
```

The theme catalog of a [`PaneGrid`](../struct.PaneGrid.md).

**Source:** [pane_grid.rs](../../../src/iced_widget/pane_grid.rs.md) (lines 1251)

## Required Associated Types

### `Class`

```rust
type Class<'a>
```

The item class of this [`Catalog`](trait.Catalog.md).

## Required Methods

### `default`

```rust
fn default<'a>() -> Self::Class<'a>
```

The default class produced by this [`Catalog`](trait.Catalog.md).

### `style`

```rust
fn style(&self, class: &Self::Class<'_>) -> Style
```

The [`Style`](struct.Style.md) of a class with the given status.

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>`
