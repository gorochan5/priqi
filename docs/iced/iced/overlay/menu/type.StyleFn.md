---
title: "StyleFn"
kind: type
path: "iced::overlay::menu::StyleFn"
crate: "iced"
module: "iced::overlay::menu"
file: "iced/overlay/menu/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced/overlay/menu/type.StyleFn.html"
source_file: "src/iced_widget/overlay/menu.rs"
source_lines: "678"
---

# `iced::overlay::menu::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`Menu`](struct.Menu.md).

**Source:** [menu.rs](../../../src/iced_widget/overlay/menu.rs.md) (lines 678)
