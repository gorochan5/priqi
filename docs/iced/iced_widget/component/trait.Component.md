---
title: "Component"
kind: trait
path: "iced_widget::component::Component"
crate: "iced_widget"
module: "iced_widget::component"
file: "iced_widget/component/trait.Component.md"
docs_url: "https://docs.iced.rs/iced_widget/component/trait.Component.html"
source_file: "src/iced_widget/component.rs"
source_lines: "42-111"
---

# `iced_widget::component::Component`

```rust
pub trait Component<'a, Message, Theme = Theme, Renderer = Renderer> where
    Renderer: Renderer, {
    type State: Default + 'static;
    type Event: 'static;

    // Required methods
    fn update(
        &self,
        state: &mut Self::State,
        event: Self::Event,
        renderer: &Renderer,
    ) -> Option<Message>;
    fn view(
        &self,
        state: &Self::State,
    ) -> impl Widget<Self::Event, Theme, Renderer> + 'a;

    // Provided methods
    fn listen(
        &self,
        _state: &Self::State,
        _event: &Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Action<Self::Event> { ... }
    fn mouse_interaction(&self, _state: &Self::State) -> Interaction { ... }
    fn diff(&mut self, _state: &mut Self::State) { ... }
    fn operate(
        &self,
        _state: &mut Self::State,
        _bounds: Rectangle,
        _operation: &mut dyn Operation,
    ) { ... }
}
```

A reusable, custom widget that uses The Elm Architecture.

A [`Component`](trait.Component.md) allows you to implement custom widgets as if they were `iced` applications with encapsulated state.

In other words, a [`Component`](trait.Component.md) allows you to turn `iced` applications into custom widgets and embed them without cumbersome wiring.

A [`Component`](trait.Component.md) produces widgets that may fire an [`Event`](trait.Component.md) and update the internal state of the [`Component`](trait.Component.md).

Additionally, a [`Component`](trait.Component.md) is capable of producing a `Message` to notify the parent application of any relevant interactions.

## State

A component can store its state in one of two ways: either as data within the implementor of the trait, or in a type [`State`](trait.Component.md) that is managed by the runtime and provided to the trait methods. These two approaches are not mutually exclusive and have opposite pros and cons.

For instance, if a piece of state is needed by multiple components that reside in different branches of the tree, then it’s more convenient to let a common ancestor store it and pass it down.

On the other hand, if a piece of state is only needed by the component itself, you can store it as part of its internal [`State`](trait.Component.md).

**Source:** [component.rs](../../src/iced_widget/component.rs.md) (lines 42-111)

## Required Associated Types

### `State`

```rust
type State: Default + 'static
```

The internal state of this [`Component`](trait.Component.md).

### `Event`

```rust
type Event: 'static
```

The type of event this [`Component`](trait.Component.md) handles internally.

## Required Methods

### `update`

```rust
fn update( &self, state: &mut Self::State, event: Self::Event, renderer: &Renderer, ) -> Option<Message>
```

Processes an [`Event`](trait.Component.md) and updates the [`Component`](trait.Component.md) state accordingly.

It can produce a `Message` for the parent application.

### `view`

```rust
fn view( &self, state: &Self::State, ) -> impl Widget<Self::Event, Theme, Renderer> + 'a
```

Produces the widgets of the [`Component`](trait.Component.md), which may trigger an [`Event`](trait.Component.md) on user interaction.

## Provided Methods

### `listen`

```rust
fn listen( &self, _state: &Self::State, _event: &Event, _bounds: Rectangle, _cursor: Cursor, ) -> Action<Self::Event>
```

Listens to a runtime [`Event`](../canvas/enum.Event.md) and performs an [`Action`](../struct.Action.md) as a result.

If the [`Action`](../struct.Action.md) publishes a [`Component::Event`](trait.Component.md), it will be immediately fed to [`update`](trait.Component.md).

By default, it returns [`Action::none`](../struct.Action.md).

### `mouse_interaction`

```rust
fn mouse_interaction(&self, _state: &Self::State) -> Interaction
```

Returns the current [`mouse::Interaction`] of the [`Component`](trait.Component.md).

This interaction will override any interaction produced by the [`view`](trait.Component.md) of the [`Component`](trait.Component.md).

By default, it returns [`mouse::Interaction::None`].

### `diff`

```rust
fn diff(&mut self, _state: &mut Self::State)
```

Reconciles the current [`Component`](trait.Component.md) with its internal [`State`](trait.Component.md) persisted in the widget tree.

This method will be called every time the widget tree changes. You can leverage it to detect and react to changes in the [`Component`](trait.Component.md).

By default, it does nothing.

### `operate`

```rust
fn operate( &self, _state: &mut Self::State, _bounds: Rectangle, _operation: &mut dyn Operation, )
```

Run the provided [`widget::Operation`] on the [`Component`](trait.Component.md).

By default, it does nothing.

## Implementors
