---
title: "Program"
kind: trait
path: "iced_widget::canvas::Program"
crate: "iced_widget"
module: "iced_widget::canvas"
file: "iced_widget/canvas/trait.Program.md"
docs_url: "https://docs.iced.rs/iced_widget/canvas/trait.Program.html"
source_file: "src/iced_widget/canvas/program.rs"
source_lines: "13-73"
---

# `iced_widget::canvas::Program`

```rust
pub trait Program<Message, Theme = Theme, Renderer = Renderer> where
    Renderer: Renderer, {
    type State: Default + 'static;

    // Required method
    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> ⓘ;

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

The state and logic of a [`Canvas`](struct.Canvas.md).

A [`Program`](trait.Program.md) can mutate internal state and produce messages for an application.

**Source:** [program.rs](../../src/iced_widget/canvas/program.rs.md) (lines 13-73)

## Required Associated Types

### `State`

```rust
type State: Default + 'static
```

The internal state mutated by the [`Program`](trait.Program.md).

## Required Methods

### `draw`

```rust
fn draw( &self, state: &Self::State, renderer: &Renderer, theme: &Theme, bounds: Rectangle, cursor: Cursor, ) -> Vec<Geometry<Renderer>> ⓘ
```

Draws the state of the [`Program`](trait.Program.md), producing a bunch of [`Geometry`](type.Geometry.md).

[`Geometry`](type.Geometry.md) can be easily generated with a [`Frame`](type.Frame.md) or stored in a [`Cache`](type.Cache.md).

## Provided Methods

### `update`

```rust
fn update( &self, _state: &mut Self::State, _event: &Event, _bounds: Rectangle, _cursor: Cursor, ) -> Option<Action<Message>>
```

Updates the [`State`](trait.Program.md) of the [`Program`](trait.Program.md).

When a [`Program`](trait.Program.md) is used in a [`Canvas`](struct.Canvas.md), the runtime will call this method for each [`Event`](enum.Event.md).

This method can optionally return an [`Action`](../struct.Action.md) to either notify an application of any meaningful interactions, capture the event, or request a redraw.

By default, this method does and returns nothing.

### `mouse_interaction`

```rust
fn mouse_interaction( &self, _state: &Self::State, _bounds: Rectangle, _cursor: Cursor, ) -> Interaction
```

Returns the current mouse interaction of the [`Program`](trait.Program.md).

The interaction returned will be in effect even if the cursor position is out of bounds of the program’s [`Canvas`](struct.Canvas.md).

## Implementors
