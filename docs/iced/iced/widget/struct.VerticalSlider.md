---
title: "VerticalSlider"
kind: struct
path: "iced::widget::VerticalSlider"
crate: "iced"
module: "iced::widget"
file: "iced/widget/struct.VerticalSlider.md"
docs_url: "https://docs.iced.rs/iced/widget/struct.VerticalSlider.html"
source_file: "src/iced_widget/vertical_slider.rs"
source_lines: "87"
---

# `iced::widget::VerticalSlider`

```rust
pub struct VerticalSlider<'a, T, Message, Theme = Theme> where
    Theme: Catalog, { /* private fields */ }
```

An vertical bar and a handle that selects a single value from a range of values.

A [`VerticalSlider`](struct.VerticalSlider.md) will try to fill the vertical space of its container.

The [`VerticalSlider`](struct.VerticalSlider.md) range of numeric values is generic and its step size defaults to 1 unit.

Note: Under the hood values are converted to/from f64 so only values representable exactly as an f64 are possible to select via the slider. However it is likely that the precision of the slider at these scales is already less than the precision lost from the f64 representation.

## Example

```rust
use iced::widget::vertical_slider;

struct State {
   value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    ValueChanged(f32),
}

fn view(state: &State) -> impl Widget<Message> {
    vertical_slider(0.0..=100.0, state.value, Message::ValueChanged)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ValueChanged(value) => {
            state.value = value;
        }
    }
}
```

**Source:** [vertical_slider.rs](../../src/iced_widget/vertical_slider.rs.md) (lines 87)

## Methods

### `impl<'a, T, Message, Theme> VerticalSlider<'a, T, Message, Theme> where T: Copy + PartialOrd, Message: Clone, Theme: Catalog,`

#### `DEFAULT_WIDTH`

```rust
pub const DEFAULT_WIDTH: f32 = 16.0
```

The default width of a [`VerticalSlider`](struct.VerticalSlider.md).

#### `new`

```rust
pub fn new<F>( range: RangeInclusive<T>, value: T, on_change: F, ) -> VerticalSlider<'a, T, Message, Theme> where F: 'a + Fn(T) -> Message,
```

Creates a new [`VerticalSlider`](struct.VerticalSlider.md).

It expects:

- an inclusive range of possible values
- the current value of the [`VerticalSlider`](struct.VerticalSlider.md)
- a function that will be called when the [`VerticalSlider`](struct.VerticalSlider.md) is dragged. It receives the new value of the [`VerticalSlider`](struct.VerticalSlider.md) and must produce a `Message`.

#### `default`

```rust
pub fn default( self, default: impl Into<T>, ) -> VerticalSlider<'a, T, Message, Theme>
```

Sets the optional default value for the [`VerticalSlider`](struct.VerticalSlider.md).

If set, the [`VerticalSlider`](struct.VerticalSlider.md) will reset to this value when ctrl-clicked or command-clicked.

#### `on_release`

```rust
pub fn on_release( self, on_release: Message, ) -> VerticalSlider<'a, T, Message, Theme>
```

Sets the release message of the [`VerticalSlider`](struct.VerticalSlider.md). This is called when the mouse is released from the slider.

Typically, the user’s interaction with the slider is finished when this message is produced. This is useful if you need to spawn a long-running task from the slider’s result, where the default on_change message could create too many events.

#### `width`

```rust
pub fn width( self, width: impl Into<Pixels>, ) -> VerticalSlider<'a, T, Message, Theme>
```

Sets the width of the [`VerticalSlider`](struct.VerticalSlider.md).

#### `height`

```rust
pub fn height( self, height: impl Into<Length>, ) -> VerticalSlider<'a, T, Message, Theme>
```

Sets the height of the [`VerticalSlider`](struct.VerticalSlider.md).

#### `step`

```rust
pub fn step( self, step: impl AsPrimitive<f64>, ) -> VerticalSlider<'a, T, Message, Theme>
```

Sets the step size of the [`VerticalSlider`](struct.VerticalSlider.md).

#### `shift_step`

```rust
pub fn shift_step( self, shift_step: impl AsPrimitive<f64>, ) -> VerticalSlider<'a, T, Message, Theme>
```

Sets the optional “shift” step for the [`VerticalSlider`](struct.VerticalSlider.md).

If set, this value is used as the step while the shift key is pressed.

#### `style`

```rust
pub fn style( self, style: impl Fn(&Theme, Status) -> Style + 'a, ) -> VerticalSlider<'a, T, Message, Theme> where <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme, Status) -> Style + 'a>>,
```

Sets the style of the [`VerticalSlider`](struct.VerticalSlider.md).

#### `class`

```rust
pub fn class( self, class: impl Into<<Theme as Catalog>::Class<'a>>, ) -> VerticalSlider<'a, T, Message, Theme>
```

*Available on crate feature advanced only.*

Sets the style class of the [`VerticalSlider`](struct.VerticalSlider.md).

## Trait Implementations

### `impl<T, Message, Theme> Meta for VerticalSlider<'_, T, Message, Theme> where Theme: Catalog,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`](../advanced/trait.Widget.md) is [`Void`](struct.Void.md).

### `impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for VerticalSlider<'_, T, Message, Theme> where T: Copy + AsPrimitive<f64> + FromPrimitive, Message: Clone, Theme: Catalog, Renderer: Renderer,`

#### `tag`

```rust
fn tag(&self) -> Tag
```

Returns the [`Tag`](../advanced/widget/tree/struct.Tag.md) of the [`Widget`](../advanced/trait.Widget.md).

#### `state`

```rust
fn state(&self) -> State
```

Returns the [`State`](../advanced/widget/tree/enum.State.md) of the [`Widget`](../advanced/trait.Widget.md).

#### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`](../struct.Size.md) of the [`Widget`](../advanced/trait.Widget.md) in lengths.

#### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`](../advanced/trait.Widget.md). [Read more](../advanced/trait.Widget.md)

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, _renderer: &Renderer, shell: &mut Shell<'_, Message>, _viewport: &Rectangle, )
```

Processes a runtime [`Event`](../enum.Event.md). [Read more](../advanced/trait.Widget.md)

#### `draw`

```rust
fn draw( &self, _tree: &Tree, renderer: &mut Renderer, theme: &Theme, _style: &Style, layout: Layout, _cursor: Cursor, _viewport: &Rectangle, )
```

Draws the [`Widget`](../advanced/trait.Widget.md) using the associated `Renderer`.

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, tree: &Tree, layout: Layout, cursor: Cursor, _viewport: &Rectangle, _renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../mouse/enum.Interaction.md) of the [`Widget`](../advanced/trait.Widget.md). [Read more](../advanced/trait.Widget.md)

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`](../advanced/trait.Widget.md) with the provided [`Tree`](../advanced/widget/struct.Tree.md).

#### `operate`

```rust
fn operate( &mut self, _tree: &mut Tree, _layout: Layout, _viewport: &Rectangle, _renderer: &Renderer, _operation: &mut dyn Operation, )
```

Applies an [`Operation`](../advanced/widget/trait.Operation.md) to the [`Widget`](../advanced/trait.Widget.md).

#### `overlay`

```rust
fn overlay<'a>( &'a mut self, _tree: &'a mut Tree, _layout: Layout, _renderer: &Renderer, _viewport: &Rectangle, _translation: Vector, _window: Size, ) -> Vec<Element<'a, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`](../advanced/trait.Widget.md).

#### `_map`

```rust
fn _map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](../advanced/trait.Widget.md), returning a [`Map`](../advanced/widget/struct.Map.md). [Read more](../advanced/trait.Widget.md)

#### `_boxed`

```rust
fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](../advanced/trait.Widget.md), turning it into a generic [`Element`](../advanced/widget/struct.Element.md).

## Auto Trait Implementations

- `impl<'a, T, Message, Theme> Freeze for VerticalSlider<'a, T, Message, Theme> where T: Freeze, <Theme as Catalog>::Class<'a>: Freeze, Message: Freeze,`
- `impl<'a, T, Message, Theme = Theme> !RefUnwindSafe for VerticalSlider<'a, T, Message, Theme>`
- `impl<'a, T, Message, Theme = Theme> !Send for VerticalSlider<'a, T, Message, Theme>`
- `impl<'a, T, Message, Theme = Theme> !Sync for VerticalSlider<'a, T, Message, Theme>`
- `impl<'a, T, Message, Theme> Unpin for VerticalSlider<'a, T, Message, Theme> where T: Unpin, <Theme as Catalog>::Class<'a>: Unpin, Message: Unpin,`
- `impl<'a, T, Message, Theme> UnsafeUnpin for VerticalSlider<'a, T, Message, Theme> where T: UnsafeUnpin, <Theme as Catalog>::Class<'a>: UnsafeUnpin, Message: UnsafeUnpin,`
- `impl<'a, T, Message, Theme = Theme> !UnwindSafe for VerticalSlider<'a, T, Message, Theme>`
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

### `impl<State, Message> IntoBoot<State, Message> for State`

#### `into_boot`

```rust
fn into_boot(self) -> (State, Task<Message>)
```

Turns some type into the initial state of some [`Application`](../application/struct.Application.md).

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

### `impl<T> Same for T`

#### `Output`

```rust
type Output = T
```

Should always be `Self`

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

### `impl<V, T> VZip<V> for T where V: MultiLane<T>,`

#### `vzip`

```rust
fn vzip(self) -> V
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
