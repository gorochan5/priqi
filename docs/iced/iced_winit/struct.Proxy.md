---
title: "Proxy"
kind: struct
path: "iced_winit::Proxy"
crate: "iced_winit"
module: "iced_winit"
file: "iced_winit/struct.Proxy.md"
docs_url: "https://docs.iced.rs/iced_winit/struct.Proxy.html"
source_file: "src/iced_winit/proxy.rs"
source_lines: "14-18"
---

# `iced_winit::Proxy`

```rust
pub struct Proxy<T: 'static> { /* private fields */ }
```

An event loop proxy with backpressure that implements `Sink`.

**Source:** [proxy.rs](../src/iced_winit/proxy.rs.md) (lines 14-18)

## Methods

### `impl<T: 'static> Proxy<T>`

#### `new`

```rust
pub fn new(raw: EventLoopProxy<Action<T>>) -> (Self, impl Future<Output = ()>)
```

Creates a new [`Proxy`](struct.Proxy.md) from an `EventLoopProxy`.

#### `input`

```rust
pub fn input(&self) -> Sender<Action<T>>
```

Creates an input channel for the [`Proxy`](struct.Proxy.md).

#### `send`

```rust
pub fn send(&self, value: T)
```

Sends a value to the event loop.

Note: This skips the backpressure mechanism with an unbounded channel. Use sparingly!

#### `send_action`

```rust
pub fn send_action(&self, action: Action<T>)
```

Sends an action to the event loop.

Note: This skips the backpressure mechanism with an unbounded channel. Use sparingly!

#### `free_slots`

```rust
pub fn free_slots(&mut self, amount: usize)
```

Frees an amount of slots for additional messages to be queued in this [`Proxy`](struct.Proxy.md).

## Trait Implementations

### `impl<T: 'static> Clone for Proxy<T>`

#### `clone`

```rust
fn clone(&self) -> Self
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T: Debug + 'static> Debug for Proxy<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T> Notifier for Proxy<T> where T: Send,`

#### `tick`

```rust
fn tick(&self)
```

Requests for `Renderer::tick` to be called by the [`Shell`].

#### `request_redraw`

```rust
fn request_redraw(&self)
```

Requests for all windows of the [`Shell`] to be redrawn.

#### `invalidate_layout`

```rust
fn invalidate_layout(&self)
```

Requests for all layouts of the [`Shell`] to be recomputed.

### `impl<T: 'static> Sink<Action<T>> for Proxy<T>`

#### `Error`

```rust
type Error = SendError
```

The type of value produced by the sink when an error occurs.

#### `poll_ready`

```rust
fn poll_ready( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Self::Error>>
```

Attempts to prepare the `Sink` to receive a value. Read more

#### `start_send`

```rust
fn start_send( self: Pin<&mut Self>, action: Action<T>, ) -> Result<(), Self::Error>
```

Begin the process of sending a value to the sink. Each call to this function must be preceded by a successful call to `poll_ready` which returned `Poll::Ready(Ok(()))`. Read more

#### `poll_flush`

```rust
fn poll_flush( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Self::Error>>
```

Flush any remaining output from this sink. Read more

#### `poll_close`

```rust
fn poll_close( self: Pin<&mut Self>, _cx: &mut Context<'_>, ) -> Poll<Result<(), Self::Error>>
```

Flush any remaining output and close this sink, if necessary. Read more

## Auto Trait Implementations

- `impl<T> Freeze for Proxy<T>`
- `impl<T> !RefUnwindSafe for Proxy<T>`
- `impl<T> Send for Proxy<T> where T: Send,`
- `impl<T> Sync for Proxy<T> where T: Send,`
- `impl<T> Unpin for Proxy<T>`
- `impl<T> UnsafeUnpin for Proxy<T>`
- `impl<T> !UnwindSafe for Proxy<T>`
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

### `impl<T, Item> SinkExt<Item> for T where T: Sink<Item> + ?Sized,`

#### `with`

```rust
fn with<U, Fut, F, E>(self, f: F) -> With<Self, Item, U, Fut, F> where F: FnMut(U) -> Fut, Fut: Future<Output = Result<Item, E>>, E: From<Self::Error>, Self: Sized,
```

Composes a function *in front of* the sink. Read more

#### `with_flat_map`

```rust
fn with_flat_map<U, St, F>(self, f: F) -> WithFlatMap<Self, Item, U, St, F> where F: FnMut(U) -> St, St: Stream<Item = Result<Item, Self::Error>>, Self: Sized,
```

Composes a function *in front of* the sink. Read more

#### `sink_map_err`

```rust
fn sink_map_err<E, F>(self, f: F) -> SinkMapErr<Self, F> where F: FnOnce(Self::Error) -> E, Self: Sized,
```

Transforms the error returned by the sink.

#### `sink_err_into`

```rust
fn sink_err_into<E>(self) -> SinkErrInto<Self, Item, E> where Self: Sized, Self::Error: Into<E>,
```

Map this sink’s error to a different error type using the `Into` trait. Read more

#### `buffer`

```rust
fn buffer(self, capacity: usize) -> Buffer<Self, Item> where Self: Sized,
```

*Available on crate feature alloc only.*

Adds a fixed-size buffer to the current sink. Read more

#### `close`

```rust
fn close(&mut self) -> Close<'_, Self, Item> where Self: Unpin,
```

Close the sink.

#### `fanout`

```rust
fn fanout<Si>(self, other: Si) -> Fanout<Self, Si> where Self: Sized, Item: Clone, Si: Sink<Item, Error = Self::Error>,
```

Fanout items to multiple sinks. Read more

#### `flush`

```rust
fn flush(&mut self) -> Flush<'_, Self, Item> where Self: Unpin,
```

Flush the sink, processing all pending items. Read more

#### `send`

```rust
fn send(&mut self, item: Item) -> Send<'_, Self, Item> where Self: Unpin,
```

A future that completes after the given item has been fully processed into the sink, including flushing. Read more

#### `feed`

```rust
fn feed(&mut self, item: Item) -> Feed<'_, Self, Item> where Self: Unpin,
```

A future that completes after the given item has been received by the sink. Read more

#### `send_all`

```rust
fn send_all<'a, St>(&'a mut self, stream: &'a mut St) -> SendAll<'a, Self, St> where St: TryStream<Ok = Item, Error = Self::Error> + Stream + Unpin + ?Sized, Self: Unpin,
```

A future that completes after the given stream has been fully processed into the sink, including flushing. Read more

#### `left_sink`

```rust
fn left_sink<Si2>(self) -> Either<Self, Si2> where Si2: Sink<Item, Error = Self::Error>, Self: Sized,
```

Wrap this sink in an `Either` sink, making it the left-hand variant of that `Either`. Read more

#### `right_sink`

```rust
fn right_sink<Si1>(self) -> Either<Si1, Self> where Si1: Sink<Item, Error = Self::Error>, Self: Sized,
```

Wrap this stream in an `Either` stream, making it the right-hand variant of that `Either`. Read more

#### `poll_ready_unpin`

```rust
fn poll_ready_unpin( &mut self, cx: &mut Context<'_>, ) -> Poll<Result<(), Self::Error>> where Self: Unpin,
```

A convenience method for calling [`Sink::poll_ready`] on [`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html) sink types.

#### `start_send_unpin`

```rust
fn start_send_unpin(&mut self, item: Item) -> Result<(), Self::Error> where Self: Unpin,
```

A convenience method for calling [`Sink::start_send`] on [`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html) sink types.

#### `poll_flush_unpin`

```rust
fn poll_flush_unpin( &mut self, cx: &mut Context<'_>, ) -> Poll<Result<(), Self::Error>> where Self: Unpin,
```

A convenience method for calling [`Sink::poll_flush`] on [`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html) sink types.

#### `poll_close_unpin`

```rust
fn poll_close_unpin( &mut self, cx: &mut Context<'_>, ) -> Poll<Result<(), Self::Error>> where Self: Unpin,
```

A convenience method for calling [`Sink::poll_close`] on [`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html) sink types.

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
