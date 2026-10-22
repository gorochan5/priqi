---
title: "Element"
kind: type
path: "iced::Element"
crate: "iced"
module: "iced"
file: "iced/type.Element.md"
docs_url: "https://docs.iced.rs/iced/type.Element.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "172"
---

# `iced::Element`

```rust
pub type Element<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> = Element<'a, Message, Theme, Renderer>;
```

A boxed [`Widget`](trait.Widget.md).

This is an alias of [`core::Element`](advanced/widget/struct.Element.md) with default `Theme` and `Renderer` parameters.

**Source:** [lib.rs](../src/iced_widget/lib.rs.md) (lines 172)
