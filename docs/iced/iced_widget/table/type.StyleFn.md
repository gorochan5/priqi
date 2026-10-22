---
title: "StyleFn"
kind: type
path: "iced_widget::table::StyleFn"
crate: "iced_widget"
module: "iced_widget::table"
file: "iced_widget/table/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/table/type.StyleFn.html"
source_file: "src/iced_widget/table.rs"
source_lines: "637"
---

# `iced_widget::table::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`Table`](struct.Table.md).

**Source:** [table.rs](../../src/iced_widget/table.rs.md) (lines 637)

## Trait Implementations

### `impl<Theme> From<Style> for StyleFn<'_, Theme>`

#### `from`

```rust
fn from(style: Style) -> Self
```

Converts to this type from the input type.
