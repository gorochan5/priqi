---
title: "Catalog"
kind: trait
path: "iced_widget::scrollable::Catalog"
crate: "iced_widget"
module: "iced_widget::scrollable"
file: "iced_widget/scrollable/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/scrollable/trait.Catalog.html"
source_file: "src/iced_widget/scrollable.rs"
source_lines: "3531-3540"
---

# `iced_widget::scrollable::Catalog`

```rust
pub trait Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}
```

The theme catalog of a [`Scrollable`](struct.Scrollable.md).

**Source:** [scrollable.rs](../../src/iced_widget/scrollable.rs.md) (lines 3531-3540)

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
fn style(&self, class: &Self::Class<'_>, status: Status) -> Style
```

The [`Style`](struct.Style.md) of a class with the given status.

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>`
