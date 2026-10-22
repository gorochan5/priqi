---
title: "Animation"
kind: struct
path: "iced::Animation"
crate: "iced"
module: "iced"
file: "iced/struct.Animation.md"
docs_url: "https://docs.iced.rs/iced/struct.Animation.html"
---

# `iced::Animation`

```rust
pub struct Animation<T> where
    T: Clone + Copy + PartialEq + FloatRepresentable, { /* private fields */ }
```

The animation of some particular state.

It tracks state changes and allows projecting interpolated values through time.

## Methods

### `impl<T> Animation<T> where T: Clone + Copy + PartialEq + FloatRepresentable,`

#### `new`

```rust
pub fn new(state: T) -> Animation<T>
```

Creates a new [`Animation`](struct.Animation.md) with the given initial state.

#### `easing`

```rust
pub fn easing(self, easing: Easing) -> Animation<T>
```

Sets the [`Easing`](animation/enum.Easing.md) function of the [`Animation`](struct.Animation.md).

See the [Easing Functions Cheat Sheet](https://easings.net) for details!

#### `very_quick`

```rust
pub fn very_quick(self) -> Animation<T>
```

Sets the duration of the [`Animation`](struct.Animation.md) to 100ms.

#### `quick`

```rust
pub fn quick(self) -> Animation<T>
```

Sets the duration of the [`Animation`](struct.Animation.md) to 200ms.

#### `slow`

```rust
pub fn slow(self) -> Animation<T>
```

Sets the duration of the [`Animation`](struct.Animation.md) to 400ms.

#### `very_slow`

```rust
pub fn very_slow(self) -> Animation<T>
```

Sets the duration of the [`Animation`](struct.Animation.md) to 500ms.

#### `duration`

```rust
pub fn duration(self, duration: Duration) -> Animation<T>
```

Sets the duration of the [`Animation`](struct.Animation.md) to the given value.

#### `delay`

```rust
pub fn delay(self, duration: Duration) -> Animation<T>
```

Sets a delay for the [`Animation`](struct.Animation.md).

#### `repeat`

```rust
pub fn repeat(self, repetitions: u32) -> Animation<T>
```

Makes the [`Animation`](struct.Animation.md) repeat a given amount of times.

Providing 1 repetition plays the animation twice in total.

#### `repeat_forever`

```rust
pub fn repeat_forever(self) -> Animation<T>
```

Makes the [`Animation`](struct.Animation.md) repeat forever.

#### `auto_reverse`

```rust
pub fn auto_reverse(self) -> Animation<T>
```

Makes the [`Animation`](struct.Animation.md) automatically reverse when repeating.

#### `go`

```rust
pub fn go(self, new_state: T, at: Instant) -> Animation<T>
```

Transitions the [`Animation`](struct.Animation.md) from its current state to the given new state at the given time.

#### `go_mut`

```rust
pub fn go_mut(&mut self, new_state: T, at: Instant)
```

Transitions the [`Animation`](struct.Animation.md) from its current state to the given new state at the given time, by reference.

#### `is_animating`

```rust
pub fn is_animating(&self, at: Instant) -> bool
```

Returns true if the [`Animation`](struct.Animation.md) is currently in progress.

An [`Animation`](struct.Animation.md) is in progress when it is transitioning to a different state.

#### `interpolate_with`

```rust
pub fn interpolate_with<I>(&self, f: impl Fn(T) -> I, at: Instant) -> I where I: Interpolable,
```

Projects the [`Animation`](struct.Animation.md) into an interpolated value at the given [`Instant`](time/struct.Instant.md); using the closure provided to calculate the different keyframes of interpolated values.

If the [`Animation`](struct.Animation.md) state is a `bool`, you can use the simpler [`interpolate`](struct.Animation.md) method.

#### `value`

```rust
pub fn value(&self) -> T
```

Retuns the current state of the [`Animation`](struct.Animation.md).

### `impl Animation<f32>`

#### `interpolate`

```rust
pub fn interpolate(&self, at: Instant) -> f32
```

Projects the [`Animation`](struct.Animation.md) into an interpolated value at the given [`Instant`](time/struct.Instant.md).

### `impl Animation<bool>`

#### `interpolate`

```rust
pub fn interpolate<I>(&self, start: I, end: I, at: Instant) -> I where I: Interpolable + Clone,
```

Projects the [`Animation`](struct.Animation.md) into an interpolated value at the given [`Instant`](time/struct.Instant.md); using the `start` and `end` values as the origin and destination keyframes.

#### `remaining`

```rust
pub fn remaining(&self, at: Instant) -> Duration
```

Returns the remaining [`Duration`](time/struct.Duration.md) of the [`Animation`](struct.Animation.md).

## Trait Implementations

### `impl<T> Clone for Animation<T> where T: Clone + Copy + PartialEq + FloatRepresentable,`

#### `clone`

```rust
fn clone(&self) -> Animation<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T> Debug for Animation<T> where T: Debug + Clone + Copy + PartialEq + FloatRepresentable,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<I> Program for Animation<I> where I: FloatRepresentable + Clone + Copy + PartialEq + 'static,`

#### `Value`

```rust
type Value = I
```

The type of value that the [`Program`](widget/transition/trait.Program.md) animates.

#### `go`

```rust
fn go(&mut self, value: <Animation<I> as Program>::Value, now: Instant)
```

Transitions the [`Program`](widget/transition/trait.Program.md) from its current state towards the given value at the given time.

#### `is_animating`

```rust
fn is_animating(&self, now: Instant) -> bool
```

Returns true if the [`Program`](widget/transition/trait.Program.md) is currently in progress.

## Auto Trait Implementations

- `impl<T> Freeze for Animation<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for Animation<T> where T: RefUnwindSafe,`
- `impl<T> Send for Animation<T> where T: Send,`
- `impl<T> Sync for Animation<T> where T: Sync,`
- `impl<T> Unpin for Animation<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Animation<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for Animation<T> where T: UnwindSafe,`
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

### `impl<T> CloneToUninit for T where T: Clone,`

#### `clone_to_uninit`

```rust
unsafe fn clone_to_uninit(&self, dest: *mut u8)
```

*🔬This is a nightly-only experimental API. (clone_to_uninit)*

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

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

### `impl<T> DowncastSync for T where T: Any + Send + Sync,`

#### `into_any_arc`

```rust
fn into_any_arc(self: Arc<T>) -> Arc<dyn Any + Sync + Send>
```

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

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

Turns some type into the initial state of some [`Application`](application/struct.Application.md).

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

### `impl<T> ToOwned for T where T: Clone,`

#### `Owned`

```rust
type Owned = T
```

The resulting type after obtaining ownership.

#### `to_owned`

```rust
fn to_owned(&self) -> T
```

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

#### `clone_into`

```rust
fn clone_into(&self, target: &mut T)
```

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

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

### `impl<T> MaybeClone for T where T: Clone,`

### `impl<T> MaybeDebug for T where T: Debug,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
