---
title: "TextInput"
kind: trait
path: "iced::advanced::widget::operation::TextInput"
crate: "iced"
module: "iced::advanced::widget::operation"
file: "iced/advanced/widget/operation/trait.TextInput.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/operation/trait.TextInput.html"
---

# `iced::advanced::widget::operation::TextInput`

```rust
pub trait TextInput {
    // Required methods
    fn text(&self) -> Cow<'_, str>;
    fn move_cursor_to_front(&mut self);
    fn move_cursor_to_end(&mut self);
    fn move_cursor_to(&mut self, location: Position);
    fn select_all(&mut self);
    fn select_range(&mut self, start: Position, end: Position);
}
```

The internal state of a widget that has text input.

## Required Methods

### `text`

```rust
fn text(&self) -> Cow<'_, str>
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

- `impl<R> TextInput for Input<R> where R: Renderer,`
- `impl<T> TextInput for T where T: Editor,`
