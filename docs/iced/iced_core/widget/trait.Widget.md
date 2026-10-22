---
title: "Widget"
kind: trait
path: "iced_core::widget::Widget"
crate: "iced_core"
module: "iced_core::widget"
file: "iced_core/widget/trait.Widget.md"
docs_url: "https://docs.iced.rs/iced_core/widget/trait.Widget.html"
source_file: "src/iced_core/widget.rs"
source_lines: "40-158"
---

# `iced_core::widget::Widget`

```rust
pub trait Widget<Message, Theme, Renderer>: Meta {
    // Required methods
    fn size(&self) -> Size<Length>;
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits);
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout,
        cursor: Cursor,
        viewport: &Rectangle,
    );

    // Provided methods
    fn tag(&self) -> Tag { ... }
    fn state(&self) -> State { ... }
    fn diff(&mut self, tree: &mut Tree) { ... }
    fn operate(
        &mut self,
        _tree: &mut Tree,
        _layout: Layout,
        _viewport: &Rectangle,
        _renderer: &Renderer,
        _operation: &mut dyn Operation,
    ) { ... }
    fn update(
        &mut self,
        _tree: &mut Tree,
        _event: &Event,
        _layout: Layout,
        _cursor: Cursor,
        _renderer: &Renderer,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) { ... }
    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout,
        _cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> Interaction { ... }
    fn overlay<'a>(
        &'a mut self,
        _tree: &'a mut Tree,
        _layout: Layout,
        _renderer: &Renderer,
        _viewport: &Rectangle,
        _translation: Vector,
        _window: Size,
    ) -> Vec<Element<'a, Message, Theme, Renderer>> { ... }
    fn _map<F, B>(self, f: F) -> Map<Self, F, Message>
       where Self: Sized,
             F: Fn(Message) -> B { ... }
    fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer>
       where Self: Sized + 'a { ... }
}
```

A component that displays information and allows interaction.

If you want to build your own widgets, you will need to implement this trait.

## Examples

The repository has some [examples](https://github.com/iced-rs/iced/tree/master/examples) showcasing how to implement a custom widget:

- [`custom_widget`](https://github.com/iced-rs/iced/tree/master/examples/custom_widget), a demonstration of how to build a custom widget that draws a circle.
- [`geometry`](https://github.com/iced-rs/iced/tree/master/examples/geometry), a custom widget showcasing how to draw geometry with the `Mesh2D` primitive in [`iced_wgpu`](https://github.com/iced-rs/iced/tree/master/wgpu).

**Source:** [widget.rs](../../src/iced_core/widget.rs.md) (lines 40-158)

## Required Methods

### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`](../struct.Size.md) of the [`Widget`](trait.Widget.md) in lengths.

### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`](trait.Widget.md).

This computes the [`Layout`](../layout/struct.Layout.md) of the [`Widget`](trait.Widget.md) and stores the result in the provided [`Tree`](tree/struct.Tree.md).

### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout, cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`](trait.Widget.md) using the associated `Renderer`.

## Provided Methods

### `tag`

```rust
fn tag(&self) -> Tag
```

Returns the [`Tag`](tree/struct.Tag.md) of the [`Widget`](trait.Widget.md).

### `state`

```rust
fn state(&self) -> State
```

Returns the [`State`](tree/enum.State.md) of the [`Widget`](trait.Widget.md).

### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`](trait.Widget.md) with the provided [`Tree`](tree/struct.Tree.md).

### `operate`

```rust
fn operate( &mut self, _tree: &mut Tree, _layout: Layout, _viewport: &Rectangle, _renderer: &Renderer, _operation: &mut dyn Operation, )
```

Applies an [`Operation`](operation/trait.Operation.md) to the [`Widget`](trait.Widget.md).

### `update`

```rust
fn update( &mut self, _tree: &mut Tree, _event: &Event, _layout: Layout, _cursor: Cursor, _renderer: &Renderer, _shell: &mut Shell<'_, Message>, _viewport: &Rectangle, )
```

Processes a runtime [`Event`](../event/enum.Event.md).

By default, it does nothing.

### `mouse_interaction`

```rust
fn mouse_interaction( &self, _tree: &Tree, _layout: Layout, _cursor: Cursor, _viewport: &Rectangle, _renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../mouse/enum.Interaction.md) of the [`Widget`](trait.Widget.md).

By default, it returns [`mouse::Interaction::None`](../mouse/enum.Interaction.md).

### `overlay`

```rust
fn overlay<'a>( &'a mut self, _tree: &'a mut Tree, _layout: Layout, _renderer: &Renderer, _viewport: &Rectangle, _translation: Vector, _window: Size, ) -> Vec<Element<'a, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`](trait.Widget.md).

### `_map`

```rust
fn _map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](trait.Widget.md), returning a [`Map`](struct.Map.md).

This method is useful when you want to decouple different parts of your UI and make them **composable**.

### `_boxed`

```rust
fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](trait.Widget.md), turning it into a generic [`Element`](struct.Element.md).

## Implementors

- `impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Element<'a, Message, Theme, Renderer>`
- `impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Void`
- `impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Text<'_, Theme> where Theme: Catalog, Renderer: Renderer,`
- `impl<W, A, B, F, Theme, Renderer> Widget<B, Theme, Renderer> for Map<W, F, A> where W: Widget<A, Theme, Renderer>, B: 'static, F: Fn(A) -> B, Theme: 'static, Renderer: Renderer + 'static,`
