---
title: "Focusable"
kind: trait
path: "iced_core::widget::operation::focusable::Focusable"
crate: "iced_core"
module: "iced_core::widget::operation::focusable"
file: "iced_core/widget/operation/focusable/trait.Focusable.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/focusable/trait.Focusable.html"
source_file: "src/iced_core/widget/operation/focusable.rs"
source_lines: "7-16"
---

# `iced_core::widget::operation::focusable::Focusable`

```rust
pub trait Focusable {
    // Required methods
    fn is_focused(&self) -> bool;
    fn focus(&mut self);
    fn unfocus(&mut self);
}
```

The internal state of a widget that can be focused.

**Source:** [focusable.rs](../../../../src/iced_core/widget/operation/focusable.rs.md) (lines 7-16)

## Required Methods

### `is_focused`

```rust
fn is_focused(&self) -> bool
```

Returns whether the widget is focused or not.

### `focus`

```rust
fn focus(&mut self)
```

Focuses the widget.

### `unfocus`

```rust
fn unfocus(&mut self)
```

Unfocuses the widget.

## Implementors

- `impl Focusable for State`
- `impl<R: Renderer> Focusable for Input<R>`
