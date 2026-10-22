---
title: "Simulator"
kind: struct
path: "iced_test::simulator::Simulator"
crate: "iced_test"
module: "iced_test::simulator"
file: "iced_test/simulator/struct.Simulator.md"
docs_url: "https://docs.iced.rs/iced_test/simulator/struct.Simulator.html"
source_file: "src/iced_test/simulator.rs"
source_lines: "27-33"
---

# `iced_test::simulator::Simulator`

```rust
pub struct Simulator<'a, Message, Theme = Theme, Renderer = Renderer> { /* private fields */ }
```

A user interface that can be interacted with and inspected programmatically.

**Source:** [simulator.rs](../../src/iced_test/simulator.rs.md) (lines 27-33)

## Methods

### `impl<'a, Message, Theme, Renderer> Simulator<'a, Message, Theme, Renderer> where Message: 'a, Theme: Base + 'a, Renderer: Renderer + Headless + 'a,`

#### `new`

```rust
pub fn new(element: impl Widget<Message, Theme, Renderer> + 'a) -> Self
```

Creates a new [`Simulator`](struct.Simulator.md) with default [`Settings`] and a default size (1024x768).

#### `with_settings`

```rust
pub fn with_settings( settings: Settings, element: impl Widget<Message, Theme, Renderer> + 'a, ) -> Self
```

Creates a new [`Simulator`](struct.Simulator.md) with the given [`Settings`] and a default size (1024x768).

#### `with_size`

```rust
pub fn with_size( settings: Settings, size: impl Into<Size>, element: impl Widget<Message, Theme, Renderer> + 'a, ) -> Self
```

Creates a new [`Simulator`](struct.Simulator.md) with the given [`Settings`] and size.

#### `find`

```rust
pub fn find<S>(&mut self, selector: S) -> Result<S::Output, Error> where S: Selector + Send, S::Output: Clone + Send,
```

Finds the target of the given widget [`Selector`](../trait.Selector.md) in the [`Simulator`](struct.Simulator.md).

#### `point_at`

```rust
pub fn point_at(&mut self, position: impl Into<Point>)
```

Points the mouse cursor at the given position in the [`Simulator`](struct.Simulator.md).

This does *not* produce mouse movement events!

#### `operate`

```rust
pub fn operate(&mut self, operation: &mut dyn Operation)
```

Applies a [`widget::Operation`] to the [`Simulator`](struct.Simulator.md)’s widget tree.

#### `rebuild`

```rust
pub fn rebuild( self, element: impl Widget<Message, Theme, Renderer> + 'a, ) -> Self
```

Rebuilds the [`Simulator`](struct.Simulator.md)’s user interface with a new `element`, preserving the state of widgets with an unchanged id.

#### `resize`

```rust
pub fn resize(self, size: impl Into<Size>) -> Self
```

Resizes the [`Simulator`](struct.Simulator.md)’s window to the given `size`, relaying out the user interface and preserving widget state.

#### `click`

```rust
pub fn click<S>(&mut self, selector: S) -> Result<S::Output, Error> where S: Selector + Send, S::Output: Bounded + Clone + Send + Sync + 'static,
```

Clicks the [`Bounded`] target found by the given [`Selector`](../trait.Selector.md), if any.

This consists in:

- Pointing the mouse cursor at the center of the [`Bounded`] target.
- Simulating a [`click`](fn.click.md).

#### `tap_key`

```rust
pub fn tap_key(&mut self, key: impl Into<Key>) -> Status
```

Simulates a key press, followed by a release, in the [`Simulator`](struct.Simulator.md).

#### `typewrite`

```rust
pub fn typewrite(&mut self, text: &str) -> Status
```

Simulates a user typing in the keyboard the given text in the [`Simulator`](struct.Simulator.md).

#### `scroll`

```rust
pub fn scroll(&mut self, delta: ScrollDelta) -> Status
```

Scrolls with the given `delta` in the [`Simulator`](struct.Simulator.md).

The mouse cursor must be over the content being scrolled for the scroll to be applied, e.g. via [`Self::point_at`](struct.Simulator.md).

#### `simulate`

```rust
pub fn simulate( &mut self, events: impl IntoIterator<Item = Event>, ) -> Vec<Status>
```

Simulates the given raw sequence of events in the [`Simulator`](struct.Simulator.md).

#### `draw`

```rust
pub fn draw(&mut self, theme: &Theme)
```

Draws the interface in the [`Simulator`](struct.Simulator.md) with the given theme.

A `RedrawRequested` event is processed before drawing.

#### `snapshot`

```rust
pub fn snapshot(&mut self, theme: &Theme) -> Result<Snapshot, Error>
```

Draws and takes a [`Snapshot`](struct.Snapshot.md) of the interface in the [`Simulator`](struct.Simulator.md).

#### `into_messages`

```rust
pub fn into_messages( self, ) -> impl Iterator<Item = Message> + use<Message, Theme, Renderer>
```

Turns the [`Simulator`](struct.Simulator.md) into the sequence of messages produced by any interactions.

#### `drain`

```rust
pub fn drain(&mut self) -> impl Iterator<Item = Message>
```

Returns the messages published so far, clearing the queue.

## Auto Trait Implementations

- `impl<'a, Message, Theme, Renderer> Freeze for Simulator<'a, Message, Theme, Renderer> where Renderer: Freeze,`
- `impl<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !RefUnwindSafe for Simulator<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Send for Simulator<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Sync for Simulator<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> Unpin for Simulator<'a, Message, Theme, Renderer> where Renderer: Unpin, Message: Unpin,`
- `impl<'a, Message, Theme, Renderer> UnsafeUnpin for Simulator<'a, Message, Theme, Renderer> where Renderer: UnsafeUnpin,`
- `impl<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !UnwindSafe for Simulator<'a, Message, Theme, Renderer>`
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
