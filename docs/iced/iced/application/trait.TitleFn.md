---
title: "TitleFn"
kind: trait
path: "iced::application::TitleFn"
crate: "iced"
module: "iced::application"
file: "iced/application/trait.TitleFn.md"
docs_url: "https://docs.iced.rs/iced/application/trait.TitleFn.html"
source_file: "src/iced/application.rs"
source_lines: "591-594"
---

# `iced::application::TitleFn`

```rust
pub trait TitleFn<State> {
    // Required method
    fn title(&self, state: &State) -> String;
}
```

The title logic of some [`Application`](struct.Application.md).

This trait is implemented both for `&'static str` and any closure `Fn(&State) -> String`.

This trait allows the [`Application::title`](struct.Application.md) builder to take any of them.

**Source:** [application.rs](../../src/iced/application.rs.md) (lines 591-594)

## Required Methods

### `title`

```rust
fn title(&self, state: &State) -> String
```

Produces the title of the [`Application`](struct.Application.md).

## Implementors

- `impl<T, State> TitleFn<State> for T where T: Fn(&State) -> String,`
