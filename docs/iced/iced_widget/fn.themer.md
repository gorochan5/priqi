---
title: "themer"
kind: function
path: "iced_widget::themer"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.themer.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.themer.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1662-1671"
---

# `iced_widget::themer`

```rust
pub fn themer<'a, Message, Theme, Renderer>(
    theme: Option<Theme>,
    content: impl Widget<Message, Theme, Renderer> + 'a,
) -> Themer<'a, Message, Theme, Renderer> where
    Theme: Base,
    Renderer: Renderer,
```

A widget that applies any `Theme` to its contents.

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1662-1671)
