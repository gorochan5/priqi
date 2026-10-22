---
title: "Catalog"
kind: trait
path: "iced_widget::text_input::Catalog"
crate: "iced_widget"
module: "iced_widget::text_input"
file: "iced_widget/text_input/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/text_input/trait.Catalog.html"
source_file: "src/iced_widget/text_input.rs"
source_lines: "551-560"
---

# `iced_widget::text_input::Catalog`

```rust
pub trait Catalog: Sized {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}
```

The theme catalog of a [`TextInput`](struct.TextInput.md).

**Source:** [text_input.rs](../../src/iced_widget/text_input.rs.md) (lines 551-560)

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
