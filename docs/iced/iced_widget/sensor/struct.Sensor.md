---
title: "Sensor"
kind: struct
path: "iced_widget::sensor::Sensor"
crate: "iced_widget"
module: "iced_widget::sensor"
file: "iced_widget/sensor/struct.Sensor.md"
docs_url: "https://docs.iced.rs/iced_widget/sensor/struct.Sensor.html"
source_file: "src/iced_widget/sensor.rs"
source_lines: "15-23"
---

# `iced_widget::sensor::Sensor`

```rust
pub struct Sensor<'a, Key, Message, W = Element<'a, Message>> { /* private fields */ }
```

A widget that can generate messages when its content pops in and out of view.

It can even notify you with anticipation at a given distance!

**Source:** [sensor.rs](../../src/iced_widget/sensor.rs.md) (lines 15-23)

## Methods

### `impl<'a, Message, W> Sensor<'a, (), Message, W>`

#### `new`

```rust
pub fn new(content: W) -> Self
```

Creates a new [`Sensor`](struct.Sensor.md) widget with the given content.

### `impl<'a, Key, Message, W> Sensor<'a, Key, Message, W> where Key: Key,`

#### `on_show`

```rust
pub fn on_show(self, on_show: impl Fn(Size) -> Message + 'a) -> Self
```

Sets the message to be produced when the content pops into view.

The closure will receive the [`Size`] of the content in that moment.

#### `on_resize`

```rust
pub fn on_resize<T>(self, on_resize: impl Fn(Size) -> T + 'a) -> Self where T: Into<Option<Message>>,
```

Sets the message to be produced when the content changes [`Size`] once its in view.

The closure will receive the new [`Size`] of the content.

#### `on_hide`

```rust
pub fn on_hide(self, on_hide: Message) -> Self
```

Sets the message to be produced when the content pops out of view.

#### `key`

```rust
pub fn key<K>(self, key: K) -> Sensor<'a, impl Key, Message, W> where K: Clone + PartialEq + 'static,
```

Sets the key of the [`Sensor`](struct.Sensor.md) widget, for continuity.

If the key changes, the [`Sensor`](struct.Sensor.md) widget will trigger again.

#### `key_ref`

```rust
pub fn key_ref<K>(self, key: &'a K) -> Sensor<'a, &'a K, Message, W> where K: ToOwned + PartialEq<K::Owned> + ?Sized, K::Owned: 'static,
```

Sets the key of the [`Sensor`](struct.Sensor.md), for continuity; using a reference.

If the key changes, the [`Sensor`](struct.Sensor.md) will trigger again.

#### `anticipate`

```rust
pub fn anticipate(self, distance: impl Into<Pixels>) -> Self
```

Sets the distance in [`Pixels`] to use in anticipation of the content popping into view.

This can be quite useful to lazily load items in a long scrollable behind the scenes before the user can notice it!

#### `delay`

```rust
pub fn delay(self, delay: impl Into<Duration>) -> Self
```

Sets the amount of time to wait before firing an [`on_show`](struct.Sensor.md) or [`on_hide`](struct.Sensor.md) event; after the content is shown or hidden.

When combined with [`key`](struct.Sensor.md), this can be useful to debounce key changes.

## Trait Implementations

### `impl<Key, Message, W> Meta for Sensor<'_, Key, Message, W>`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`] is [`Void`](../struct.Void.md).

### `impl<Key, Message, W, Theme, Renderer> Widget<Message, Theme, Renderer> for Sensor<'_, Key, Message, W> where Key: Key, Renderer: Renderer, W: Widget<Message, Theme, Renderer>,`

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

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, renderer: &Renderer, shell: &mut Shell<'_, Message>, viewport: &Rectangle, )
```

Processes a runtime [`Event`](../canvas/enum.Event.md). Read more

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

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout, cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`] using the associated `Renderer`.

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, viewport: &Rectangle, renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`] to the [`Widget`].

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, tree: &Tree, layout: Layout, cursor: Cursor, viewport: &Rectangle, renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`] of the [`Widget`]. Read more

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

- `impl<'a, Key, Message, W> Freeze for Sensor<'a, Key, Message, W> where W: Freeze, Key: Freeze, Message: Freeze,`
- `impl<'a, Key, Message, W = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>> !RefUnwindSafe for Sensor<'a, Key, Message, W>`
- `impl<'a, Key, Message, W = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>> !Send for Sensor<'a, Key, Message, W>`
- `impl<'a, Key, Message, W = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>> !Sync for Sensor<'a, Key, Message, W>`
- `impl<'a, Key, Message, W> Unpin for Sensor<'a, Key, Message, W> where W: Unpin, Key: Unpin, Message: Unpin,`
- `impl<'a, Key, Message, W> UnsafeUnpin for Sensor<'a, Key, Message, W> where W: UnsafeUnpin, Key: UnsafeUnpin, Message: UnsafeUnpin,`
- `impl<'a, Key, Message, W = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>> !UnwindSafe for Sensor<'a, Key, Message, W>`
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
