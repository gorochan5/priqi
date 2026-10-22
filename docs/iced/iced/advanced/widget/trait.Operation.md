---
title: "Operation"
kind: trait
path: "iced::advanced::widget::Operation"
crate: "iced"
module: "iced::advanced::widget"
file: "iced/advanced/widget/trait.Operation.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/trait.Operation.html"
---

# `iced::advanced::widget::Operation`

```rust
pub trait Operation<T = ()>: Send {
    // Required method
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>));

    // Provided methods
    fn container(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _viewport: &Rectangle,
    ) { ... }
    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _content: Size,
        _translation: Vector,
        _state: &mut dyn Scrollable,
    ) { ... }
    fn focusable(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _state: &mut dyn Focusable,
    ) { ... }
    fn text_input(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _state: &mut dyn TextInput,
    ) { ... }
    fn text(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _state: &mut dyn Text,
    ) { ... }
    fn custom(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _state: &mut (dyn Any + 'static),
    ) { ... }
    fn finish(&self) -> Outcome<T> { ... }
}
```

A piece of logic that can traverse the widget tree of an application in order to query or update some widget state.

## Required Methods

### `traverse`

```rust
fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>))
```

Requests further traversal of the widget tree to keep operating.

The provided `operate` closure may be called by an [`Operation`](trait.Operation.md) to return control to the widget tree and keep traversing it. If the closure is not called, the children of the widget asking for traversal will be skipped.

## Provided Methods

### `container`

```rust
fn container( &mut self, _id: Option<&Id>, _bounds: Rectangle, _viewport: &Rectangle, )
```

Operates on a widget that contains other widgets.

### `scrollable`

```rust
fn scrollable( &mut self, _id: Option<&Id>, _bounds: Rectangle, _content: Size, _translation: Vector, _state: &mut dyn Scrollable, )
```

Operates on a widget that can be scrolled.

### `focusable`

```rust
fn focusable( &mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn Focusable, )
```

Operates on a widget that can be focused.

### `text_input`

```rust
fn text_input( &mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn TextInput, )
```

Operates on a widget that has text input.

### `text`

```rust
fn text(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn Text)
```

Operates on a widget that contains some text.

### `custom`

```rust
fn custom( &mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut (dyn Any + 'static), )
```

Operates on a custom widget with some state.

### `finish`

```rust
fn finish(&self) -> Outcome<T>
```

Finishes the [`Operation`](trait.Operation.md) and returns its [`Outcome`](operation/enum.Outcome.md).

## Implementors
