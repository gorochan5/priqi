---
title: "Catalog"
kind: trait
path: "iced_core::widget::text::Catalog"
crate: "iced_core"
module: "iced_core::widget::text"
file: "iced_core/widget/text/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_core/widget/text/trait.Catalog.html"
source_file: "src/iced_core/widget/text.rs"
source_lines: "618-630"
---

# `iced_core::widget::text::Catalog`

```rust
pub trait Catalog: Sized {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, item: &Self::Class<'_>) -> Style;
    fn selection(&self) -> Color;
}
```

The theme catalog of a [`Text`](struct.Text.md).

**Source:** [text.rs](../../../src/iced_core/widget/text.rs.md) (lines 618-630)

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

The global selection [`Color`](../../color/struct.Color.md).

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>`
