---
title: "Runtime"
kind: struct
path: "iced_futures::Runtime"
crate: "iced_futures"
module: "iced_futures"
file: "iced_futures/struct.Runtime.md"
docs_url: "https://docs.iced.rs/iced_futures/struct.Runtime.html"
source_file: "src/iced_futures/runtime.rs"
source_lines: "15-20"
---

# `iced_futures::Runtime`

```rust
pub struct Runtime<Executor, Sender, Message> { /* private fields */ }
```

A batteries-included runtime of commands and subscriptions.

If you have an [`Executor`](executor/trait.Executor.md), a [`Runtime`](struct.Runtime.md) can be leveraged to run any `Command` or [`Subscription`](subscription/struct.Subscription.md) and get notified of the results!

**Source:** [runtime.rs](../src/iced_futures/runtime.rs.md) (lines 15-20)

## Methods

### `impl<Executor, Sender, Message> Runtime<Executor, Sender, Message> where Executor: Executor, Sender: Sink<Message, Error = SendError> + Unpin + MaybeSend + Clone + 'static, Message: MaybeSend + 'static,`

#### `new`

```rust
pub fn new(executor: Executor, sender: Sender) -> Self
```

Creates a new empty [`Runtime`](struct.Runtime.md).

You need to provide:

- an [`Executor`](executor/trait.Executor.md) to spawn futures
- a `Sender` implementing `Sink` to receive the results

#### `enter`

```rust
pub fn enter<R>(&self, f: impl FnOnce() -> R) -> R
```

Runs the given closure inside the [`Executor`](executor/trait.Executor.md) of the [`Runtime`](struct.Runtime.md).

See [`Executor::enter`](executor/trait.Executor.md) to learn more.

#### `block_on`

```rust
pub fn block_on<T>(&mut self, future: impl Future<Output = T>) -> T
```

*Available on non-WebAssembly only.*

Runs a future to completion in the current thread within the [`Runtime`](struct.Runtime.md).

#### `run`

```rust
pub fn run(&mut self, stream: BoxStream<Message>)
```

Runs a [`Stream`](type.BoxStream.md) in the [`Runtime`](struct.Runtime.md) until completion.

The resulting `Message`s will be forwarded to the `Sender` of the [`Runtime`](struct.Runtime.md).

#### `send`

```rust
pub fn send(&mut self, message: Message)
```

Sends a message concurrently through the [`Runtime`](struct.Runtime.md).

#### `track`

```rust
pub fn track( &mut self, recipes: impl IntoIterator<Item = Box<dyn Recipe<Output = Message>>>, )
```

Tracks a [`Subscription`](subscription/struct.Subscription.md) in the [`Runtime`](struct.Runtime.md).

It will spawn new streams or close old ones as necessary! See [`Tracker::update`](subscription/struct.Tracker.md) to learn more about this!

#### `broadcast`

```rust
pub fn broadcast(&mut self, event: Event)
```

Broadcasts an event to all the subscriptions currently alive in the [`Runtime`](struct.Runtime.md).

See [`Tracker::broadcast`](subscription/struct.Tracker.md) to learn more.

## Trait Implementations

### `impl<Executor: Debug, Sender: Debug, Message: Debug> Debug for Runtime<Executor, Sender, Message>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl<Executor, Sender, Message> Freeze for Runtime<Executor, Sender, Message> where Executor: Freeze, Sender: Freeze,`
- `impl<Executor, Sender, Message> !RefUnwindSafe for Runtime<Executor, Sender, Message>`
- `impl<Executor, Sender, Message> Send for Runtime<Executor, Sender, Message> where Executor: Send, Sender: Send, Message: Send,`
- `impl<Executor, Sender, Message> Sync for Runtime<Executor, Sender, Message> where Executor: Sync, Sender: Sync, Message: Sync,`
- `impl<Executor, Sender, Message> Unpin for Runtime<Executor, Sender, Message> where Executor: Unpin, Sender: Unpin, Message: Unpin,`
- `impl<Executor, Sender, Message> UnsafeUnpin for Runtime<Executor, Sender, Message> where Executor: UnsafeUnpin, Sender: UnsafeUnpin,`
- `impl<Executor, Sender, Message> !UnwindSafe for Runtime<Executor, Sender, Message>`
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

### `impl<T> From<T> for T`

#### `from`

```rust
fn from(t: T) -> T
```

Returns the argument unchanged.

### `impl<T, U> Into<U> for T where U: From<T>,`

#### `into`

```rust
fn into(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

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

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`
