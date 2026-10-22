---
title: "UpdateFn"
kind: trait
path: "iced::application::timed::UpdateFn"
crate: "iced"
module: "iced::application::timed"
file: "iced/application/timed/trait.UpdateFn.md"
docs_url: "https://docs.iced.rs/iced/application/timed/trait.UpdateFn.html"
source_file: "src/iced/application/timed.rs"
source_lines: "133-137"
---

# `iced::application::timed::UpdateFn`

```rust
pub trait UpdateFn<State, Message> {
    // Required method
    fn update(
        &self,
        state: &mut State,
        message: Message,
        now: Instant,
    ) -> impl Into<Task<Message>>;
}
```

The update logic of some timed [`Application`](../struct.Application.md).

This is like [`application::UpdateFn`](../trait.UpdateFn.md), but it also takes an [`Instant`](../../time/struct.Instant.md).

**Source:** [timed.rs](../../../src/iced/application/timed.rs.md) (lines 133-137)

## Required Methods

### `update`

```rust
fn update( &self, state: &mut State, message: Message, now: Instant, ) -> impl Into<Task<Message>>
```

Processes the message and updates the state of the [`Application`](../struct.Application.md).

## Implementors

- `impl<T, State, Message, C> UpdateFn<State, Message> for T where T: Fn(&mut State, Message, Instant) -> C, C: Into<Task<Message>>,`
