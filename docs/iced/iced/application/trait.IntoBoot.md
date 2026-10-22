---
title: "IntoBoot"
kind: trait
path: "iced::application::IntoBoot"
crate: "iced"
module: "iced::application"
file: "iced/application/trait.IntoBoot.md"
docs_url: "https://docs.iced.rs/iced/application/trait.IntoBoot.html"
source_file: "src/iced/application.rs"
source_lines: "568-571"
---

# `iced::application::IntoBoot`

```rust
pub trait IntoBoot<State, Message> {
    // Required method
    fn into_boot(self) -> (State, Task<Message>);
}
```

The initial state of some [`Application`](struct.Application.md).

**Source:** [application.rs](../../src/iced/application.rs.md) (lines 568-571)

## Required Methods

### `into_boot`

```rust
fn into_boot(self) -> (State, Task<Message>)
```

Turns some type into the initial state of some [`Application`](struct.Application.md).

## Implementors

- `impl<State, Message> IntoBoot<State, Message> for State`
