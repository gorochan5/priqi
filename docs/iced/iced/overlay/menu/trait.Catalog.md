---
title: "Catalog"
kind: trait
path: "iced::overlay::menu::Catalog"
crate: "iced"
module: "iced::overlay::menu"
file: "iced/overlay/menu/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced/overlay/menu/trait.Catalog.html"
source_file: "src/iced_widget/overlay/menu.rs"
source_lines: "661"
---

# `iced::overlay::menu::Catalog`

```rust
pub trait Catalog: Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>) -> Style;

    // Provided method
    fn default_scrollable<'a>() -> Self::Class<'a> { ... }
}
```

The theme catalog of a [`Menu`](struct.Menu.md).

**Source:** [menu.rs](../../../src/iced_widget/overlay/menu.rs.md) (lines 661)

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

## Provided Methods

### `default_scrollable`

```rust
fn default_scrollable<'a>() -> Self::Class<'a>
```

The default class for the scrollable of the [`Menu`](struct.Menu.md).

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>`
