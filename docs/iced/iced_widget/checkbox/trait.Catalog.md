---
title: "Catalog"
kind: trait
path: "iced_widget::checkbox::Catalog"
crate: "iced_widget"
module: "iced_widget::checkbox"
file: "iced_widget/checkbox/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/checkbox/trait.Catalog.html"
source_file: "src/iced_widget/checkbox.rs"
source_lines: "551-563"
---

# `iced_widget::checkbox::Catalog`

```rust
pub trait Catalog: Sized {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
    fn selection(&self) -> Color;
}
```

The theme catalog of a [`Checkbox`](struct.Checkbox.md).

**Source:** [checkbox.rs](../../src/iced_widget/checkbox.rs.md) (lines 551-563)

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
