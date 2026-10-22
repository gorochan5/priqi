---
title: "Catalog"
kind: trait
path: "iced_widget::pick_list::Catalog"
crate: "iced_widget"
module: "iced_widget::pick_list"
file: "iced_widget/pick_list/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/pick_list/trait.Catalog.html"
source_file: "src/iced_widget/pick_list.rs"
source_lines: "874-888"
---

# `iced_widget::pick_list::Catalog`

```rust
pub trait Catalog: Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> <Self as Catalog>::Class<'a>;
    fn style(
        &self,
        class: &<Self as Catalog>::Class<'_>,
        status: Status,
    ) -> Style;

    // Provided method
    fn default_menu<'a>() -> <Self as Catalog>::Class<'a> { ... }
}
```

The theme catalog of a [`PickList`](struct.PickList.md).

**Source:** [pick_list.rs](../../src/iced_widget/pick_list.rs.md) (lines 874-888)

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
fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style
```

The [`Style`](struct.Style.md) of a class with the given status.

## Provided Methods

### `default_menu`

```rust
fn default_menu<'a>() -> <Self as Catalog>::Class<'a>
```

The default class for the menu of the [`PickList`](struct.PickList.md).

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>`
