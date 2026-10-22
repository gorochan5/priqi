---
title: "Catalog"
kind: trait
path: "iced_widget::text::Catalog"
crate: "iced_widget"
module: "iced_widget::text"
file: "iced_widget/text/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/text/trait.Catalog.html"
---

# `iced_widget::text::Catalog`

```rust
pub trait Catalog: Sized {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, item: &Self::Class<'_>) -> Style;
    fn selection(&self) -> Color;
}
```

The theme catalog of a [`Text`].

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
fn style(&self, item: &Self::Class<'_>) -> Style
```

The [`Style`](struct.Style.md) of a class with the given status.

### `selection`

```rust
fn selection(&self) -> Color
```

The global selection [`Color`].

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>`
