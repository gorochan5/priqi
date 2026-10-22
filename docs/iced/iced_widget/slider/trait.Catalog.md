---
title: "Catalog"
kind: trait
path: "iced_widget::slider::Catalog"
crate: "iced_widget"
module: "iced_widget::slider"
file: "iced_widget/slider/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/slider/trait.Catalog.html"
source_file: "src/iced_widget/slider.rs"
source_lines: "609-618"
---

# `iced_widget::slider::Catalog`

```rust
pub trait Catalog: Sized {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}
```

The theme catalog of a [`Slider`](struct.Slider.md).

**Source:** [slider.rs](../../src/iced_widget/slider.rs.md) (lines 609-618)

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
