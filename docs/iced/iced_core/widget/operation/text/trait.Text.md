---
title: "Text"
kind: trait
path: "iced_core::widget::operation::text::Text"
crate: "iced_core"
module: "iced_core::widget::operation::text"
file: "iced_core/widget/operation/text/trait.Text.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/text/trait.Text.html"
source_file: "src/iced_core/widget/operation/text.rs"
source_lines: "8-28"
---

# `iced_core::widget::operation::text::Text`

```rust
pub trait Text {
    // Required methods
    fn text(&self) -> Fragment<'_>;
    fn select(&mut self, start: Point, end: Point, target: Target);
    fn select_all(&mut self);
    fn deselect(&mut self);
    fn copy(&mut self) -> Option<String>;
}
```

The internal state of a widget that has text.

**Source:** [text.rs](../../../../src/iced_core/widget/operation/text.rs.md) (lines 8-28)

## Required Methods

### `text`

```rust
fn text(&self) -> Fragment<'_>
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

- `impl<P: Paragraph> Text for Operand<'_, P>`
