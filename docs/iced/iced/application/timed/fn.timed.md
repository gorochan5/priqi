---
title: "timed"
kind: function
path: "iced::application::timed::timed"
crate: "iced"
module: "iced::application::timed"
file: "iced/application/timed/fn.timed.md"
docs_url: "https://docs.iced.rs/iced/application/timed/fn.timed.html"
source_file: "src/iced/application/timed.rs"
source_lines: "21-127"
---

# `iced::application::timed::timed`

```rust
pub fn timed<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    subscription: impl Fn(&State) -> Subscription<Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = (Message, Instant), Theme = Theme>> where
    State: 'static,
    Message: Send + 'static,
    Theme: Base + 'static,
    Renderer: Renderer + 'static,
```

Creates an [`Application`](../struct.Application.md) with an `update` function that also takes the [`Instant`](../../time/struct.Instant.md) of each `Message`.

This constructor is useful to create animated applications that are *pure* (e.g. without relying on side-effect calls like [`Instant::now`](../../time/struct.Instant.md)).

Purity is needed when you want your application to end up in the same exact state given the same history of messages. This property enables proper time traveling debugging with [`comet`](https://github.com/iced-rs/comet).

**Source:** [timed.rs](../../../src/iced/application/timed.rs.md) (lines 21-127)
