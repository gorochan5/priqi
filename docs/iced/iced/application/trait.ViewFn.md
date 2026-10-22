---
title: "ViewFn"
kind: trait
path: "iced::application::ViewFn"
crate: "iced"
module: "iced::application"
file: "iced/application/trait.ViewFn.md"
docs_url: "https://docs.iced.rs/iced/application/trait.ViewFn.html"
source_file: "src/iced/application.rs"
source_lines: "680-686"
---

# `iced::application::ViewFn`

```rust
pub trait ViewFn<'a, State, Message, Theme, Renderer> where
    Renderer: Renderer, {
    // Required method
    fn view(
        &self,
        state: &'a State,
    ) -> impl Widget<Message, Theme, Renderer> + 'a;
}
```

The view logic of some [`Application`](struct.Application.md).

This trait allows the [`application`](../fn.application.md) builder to take any closure that returns any widget.

**Source:** [application.rs](../../src/iced/application.rs.md) (lines 680-686)

## Required Methods

### `view`

```rust
fn view(&self, state: &'a State) -> impl Widget<Message, Theme, Renderer> + 'a
```

Produces the widget of the [`Application`](struct.Application.md).

## Implementors

- `impl<'a, T, State, Message, Theme, Renderer, W> ViewFn<'a, State, Message, Theme, Renderer> for T where T: Fn(&'a State) -> W, State: 'static, Message: 'static, Theme: 'static, Renderer: 'static + Renderer, W: Widget<Message, Theme, Renderer> + 'a,`
