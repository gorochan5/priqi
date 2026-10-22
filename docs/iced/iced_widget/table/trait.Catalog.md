---
title: "Catalog"
kind: trait
path: "iced_widget::table::Catalog"
crate: "iced_widget"
module: "iced_widget::table"
file: "iced_widget/table/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/table/trait.Catalog.html"
source_file: "src/iced_widget/table.rs"
source_lines: "625-634"
---

# `iced_widget::table::Catalog`

```rust
pub trait Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>) -> Style;
}
```

The theme catalog of a [`Table`](struct.Table.md).

**Source:** [table.rs](../../src/iced_widget/table.rs.md) (lines 625-634)

## Required Associated Types

### `Class`

```rust
type Class<'a>
```

The item class of the [`Catalog`](trait.Catalog.md).

## Required Methods

### `default`

```rust
fn default<'a>() -> Self::Class<'a>
```

The default class produced by the [`Catalog`](trait.Catalog.md).

### `style`

```rust
fn style(&self, class: &Self::Class<'_>) -> Style
```

The [`Style`](struct.Style.md) of a class with the given status.

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>`
