---
title: "StyleFn"
kind: type
path: "iced_widget::svg::StyleFn"
crate: "iced_widget"
module: "iced_widget::svg"
file: "iced_widget/svg/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/svg/type.StyleFn.html"
source_file: "src/iced_widget/svg.rs"
source_lines: "317"
---

# `iced_widget::svg::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for an [`Svg`](struct.Svg.md).

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.

**Source:** [svg.rs](../../src/iced_widget/svg.rs.md) (lines 317)

## Trait Implementations

### `impl<Theme> From<Style> for StyleFn<'_, Theme>`

#### `from`

```rust
fn from(style: Style) -> Self
```

Converts to this type from the input type.
