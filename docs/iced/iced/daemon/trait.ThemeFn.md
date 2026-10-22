---
title: "ThemeFn"
kind: trait
path: "iced::daemon::ThemeFn"
crate: "iced"
module: "iced::daemon"
file: "iced/daemon/trait.ThemeFn.md"
docs_url: "https://docs.iced.rs/iced/daemon/trait.ThemeFn.html"
source_file: "src/iced/daemon.rs"
source_lines: "417-423"
---

# `iced::daemon::ThemeFn`

```rust
pub trait ThemeFn<State, Theme> {
    // Required method
    fn theme(&self, state: &State, window: Id) -> Option<Theme>;
}
```

The theme logic of some [`Daemon`](struct.Daemon.md).

Any implementors of this trait can be provided as an argument to [`Daemon::theme`](struct.Daemon.md).

`iced` provides two implementors:

- the built-in [`Theme`](../enum.Theme.md) itself
- and any `Fn(&State, window::Id) -> impl Into<Option<Theme>>`.

**Source:** [daemon.rs](../../src/iced/daemon.rs.md) (lines 417-423)

## Required Methods

### `theme`

```rust
fn theme(&self, state: &State, window: Id) -> Option<Theme>
```

Returns the theme of the [`Daemon`](struct.Daemon.md) for the current state and window.

If `None` is returned, `iced` will try to use a theme that matches the system color scheme.

## Implementors

- `impl<F, T, State, Theme> ThemeFn<State, Theme> for F where F: Fn(&State, Id) -> T, T: Into<Option<Theme>>,`
- `impl<State> ThemeFn<State, Theme> for Theme`
