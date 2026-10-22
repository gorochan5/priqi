---
title: "daemon"
kind: function
path: "iced::daemon"
crate: "iced"
module: "iced"
file: "iced/fn.daemon.md"
docs_url: "https://docs.iced.rs/iced/fn.daemon.html"
source_file: "src/iced/daemon.rs"
source_lines: "26-109"
---

# `iced::daemon`

```rust
pub fn daemon<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Daemon<impl Program<State = State, Message = Message, Theme = Theme>> where
    State: 'static,
    Message: Send + 'static,
    Theme: Base,
    Renderer: Renderer,
```

Creates an iced [`Daemon`](daemon/struct.Daemon.md) given its boot, update, and view logic.

A [`Daemon`](daemon/struct.Daemon.md) will not open a window by default, but will run silently instead until a [`Task`](struct.Task.md) from [`window::open`](window/fn.open.md) is returned by its update logic.

Furthermore, a [`Daemon`](daemon/struct.Daemon.md) will not stop running when all its windows are closed. In order to completely terminate a [`Daemon`](daemon/struct.Daemon.md), its process must be interrupted or its update logic must produce a [`Task`](struct.Task.md) from [`exit`](fn.exit.md).

**Source:** [daemon.rs](../src/iced/daemon.rs.md) (lines 26-109)
