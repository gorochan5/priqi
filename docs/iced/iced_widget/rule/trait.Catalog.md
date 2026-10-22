---
title: "Catalog"
kind: trait
path: "iced_widget::rule::Catalog"
crate: "iced_widget"
module: "iced_widget::rule"
file: "iced_widget/rule/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/rule/trait.Catalog.html"
source_file: "src/iced_widget/rule.rs"
source_lines: "264-273"
---

# `iced_widget::rule::Catalog`

```rust
pub trait Catalog: Sized {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>) -> Style;
}
```

The theme catalog of a [`Rule`](struct.Rule.md).

**Source:** [rule.rs](../../src/iced_widget/rule.rs.md) (lines 264-273)

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
