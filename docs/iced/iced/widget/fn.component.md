---
title: "component"
kind: function
path: "iced::widget::component"
crate: "iced"
module: "iced::widget"
file: "iced/widget/fn.component.md"
docs_url: "https://docs.iced.rs/iced/widget/fn.component.html"
source_file: "src/iced_widget/component.rs"
source_lines: "115-123"
---

# `iced::widget::component`

```rust
pub fn component<'a, C, Message, Theme, Renderer>(
    component: C,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    C: Component<'a, Message, Theme, Renderer> + 'a,
    <C as Component<'a, Message, Theme, Renderer>>::State: 'static,
    Message: 'a,
    Theme: 'a,
    Renderer: Renderer + 'a,
```

Turns an implementor of [`Component`](trait.Component.md) into an [`Element`](../advanced/widget/struct.Element.md) that can be embedded in any application.

**Source:** [component.rs](../../src/iced_widget/component.rs.md) (lines 115-123)
