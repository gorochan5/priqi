---
title: "Catalog"
kind: trait
path: "iced_widget::float::Catalog"
crate: "iced_widget"
module: "iced_widget::float"
file: "iced_widget/float/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/float/trait.Catalog.html"
source_file: "src/iced_widget/float.rs"
source_lines: "355-364"
---

# `iced_widget::float::Catalog`

```rust
pub trait Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>) -> Style;
}
```

The theme catalog of a [`Float`](struct.Float.md).

All themes that can be used with [`Float`](struct.Float.md) must implement this trait.

**Source:** [float.rs](../../src/iced_widget/float.rs.md) (lines 355-364)

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
