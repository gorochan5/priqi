---
title: "BootFn"
kind: trait
path: "iced::application::BootFn"
crate: "iced"
module: "iced::application"
file: "iced/application/trait.BootFn.md"
docs_url: "https://docs.iced.rs/iced/application/trait.BootFn.html"
source_file: "src/iced/application.rs"
source_lines: "552-555"
---

# `iced::application::BootFn`

```rust
pub trait BootFn<State, Message> {
    // Required method
    fn boot(&self) -> (State, Task<Message>);
}
```

The logic to initialize the `State` of some [`Application`](struct.Application.md).

This trait is implemented for both `Fn() -> State` and `Fn() -> (State, Task<Message>)`.

In practice, this means that [`application`](../fn.application.md) can both take simple functions like `State::default` and more advanced ones that return a [`Task`](../struct.Task.md).

**Source:** [application.rs](../../src/iced/application.rs.md) (lines 552-555)

## Required Methods

### `boot`

```rust
fn boot(&self) -> (State, Task<Message>)
```

Initializes the [`Application`](struct.Application.md) state.

## Implementors

- `impl<T, C, State, Message> BootFn<State, Message> for T where T: Fn() -> C, C: IntoBoot<State, Message>,`
