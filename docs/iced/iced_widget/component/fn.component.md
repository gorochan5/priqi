---
title: "component"
kind: function
path: "iced_widget::component::component"
crate: "iced_widget"
module: "iced_widget::component"
file: "iced_widget/component/fn.component.md"
docs_url: "https://docs.iced.rs/iced_widget/component/fn.component.html"
source_file: "src/iced_widget/component.rs"
source_lines: "115-132"
---

# `iced_widget::component::component`

```rust
pub fn component<'a, C, Message, Theme, Renderer>(
    component: C,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    C: Component<'a, Message, Theme, Renderer> + 'a,
    C::State: 'static,
    Message: 'a,
    Theme: 'a,
    Renderer: Renderer + 'a,
```

Turns an implementor of [`Component`](trait.Component.md) into an [`Element`] that can be embedded in any application.

**Source:** [component.rs](../../src/iced_widget/component.rs.md) (lines 115-132)
