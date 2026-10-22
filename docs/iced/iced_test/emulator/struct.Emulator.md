---
title: "Emulator"
kind: struct
path: "iced_test::emulator::Emulator"
crate: "iced_test"
module: "iced_test::emulator"
file: "iced_test/emulator/struct.Emulator.md"
docs_url: "https://docs.iced.rs/iced_test/emulator/struct.Emulator.html"
source_file: "src/iced_test/emulator.rs"
source_lines: "37-47"
---

# `iced_test::emulator::Emulator`

```rust
pub struct Emulator<P: Program> { /* private fields */ }
```

A headless runtime that can run iced applications and execute [instructions](../instruction/enum.Instruction.md).

An [`Emulator`](struct.Emulator.md) runs its program as faithfully as possible to the real thing. It will run subscriptions and tasks with the `Executor` of the [`Program`].

If you want to run a simulation without side effects, use a [`Simulator`](../simulator/struct.Simulator.md) instead.

**Source:** [emulator.rs](../../src/iced_test/emulator.rs.md) (lines 37-47)

## Methods

### `impl<P: Program + 'static> Emulator<P>`

#### `new`

```rust
pub fn new( sender: Sender<Event<P>>, program: &P, mode: Mode, size: Size, ) -> Emulator<P>
```

Creates a new [`Emulator`](struct.Emulator.md) of the [`Program`] with the given [`Mode`](enum.Mode.md) and [`Size`].

The [`Emulator`](struct.Emulator.md) will send [`Event`](enum.Event.md) notifications through the provided [`mpsc::Sender`].

When the [`Emulator`](struct.Emulator.md) has finished booting, an [`Event::Ready`](enum.Event.md) will be produced.

#### `with_preset`

```rust
pub fn with_preset( sender: Sender<Event<P>>, program: &P, mode: Mode, size: Size, preset: Option<&Preset<P::State, P::Message>>, ) -> Emulator<P>
```

Creates a new [`Emulator`](struct.Emulator.md) analogously to [`new`](struct.Emulator.md), but it also takes a [`program::Preset`] that will be used as the initial state.

When the [`Emulator`](struct.Emulator.md) has finished booting, an [`Event::Ready`](enum.Event.md) will be produced.

#### `update`

```rust
pub fn update(&mut self, program: &P, message: P::Message)
```

Updates the state of the [`Emulator`](struct.Emulator.md) program.

This is equivalent to calling the [`Program::update`] function, resubscribing to any subscriptions, and running the resulting tasks concurrently.

#### `perform`

```rust
pub fn perform(&mut self, program: &P, action: Action<P>)
```

Performs an [`Action`](struct.Action.md).

Whenever an [`Emulator`](struct.Emulator.md) sends an [`Event::Action`](enum.Event.md), this method must be called to proceed with the execution.

#### `run`

```rust
pub fn run(&mut self, program: &P, instruction: &Instruction)
```

Runs an [`Instruction`](../instruction/enum.Instruction.md).

If the [`Instruction`](../instruction/enum.Instruction.md) executes successfully, an [`Event::Ready`](enum.Event.md) will be produced by the [`Emulator`](struct.Emulator.md).

Otherwise, an [`Event::Failed`](enum.Event.md) will be triggered.

#### `view`

```rust
pub fn view( &self, program: &P, ) -> impl Widget<P::Message, P::Theme, P::Renderer> + '_
```

Returns the current view of the [`Emulator`](struct.Emulator.md).

#### `theme`

```rust
pub fn theme(&self, program: &P) -> Option<P::Theme>
```

Returns the current theme of the [`Emulator`](struct.Emulator.md).

#### `screenshot`

```rust
pub fn screenshot( &mut self, program: &P, theme: &P::Theme, scale_factor: f32, ) -> Screenshot
```

Takes a [`window::Screenshot`] of the current state of the [`Emulator`](struct.Emulator.md).

#### `state`

```rust
pub fn state(&self) -> &P::State
```

Returns a reference to the state of the [`Emulator`](struct.Emulator.md).

#### `into_state`

```rust
pub fn into_state(self) -> (P::State, Id)
```

Turns the [`Emulator`](struct.Emulator.md) into its internal state.

## Auto Trait Implementations

- `impl<P> Freeze for Emulator<P> where <P as Program>::State: Freeze, <P as Program>::Renderer: Freeze, <P as Program>::Executor: Freeze,`
- `impl<P> !RefUnwindSafe for Emulator<P>`
- `impl<P> !Send for Emulator<P>`
- `impl<P> !Sync for Emulator<P>`
- `impl<P> Unpin for Emulator<P> where <P as Program>::State: Unpin, <P as Program>::Renderer: Unpin, <P as Program>::Executor: Unpin, <P as Program>::Message: Unpin,`
- `impl<P> UnsafeUnpin for Emulator<P> where <P as Program>::State: UnsafeUnpin, <P as Program>::Renderer: UnsafeUnpin, <P as Program>::Executor: UnsafeUnpin,`
- `impl<P> !UnwindSafe for Emulator<P>`
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
