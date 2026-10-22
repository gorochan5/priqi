---
title: "Catalog"
kind: trait
path: "iced_widget::svg::Catalog"
crate: "iced_widget"
module: "iced_widget::svg"
file: "iced_widget/svg/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/svg/trait.Catalog.html"
source_file: "src/iced_widget/svg.rs"
source_lines: "291-300"
---

# `iced_widget::svg::Catalog`

```rust
pub trait Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}
```

The theme catalog of an [`Svg`](struct.Svg.md).

**Source:** [svg.rs](../../src/iced_widget/svg.rs.md) (lines 291-300)

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
