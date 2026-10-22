---
title: "ThemeFn"
kind: trait
path: "iced::application::ThemeFn"
crate: "iced"
module: "iced::application"
file: "iced/application/trait.ThemeFn.md"
docs_url: "https://docs.iced.rs/iced/application/trait.ThemeFn.html"
source_file: "src/iced/application.rs"
source_lines: "621-630"
---

# `iced::application::ThemeFn`

```rust
pub trait ThemeFn<State, Theme> where
    Theme: Base, {
    // Required method
    fn theme(&self, state: &State) -> Option<Theme>;
}
```

The theme logic of some [`Application`](struct.Application.md).

Any implementors of this trait can be provided as an argument to [`Application::theme`](struct.Application.md).

`iced` provides two implementors:

- the built-in [`Theme`](../enum.Theme.md) itself
- and any `Fn(&State) -> impl Into<Option<Theme>>`.

**Source:** [application.rs](../../src/iced/application.rs.md) (lines 621-630)

## Required Methods

### `theme`

```rust
fn theme(&self, state: &State) -> Option<Theme>
```

Returns the theme of the [`Application`](struct.Application.md) for the current state.

If `None` is returned, `iced` will try to use a theme that matches the system color scheme.

## Implementors

- `impl<F, T, State, Theme> ThemeFn<State, Theme> for F where F: Fn(&State) -> T, T: Into<Option<Theme>>, Theme: Base,`
- `impl<State> ThemeFn<State, Theme> for Theme`
