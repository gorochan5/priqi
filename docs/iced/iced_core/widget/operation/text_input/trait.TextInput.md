---
title: "TextInput"
kind: trait
path: "iced_core::widget::operation::text_input::TextInput"
crate: "iced_core"
module: "iced_core::widget::operation::text_input"
file: "iced_core/widget/operation/text_input/trait.TextInput.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/text_input/trait.TextInput.html"
source_file: "src/iced_core/widget/operation/text_input.rs"
source_lines: "8-28"
---

# `iced_core::widget::operation::text_input::TextInput`

```rust
pub trait TextInput {
    // Required methods
    fn text(&self) -> Fragment<'_>;
    fn move_cursor_to_front(&mut self);
    fn move_cursor_to_end(&mut self);
    fn move_cursor_to(&mut self, location: Position);
    fn select_all(&mut self);
    fn select_range(&mut self, start: Position, end: Position);
}
```

The internal state of a widget that has text input.

**Source:** [text_input.rs](../../../../src/iced_core/widget/operation/text_input.rs.md) (lines 8-28)

## Required Methods

### `text`

```rust
fn text(&self) -> Fragment<'_>
```

Returns the current *visible* text of the text input.

Normally, this is either its value or its placeholder.

### `move_cursor_to_front`

```rust
fn move_cursor_to_front(&mut self)
```

Moves the cursor of the text input to the front of the input text.

### `move_cursor_to_end`

```rust
fn move_cursor_to_end(&mut self)
```

Moves the cursor of the text input to the end of the input text.

### `move_cursor_to`

```rust
fn move_cursor_to(&mut self, location: Position)
```

Moves the cursor of the text input to an arbitrary location.

### `select_all`

```rust
fn select_all(&mut self)
```

Selects all the content of the text input.

### `select_range`

```rust
fn select_range(&mut self, start: Position, end: Position)
```

Selects the given content range of the text input.

## Implementors

- `impl<R: Renderer> TextInput for Input<R>`
- `impl<T: Editor> TextInput for T`
