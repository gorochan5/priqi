---
title: "UpdateFn"
kind: trait
path: "iced::application::UpdateFn"
crate: "iced"
module: "iced::application"
file: "iced/application/trait.UpdateFn.md"
docs_url: "https://docs.iced.rs/iced/application/trait.UpdateFn.html"
source_file: "src/iced/application.rs"
source_lines: "655-658"
---

# `iced::application::UpdateFn`

```rust
pub trait UpdateFn<State, Message> {
    // Required method
    fn update(&self, state: &mut State, message: Message) -> Task<Message>;
}
```

The update logic of some [`Application`](struct.Application.md).

This trait allows the [`application`](../fn.application.md) builder to take any closure that returns any `Into<Task<Message>>`.

**Source:** [application.rs](../../src/iced/application.rs.md) (lines 655-658)

## Required Methods

### `update`

```rust
fn update(&self, state: &mut State, message: Message) -> Task<Message>
```

Processes the message and updates the state of the [`Application`](struct.Application.md).

## Implementors

- `impl<T, State, Message, C> UpdateFn<State, Message> for T where T: Fn(&mut State, Message) -> C, C: Into<Task<Message>>,`
