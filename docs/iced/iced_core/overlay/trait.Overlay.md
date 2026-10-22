---
title: "Overlay"
kind: trait
path: "iced_core::overlay::Overlay"
crate: "iced_core"
module: "iced_core::overlay"
file: "iced_core/overlay/trait.Overlay.md"
docs_url: "https://docs.iced.rs/iced_core/overlay/trait.Overlay.html"
source_file: "src/iced_core/overlay.rs"
source_lines: "15-75"
---

# `iced_core::overlay::Overlay`

```rust
pub trait Overlay<Message, Theme, Renderer> where
    Renderer: Renderer, {
    // Required method
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        cursor: Cursor,
    );

    // Provided methods
    fn operate(&mut self, _renderer: &Renderer, _operation: &mut dyn Operation) { ... }
    fn update(
        &mut self,
        _event: &Event,
        _cursor: Cursor,
        _renderer: &Renderer,
        _shell: &mut Shell<'_, Message>,
    ) { ... }
    fn mouse_interaction(
        &self,
        _cursor: Cursor,
        _renderer: &Renderer,
    ) -> Interaction { ... }
    fn overlay<'a>(
        &'a mut self,
        _renderer: &Renderer,
    ) -> Vec<Element<'a, Message, Theme, Renderer>> { ... }
    fn index(&self) -> f32 { ... }
}
```

An interactive component that can be displayed on top of other widgets.

**Source:** [overlay.rs](../../src/iced_core/overlay.rs.md) (lines 15-75)

## Required Methods

### `draw`

```rust
fn draw( &self, renderer: &mut Renderer, theme: &Theme, style: &Style, cursor: Cursor, )
```

Draws the [`Overlay`](trait.Overlay.md) using the associated `Renderer`.

Implementors are expected to call [`Renderer::with_layer`](../renderer/trait.Renderer.md) themselves, so that their contents are properly clipped.

## Provided Methods

### `operate`

```rust
fn operate(&mut self, _renderer: &Renderer, _operation: &mut dyn Operation)
```

Applies a [`widget::Operation`](../widget/operation/trait.Operation.md) to the [`Overlay`](trait.Overlay.md).

### `update`

```rust
fn update( &mut self, _event: &Event, _cursor: Cursor, _renderer: &Renderer, _shell: &mut Shell<'_, Message>, )
```

Processes a runtime [`Event`](../event/enum.Event.md).

By default, it does nothing.

### `mouse_interaction`

```rust
fn mouse_interaction( &self, _cursor: Cursor, _renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../mouse/enum.Interaction.md) of the [`Overlay`](trait.Overlay.md).

By default, it returns [`mouse::Interaction::None`](../mouse/enum.Interaction.md).

### `overlay`

```rust
fn overlay<'a>( &'a mut self, _renderer: &Renderer, ) -> Vec<Element<'a, Message, Theme, Renderer>>
```

Returns the nested overlays of the [`Overlay`](trait.Overlay.md).

### `index`

```rust
fn index(&self) -> f32
```

The index of the overlay.

Overlays with a higher index will be rendered on top of overlays with a lower index.

By default, it returns `1.0`.

## Implementors
