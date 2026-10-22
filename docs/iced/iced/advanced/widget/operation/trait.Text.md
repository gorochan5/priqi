---
title: "Text"
kind: trait
path: "iced::advanced::widget::operation::Text"
crate: "iced"
module: "iced::advanced::widget::operation"
file: "iced/advanced/widget/operation/trait.Text.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/operation/trait.Text.html"
---

# `iced::advanced::widget::operation::Text`

```rust
pub trait Text {
    // Required methods
    fn text(&self) -> Cow<'_, str>;
    fn select(&mut self, start: Point, end: Point, target: Target);
    fn select_all(&mut self);
    fn deselect(&mut self);
    fn copy(&mut self) -> Option<String>;
}
```

The internal state of a widget that has text.

## Required Methods

### `text`

```rust
fn text(&self) -> Cow<'_, str>
```

Returns the text of the widget.

### `select`

```rust
fn select(&mut self, start: Point, end: Point, target: Target)
```

Selects text between the given positions in layout coordinates.

`start` is the position where the user started selecting from, while `end` is the final position of the selection.

### `select_all`

```rust
fn select_all(&mut self)
```

Selects all the text of the widget.

### `deselect`

```rust
fn deselect(&mut self)
```

Deselects any selected text;

### `copy`

```rust
fn copy(&mut self) -> Option<String>
```

Returns the selected text, if any.

## Implementors

- `impl<P> Text for Operand<'_, P> where P: Paragraph,`
