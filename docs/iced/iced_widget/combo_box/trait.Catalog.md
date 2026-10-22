---
title: "Catalog"
kind: trait
path: "iced_widget::combo_box::Catalog"
crate: "iced_widget"
module: "iced_widget::combo_box"
file: "iced_widget/combo_box/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/combo_box/trait.Catalog.html"
source_file: "src/iced_widget/combo_box.rs"
source_lines: "756-766"
---

# `iced_widget::combo_box::Catalog`

```rust
pub trait Catalog: Catalog + Catalog {
    // Provided methods
    fn default_input<'a>() -> <Self as Catalog>::Class<'a> { ... }
    fn default_menu<'a>() -> <Self as Catalog>::Class<'a> { ... }
}
```

The theme catalog of a [`ComboBox`](struct.ComboBox.md).

**Source:** [combo_box.rs](../../src/iced_widget/combo_box.rs.md) (lines 756-766)

## Provided Methods

### `default_input`

```rust
fn default_input<'a>() -> <Self as Catalog>::Class<'a>
```

The default class for the text input of the [`ComboBox`](struct.ComboBox.md).

### `default_menu`

```rust
fn default_menu<'a>() -> <Self as Catalog>::Class<'a>
```

The default class for the menu of the [`ComboBox`](struct.ComboBox.md).

## Implementors

- `impl Catalog for Theme`
