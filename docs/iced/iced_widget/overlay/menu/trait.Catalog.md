---
title: "Catalog"
kind: trait
path: "iced_widget::overlay::menu::Catalog"
crate: "iced_widget"
module: "iced_widget::overlay::menu"
file: "iced_widget/overlay/menu/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/overlay/menu/trait.Catalog.html"
source_file: "src/iced_widget/overlay/menu.rs"
source_lines: "661-675"
---

# `iced_widget::overlay::menu::Catalog`

```rust
pub trait Catalog: Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> <Self as Catalog>::Class<'a>;
    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style;

    // Provided method
    fn default_scrollable<'a>() -> <Self as Catalog>::Class<'a> { ... }
}
```

The theme catalog of a [`Menu`](struct.Menu.md).

**Source:** [menu.rs](../../../src/iced_widget/overlay/menu.rs.md) (lines 661-675)

## Required Associated Types

### `Class`

```rust
type Class<'a>
```

The item class of the [`Catalog`](trait.Catalog.md).

## Required Methods

### `default`

```rust
fn default<'a>() -> <Self as Catalog>::Class<'a>
```

The default class produced by the [`Catalog`](trait.Catalog.md).

### `style`

```rust
fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style
```

The [`Style`](struct.Style.md) of a class with the given status.

## Provided Methods

### `default_scrollable`

```rust
fn default_scrollable<'a>() -> <Self as Catalog>::Class<'a>
```

The default class for the scrollable of the [`Menu`](struct.Menu.md).

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>`
