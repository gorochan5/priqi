---
title: "Program"
kind: trait
path: "iced_widget::shader::Program"
crate: "iced_widget"
module: "iced_widget::shader"
file: "iced_widget/shader/trait.Program.md"
docs_url: "https://docs.iced.rs/iced_widget/shader/trait.Program.html"
source_file: "src/iced_widget/shader/program.rs"
source_lines: "12-60"
---

# `iced_widget::shader::Program`

```rust
pub trait Program<Message> {
    type State: Default + 'static;
    type Primitive: Primitive + 'static;

    // Required method
    fn draw(
        &self,
        state: &Self::State,
        cursor: Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive;

    // Provided methods
    fn update(
        &self,
        _state: &mut Self::State,
        _event: &Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Option<Action<Message>> { ... }
    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction { ... }
}
```

The state and logic of a [`Shader`](struct.Shader.md) widget.

A [`Program`](trait.Program.md) can mutate the internal state of a [`Shader`](struct.Shader.md) widget and produce messages for an application.

**Source:** [program.rs](../../src/iced_widget/shader/program.rs.md) (lines 12-60)

## Required Associated Types

### `State`

```rust
type State: Default + 'static
```

The internal state of the [`Program`](trait.Program.md).

### `Primitive`

```rust
type Primitive: Primitive + 'static
```

The type of primitive this [`Program`](trait.Program.md) can draw.

## Required Methods

### `draw`

```rust
fn draw( &self, state: &Self::State, cursor: Cursor, bounds: Rectangle, ) -> Self::Primitive
```

Draws the [`Primitive`](trait.Program.md).

## Provided Methods

### `update`

```rust
fn update( &self, _state: &mut Self::State, _event: &Event, _bounds: Rectangle, _cursor: Cursor, ) -> Option<Action<Message>>
```

Update the internal [`State`](trait.Program.md) of the [`Program`](trait.Program.md). This can be used to reflect state changes based on mouse & other events. You can return an [`Action`](../struct.Action.md) to publish a message, request a redraw, or capture the event.

By default, this method returns `None`.

### `mouse_interaction`

```rust
fn mouse_interaction( &self, _state: &Self::State, _bounds: Rectangle, _cursor: Cursor, ) -> Interaction
```

Returns the current mouse interaction of the [`Program`](trait.Program.md).

The interaction returned will be in effect even if the cursor position is out of bounds of the [`Shader`](struct.Shader.md)’s program.

## Implementors
