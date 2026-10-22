---
title: "ViewFn"
kind: trait
path: "iced::daemon::ViewFn"
crate: "iced"
module: "iced::daemon"
file: "iced/daemon/trait.ViewFn.md"
docs_url: "https://docs.iced.rs/iced/daemon/trait.ViewFn.html"
source_file: "src/iced/daemon.rs"
source_lines: "380-390"
---

# `iced::daemon::ViewFn`

```rust
pub trait ViewFn<'a, State, Message, Theme, Renderer> where
    Renderer: Renderer, {
    // Required method
    fn view(
        &self,
        state: &'a State,
        window: Id,
    ) -> impl Widget<Message, Theme, Renderer> + 'a;
}
```

The view logic of some [`Daemon`](struct.Daemon.md).

This trait allows the [`daemon`](../fn.daemon.md) builder to take any closure that returns any widget.

**Source:** [daemon.rs](../../src/iced/daemon.rs.md) (lines 380-390)

## Required Methods

### `view`

```rust
fn view( &self, state: &'a State, window: Id, ) -> impl Widget<Message, Theme, Renderer> + 'a
```

Produces the widget of the [`Daemon`](struct.Daemon.md).

## Implementors

- `impl<'a, T, State, Message, Theme, Renderer, W> ViewFn<'a, State, Message, Theme, Renderer> for T where T: Fn(&'a State, Id) -> W, State: 'static, W: Widget<Message, Theme, Renderer> + 'a, Renderer: Renderer,`
