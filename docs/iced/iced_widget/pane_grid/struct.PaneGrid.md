---
title: "PaneGrid"
kind: struct
path: "iced_widget::pane_grid::PaneGrid"
crate: "iced_widget"
module: "iced_widget::pane_grid"
file: "iced_widget/pane_grid/struct.PaneGrid.md"
docs_url: "https://docs.iced.rs/iced_widget/pane_grid/struct.PaneGrid.html"
source_file: "src/iced_widget/pane_grid.rs"
source_lines: "148-165"
---

# `iced_widget::pane_grid::PaneGrid`

```rust
pub struct PaneGrid<'a, Message, T, W, Theme = Theme, Renderer = Renderer> where
    Theme: Catalog,
    Renderer: Renderer, { /* private fields */ }
```

A collection of panes distributed using either vertical or horizontal splits to completely fill the space available.

![Pane grid - Iced](https://iced.rs/examples/pane_grid.gif)

This distribution of space is common in tiling window managers (like [`awesome`](https://awesomewm.org/), [`i3`](https://i3wm.org/), or even [`tmux`](https://github.com/tmux/tmux)).

A [`PaneGrid`](struct.PaneGrid.md) supports:

- Vertical and horizontal splits
- Tracking of the last active pane
- Mouse-based resizing
- Drag and drop to reorganize panes
- Hotkey support
- Configurable modifier keys
- [`State`](state/struct.State.md) API to perform actions programmatically (`split`, `swap`, `resize`, etc.)

## Example

```rust
use iced::widget::{pane_grid, text};

struct State {
    panes: pane_grid::State<Pane>,
}

enum Pane {
    SomePane,
    AnotherKindOfPane,
}

enum Message {
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
}

fn view(state: &State) -> impl Widget<Message> {
    pane_grid(&state.panes, |pane, state, is_maximized| {
        pane_grid::Content::new(match state {
            Pane::SomePane => text("This is some pane"),
            Pane::AnotherKindOfPane => text("This is another kind of pane"),
        })
    })
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
}
```

**Source:** [pane_grid.rs](../../src/iced_widget/pane_grid.rs.md) (lines 148-165)

## Methods

### `impl<'a, Message, T, W, Theme, Renderer> PaneGrid<'a, Message, T, W, Theme, Renderer> where Theme: Catalog, Renderer: Renderer,`

#### `new`

```rust
pub fn new<S>( state: &'a State<S>, view: impl Fn(Pane, &'a S, bool) -> Content<'a, Message, T, W, Theme, Renderer>, ) -> Self
```

Creates a [`PaneGrid`](struct.PaneGrid.md) with the given [`State`](state/struct.State.md) and view function.

The view function will be called to display each [`Pane`](struct.Pane.md) present in the [`State`](state/struct.State.md). [`bool`](https://doc.rust-lang.org/nightly/std/primitive.bool.html) is set if the pane is maximized.

#### `width`

```rust
pub fn width(self, width: impl Into<Length>) -> Self
```

Sets the width of the [`PaneGrid`](struct.PaneGrid.md).

#### `height`

```rust
pub fn height(self, height: impl Into<Length>) -> Self
```

Sets the height of the [`PaneGrid`](struct.PaneGrid.md).

#### `spacing`

```rust
pub fn spacing(self, amount: impl Into<Pixels>) -> Self
```

Sets the spacing *between* the panes of the [`PaneGrid`](struct.PaneGrid.md).

#### `min_size`

```rust
pub fn min_size(self, min_size: impl Into<Pixels>) -> Self
```

Sets the minimum size of a [`Pane`](struct.Pane.md) in the [`PaneGrid`](struct.PaneGrid.md) on both axes.

#### `on_click`

```rust
pub fn on_click<F>(self, f: F) -> Self where F: 'a + Fn(Pane) -> Message,
```

Sets the message that will be produced when a [`Pane`](struct.Pane.md) of the [`PaneGrid`](struct.PaneGrid.md) is clicked.

#### `on_drag`

```rust
pub fn on_drag<F>(self, f: F) -> Self where F: 'a + Fn(DragEvent) -> Message,
```

Enables the drag and drop interactions of the [`PaneGrid`](struct.PaneGrid.md), which will use the provided function to produce messages.

#### `on_resize`

```rust
pub fn on_resize<F>(self, leeway: impl Into<Pixels>, f: F) -> Self where F: 'a + Fn(ResizeEvent) -> Message,
```

Enables the resize interactions of the [`PaneGrid`](struct.PaneGrid.md), which will use the provided function to produce messages.

The `leeway` describes the amount of space around a split that can be used to grab it.

The grabbable area of a split will have a length of `spacing + leeway`, properly centered. In other words, a length of `(spacing + leeway) / 2.0` on either side of the split line.

#### `style`

```rust
pub fn style(self, style: impl Fn(&Theme) -> Style + 'a) -> Self where <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the style of the [`PaneGrid`](struct.PaneGrid.md).

#### `class`

```rust
pub fn class(self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self
```

*Available on crate feature advanced only.*

Sets the style class of the [`PaneGrid`](struct.PaneGrid.md).

## Trait Implementations

### `impl<Message, T, W, Theme, Renderer> Meta for PaneGrid<'_, Message, T, W, Theme, Renderer> where Theme: Catalog, Renderer: Renderer,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`] is [`Void`](../struct.Void.md).

### `impl<Message, T, W, Theme, Renderer> Widget<Message, Theme, Renderer> for PaneGrid<'_, Message, T, W, Theme, Renderer> where Theme: Catalog, Renderer: Renderer, T: Widget<Message, Theme, Renderer>, W: Widget<Message, Theme, Renderer>,`

#### `tag`

```rust
fn tag(&self) -> Tag
```

Returns the `Tag` of the [`Widget`].

#### `state`

```rust
fn state(&self) -> State
```

Returns the `State` of the [`Widget`].

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`] with the provided [`Tree`].

#### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`] of the [`Widget`] in lengths.

#### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`]. Read more

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, viewport: &Rectangle, renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`] to the [`Widget`].

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, renderer: &Renderer, shell: &mut Shell<'_, Message>, viewport: &Rectangle, )
```

Processes a runtime [`Event`](../canvas/enum.Event.md). Read more

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, tree: &Tree, layout: Layout, cursor: Cursor, viewport: &Rectangle, renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`] of the [`Widget`]. Read more

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, defaults: &Style, layout: Layout, cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`] using the associated `Renderer`.

#### `overlay`

```rust
fn overlay<'b>( &'b mut self, tree: &'b mut Tree, layout: Layout, renderer: &Renderer, viewport: &Rectangle, translation: Vector, window: Size, ) -> Vec<Element<'b, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`].

#### `_map`

```rust
fn _map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`], returning a [`Map`]. Read more

#### `_boxed`

```rust
fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`], turning it into a generic [`Element`].

## Auto Trait Implementations

- `impl<'a, Message, T, W, Theme, Renderer> Freeze for PaneGrid<'a, Message, T, W, Theme, Renderer> where <Theme as Catalog>::Class<'a>: Freeze,`
- `impl<'a, Message, T, W, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !RefUnwindSafe for PaneGrid<'a, Message, T, W, Theme, Renderer>`
- `impl<'a, Message, T, W, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Send for PaneGrid<'a, Message, T, W, Theme, Renderer>`
- `impl<'a, Message, T, W, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Sync for PaneGrid<'a, Message, T, W, Theme, Renderer>`
- `impl<'a, Message, T, W, Theme, Renderer> Unpin for PaneGrid<'a, Message, T, W, Theme, Renderer> where <Theme as Catalog>::Class<'a>: Unpin, W: Unpin, <Theme as Catalog>::Class<'a>: Unpin, T: Unpin,`
- `impl<'a, Message, T, W, Theme, Renderer> UnsafeUnpin for PaneGrid<'a, Message, T, W, Theme, Renderer> where <Theme as Catalog>::Class<'a>: UnsafeUnpin,`
- `impl<'a, Message, T, W, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !UnwindSafe for PaneGrid<'a, Message, T, W, Theme, Renderer>`
## Blanket Implementations

### `impl<T> Any for T where T: 'static + ?Sized,`

#### `type_id`

```rust
fn type_id(&self) -> TypeId
```

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

### `impl<T> Borrow<T> for T where T: ?Sized,`

#### `borrow`

```rust
fn borrow(&self) -> &T
```

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

### `impl<T> BorrowMut<T> for T where T: ?Sized,`

#### `borrow_mut`

```rust
fn borrow_mut(&mut self) -> &mut T
```

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

### `impl<T> Downcast<T> for T`

#### `downcast`

```rust
fn downcast(&self) -> &T
```

### `impl<T> Downcast for T where T: Any,`

#### `into_any`

```rust
fn into_any(self: Box<T>) -> Box<dyn Any>
```

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `into_any_rc`

```rust
fn into_any_rc(self: Rc<T>) -> Rc<dyn Any>
```

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `as_any`

```rust
fn as_any(&self) -> &(dyn Any + 'static)
```

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

#### `as_any_mut`

```rust
fn as_any_mut(&mut self) -> &mut (dyn Any + 'static)
```

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

### `impl<T> From<T> for T`

#### `from`

```rust
fn from(t: T) -> T
```

Returns the argument unchanged.

### `impl<T> Instrument for T`

#### `instrument`

```rust
fn instrument(self, span: Span) -> Instrumented<Self>
```

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

#### `in_current_span`

```rust
fn in_current_span(self) -> Instrumented<Self>
```

Instruments this type with the current `Span`, returning an `Instrumented` wrapper. Read more

### `impl<T, U> Into<U> for T where U: From<T>,`

#### `into`

```rust
fn into(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

### `impl<T> IntoEither for T`

#### `into_either`

```rust
fn into_either(self, into_left: bool) -> Either<Self, Self>
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either)

#### `into_either_with`

```rust
fn into_either_with<F>(self, into_left: F) -> Either<Self, Self> where F: FnOnce(&Self) -> bool,
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either_with)

### `impl<T> Pointable for T`

#### `ALIGN`

```rust
const ALIGN: usize
```

The alignment of pointer.

#### `Init`

```rust
type Init = T
```

The type for initializers.

#### `init`

```rust
unsafe fn init(init: <T as Pointable>::Init) -> usize
```

Initializes a with the given initializer. Read more

#### `deref`

```rust
unsafe fn deref<'a>(ptr: usize) -> &'a T
```

Dereferences the given pointer. Read more

#### `deref_mut`

```rust
unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T
```

Mutably dereferences the given pointer. Read more

#### `drop`

```rust
unsafe fn drop(ptr: usize)
```

Drops the object pointed to by the given pointer. Read more

### `impl<T, U> TryFrom<U> for T where U: Into<T>,`

#### `Error`

```rust
type Error = Infallible
```

The type returned in the event of a conversion error.

#### `try_from`

```rust
fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error>
```

Performs the conversion.

### `impl<T, U> TryInto<U> for T where U: TryFrom<T>,`

#### `Error`

```rust
type Error = <U as TryFrom<T>>::Error
```

The type returned in the event of a conversion error.

#### `try_into`

```rust
fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error>
```

Performs the conversion.

### `impl<T> Upcast<T> for T`

#### `upcast`

```rust
fn upcast(&self) -> Option<&T>
```

### `impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for T where T: Widget<Message, Theme, Renderer>,`

#### `map`

```rust
fn map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](../trait.Widget.md). [Read more](../trait.Widget.md)

#### `boxed`

```rust
fn boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](../trait.Widget.md), turning it into a generic [`Element`](../type.Element.md).

#### `explain`

```rust
fn explain( self, color: impl Into<Color>, ) -> impl Widget<Message, Theme, Renderer> where Self: Sized, Renderer: Renderer,
```

Marks this [`Widget`](../trait.Widget.md) as *to-be-explained*. [Read more](../trait.Widget.md)

### `impl<T> WithSubscriber for T`

#### `with_subscriber`

```rust
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> where S: Into<Dispatch>,
```

Attaches the provided `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

#### `with_current_subscriber`

```rust
fn with_current_subscriber(self) -> WithDispatch<Self>
```

Attaches the current default `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
