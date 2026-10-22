---
title: "StyleFn"
kind: type
path: "iced_widget::container::StyleFn"
crate: "iced_widget"
module: "iced_widget::container"
file: "iced_widget/container/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/container/type.StyleFn.html"
source_file: "src/iced_widget/container.rs"
source_lines: "488"
---

# `iced_widget::container::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`Container`](struct.Container.md).

**Source:** [container.rs](../../src/iced_widget/container.rs.md) (lines 488)

## Trait Implementations

### `impl<Theme> From<Style> for StyleFn<'_, Theme>`

#### `from`

```rust
fn from(style: Style) -> Self
```

Converts to this type from the input type.
