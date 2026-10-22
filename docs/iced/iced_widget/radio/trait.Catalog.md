---
title: "Catalog"
kind: trait
path: "iced_widget::radio::Catalog"
crate: "iced_widget"
module: "iced_widget::radio"
file: "iced_widget/radio/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/radio/trait.Catalog.html"
source_file: "src/iced_widget/radio.rs"
source_lines: "481-493"
---

# `iced_widget::radio::Catalog`

```rust
pub trait Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
    fn selection(&self) -> Color;
}
```

The theme catalog of a [`Radio`](struct.Radio.md).

**Source:** [radio.rs](../../src/iced_widget/radio.rs.md) (lines 481-493)

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

### `selection`

```rust
fn selection(&self) -> Color
```

The global selection [`Color`].

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>`
