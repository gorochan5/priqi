---
title: "TitleFn"
kind: trait
path: "iced::daemon::TitleFn"
crate: "iced"
module: "iced::daemon"
file: "iced/daemon/trait.TitleFn.md"
docs_url: "https://docs.iced.rs/iced/daemon/trait.TitleFn.html"
source_file: "src/iced/daemon.rs"
source_lines: "356-359"
---

# `iced::daemon::TitleFn`

```rust
pub trait TitleFn<State> {
    // Required method
    fn title(&self, state: &State, window: Id) -> String;
}
```

The title logic of some [`Daemon`](struct.Daemon.md).

This trait is implemented both for `&static str` and any closure `Fn(&State, window::Id) -> String`.

This trait allows the [`daemon`](../fn.daemon.md) builder to take any of them.

**Source:** [daemon.rs](../../src/iced/daemon.rs.md) (lines 356-359)

## Required Methods

### `title`

```rust
fn title(&self, state: &State, window: Id) -> String
```

Produces the title of the [`Daemon`](struct.Daemon.md).

## Implementors

- `impl<T, State> TitleFn<State> for T where T: Fn(&State, Id) -> String,`
