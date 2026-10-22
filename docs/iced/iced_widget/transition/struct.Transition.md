---
title: "Transition"
kind: struct
path: "iced_widget::transition::Transition"
crate: "iced_widget"
module: "iced_widget::transition"
file: "iced_widget/transition/struct.Transition.md"
docs_url: "https://docs.iced.rs/iced_widget/transition/struct.Transition.html"
source_file: "src/iced_widget/transition.rs"
source_lines: "43-56"
---

# `iced_widget::transition::Transition`

```rust
pub struct Transition<'a, Message, W, P> where
    P: Program, { /* private fields */ }
```

A widget that can be used to animate its contents.

**Source:** [transition.rs](../../src/iced_widget/transition.rs.md) (lines 43-56)

## Methods

### `impl<'a, Message, W, P> Transition<'a, Message, W, P> where P: Program,`

#### `new`

```rust
pub fn new( init: impl Fn() -> P + 'a, value: P::Value, view: impl Fn(&P, Instant) -> W + 'a, ) -> Self
```

Creates a new [`Transition`](struct.Transition.md).

The `init` closure will be used to initialize an animation.

The `view` closure will receive the animation and an [`Instant`](https://doc.rust-lang.org/nightly/std/time/struct.Instant.html), which can be used for interpolating values. This will be called every frame until the given `value` is reached.

#### `id`

```rust
pub fn id(self, id: impl Into<Id>) -> Self
```

Sets the [`widget::Id`](../struct.Id.md) of the [`Transition`](struct.Transition.md).

The [`widget::Id`](../struct.Id.md) can subsequently be used to reset the [`Animation`] via [`reset`](fn.reset.md).

#### `key`

```rust
pub fn key(self, key: impl Hash) -> Self
```

Sets the key of the [`Transition`](struct.Transition.md) widget for continuity.

Changing the key will reset the [`Animation`].

#### `on_finish`

```rust
pub fn on_finish(self, on_finish: Message) -> Self where Message: Clone + 'a,
```

Sets the message that will be produced when the [`Transition`](struct.Transition.md) has finished animating.

Note that if an [`Animation`] is set to loop forever, the message will never be produced!

#### `on_finish_maybe`

```rust
pub fn on_finish_maybe(self, on_finish: impl Into<Option<Message>>) -> Self where Message: Clone + 'a,
```

Sets the message that will be produced when the [`Transition`](struct.Transition.md) has finished animating, if [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some).

Note that if an [`Animation`] is set to loop forever, the message will never be produced!

#### `on_finish_with`

```rust
pub fn on_finish_with(self, on_finish: impl Fn() -> Message + 'a) -> Self
```

Sets the message that will be produced when the [`Transition`](struct.Transition.md) has finished animating.

This is analogous to [`Transition::on_finish`](struct.Transition.md), but using a closure to produce the message.

This closure will only be called when the [`Transition`](struct.Transition.md) has actually finished animating and, therefore, this method is useful to reduce overhead if creating the resulting message is slow.

Note that if an [`Animation`] is set to loop forever, the message will never be produced!

## Trait Implementations

### `impl<Message, W, P> Meta for Transition<'_, Message, W, P> where P: Program,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`] is [`Void`](../struct.Void.md).

### `impl<Message, W, Theme, Renderer, P> Widget<Message, Theme, Renderer> for Transition<'_, Message, W, P> where Renderer: Renderer, P: Program, W: Widget<Message, Theme, Renderer>,`

#### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`] of the [`Widget`] in lengths.

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

#### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`]. Read more

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, renderer: &Renderer, shell: &mut Shell<'_, Message>, viewport: &Rectangle, )
```

Processes a runtime [`Event`](../canvas/enum.Event.md). Read more

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout, cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`] using the associated `Renderer`.

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, tree: &Tree, layout: Layout, cursor: Cursor, viewport: &Rectangle, renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`] of the [`Widget`]. Read more

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, viewport: &Rectangle, renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`] to the [`Widget`].

#### `overlay`

```rust
fn overlay<'a>( &'a mut self, tree: &'a mut Tree, layout: Layout, renderer: &Renderer, viewport: &Rectangle, translation: Vector, window: Size, ) -> Vec<Element<'a, Message, Theme, Renderer>>
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

- `impl<'a, Message, W, P> Freeze for Transition<'a, Message, W, P> where <P as Program>::Value: Freeze, W: Freeze,`
- `impl<'a, Message, W, P> !RefUnwindSafe for Transition<'a, Message, W, P>`
- `impl<'a, Message, W, P> !Send for Transition<'a, Message, W, P>`
- `impl<'a, Message, W, P> !Sync for Transition<'a, Message, W, P>`
- `impl<'a, Message, W, P> Unpin for Transition<'a, Message, W, P> where <P as Program>::Value: Unpin, W: Unpin,`
- `impl<'a, Message, W, P> UnsafeUnpin for Transition<'a, Message, W, P> where <P as Program>::Value: UnsafeUnpin, W: UnsafeUnpin,`
- `impl<'a, Message, W, P> !UnwindSafe for Transition<'a, Message, W, P>`
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
