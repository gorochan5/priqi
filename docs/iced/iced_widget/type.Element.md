---
title: "Element"
kind: type
path: "iced_widget::Element"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/type.Element.md"
docs_url: "https://docs.iced.rs/iced_widget/type.Element.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "172-173"
---

# `iced_widget::Element`

```rust
pub type Element<'a, Message, Theme = Theme, Renderer = Renderer> = Element<'a, Message, Theme, Renderer>;
```

A boxed [`Widget`](trait.Widget.md).

This is an alias of [`core::Element`] with default `Theme` and `Renderer` parameters.

**Source:** [lib.rs](../src/iced_widget/lib.rs.md) (lines 172-173)
