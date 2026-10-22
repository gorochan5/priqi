---
title: "Catalog"
kind: trait
path: "iced_widget::container::Catalog"
crate: "iced_widget"
module: "iced_widget::container"
file: "iced_widget/container/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/container/trait.Catalog.html"
source_file: "src/iced_widget/container.rs"
source_lines: "476-485"
---

# `iced_widget::container::Catalog`

```rust
pub trait Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>) -> Style;
}
```

The theme catalog of a [`Container`](struct.Container.md).

**Source:** [container.rs](../../src/iced_widget/container.rs.md) (lines 476-485)

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
