---
title: "deviate"
kind: function
path: "iced_widget::theme::palette::deviate"
crate: "iced_widget"
module: "iced_widget::theme::palette"
file: "iced_widget/theme/palette/fn.deviate.md"
docs_url: "https://docs.iced.rs/iced_widget/theme/palette/fn.deviate.html"
---

# `iced_widget::theme::palette::deviate`

```rust
pub fn deviate(color: Color, amount: f32) -> Color
```

Deviates a [`Color`] by the given factor. Lightens if the [`Color`] is dark, darkens otherwise.
