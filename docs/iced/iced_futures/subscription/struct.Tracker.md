---
title: "Tracker"
kind: struct
path: "iced_futures::subscription::Tracker"
crate: "iced_futures"
module: "iced_futures::subscription"
file: "iced_futures/subscription/struct.Tracker.md"
docs_url: "https://docs.iced.rs/iced_futures/subscription/struct.Tracker.html"
source_file: "src/iced_futures/subscription/tracker.rs"
source_lines: "18-20"
---

# `iced_futures::subscription::Tracker`

```rust
pub struct Tracker { /* private fields */ }
```

A registry of subscription streams.

If you have an application that continuously returns a [`Subscription`](struct.Subscription.md), you can use a [`Tracker`](struct.Tracker.md) to keep track of the different recipes and keep its executions alive.

**Source:** [tracker.rs](../../src/iced_futures/subscription/tracker.rs.md) (lines 18-20)

## Methods

### `impl Tracker`

#### `new`

```rust
pub fn new() -> Self
```

Creates a new empty [`Tracker`](struct.Tracker.md).

#### `update`

```rust
pub fn update<Message, Receiver>( &mut self, recipes: impl Iterator<Item = Box<dyn Recipe<Output = Message>>>, receiver: Receiver, ) -> Vec<BoxFuture<()>> ⓘ where Message: 'static + MaybeSend, Receiver: 'static + Sink<Message, Error = SendError> + Unpin + MaybeSend + Clone,
```

Updates the [`Tracker`](struct.Tracker.md) with the given [`Subscription`](struct.Subscription.md).

A [`Subscription`](struct.Subscription.md) can cause new streams to be spawned or old streams to be closed.

The [`Tracker`](struct.Tracker.md) keeps track of these streams between calls to this method:

- If the provided [`Subscription`](struct.Subscription.md) contains a new [`Recipe`](trait.Recipe.md) that is currently not being run, it will spawn a new stream and keep it alive.
- On the other hand, if a [`Recipe`](trait.Recipe.md) is currently in execution and the provided [`Subscription`](struct.Subscription.md) does not contain it anymore, then the [`Tracker`](struct.Tracker.md) will close and drop the relevant stream.

It returns a list of futures that need to be spawned to materialize the [`Tracker`](struct.Tracker.md) changes.

#### `broadcast`

```rust
pub fn broadcast(&mut self, event: Event)
```

Broadcasts an event to the subscriptions currently alive.

A subscription’s [`Recipe::stream`](trait.Recipe.md) always receives a stream of events as input. This stream can be used by some subscription to listen to shell events.

This method publishes the given event to all the subscription streams currently open.

## Trait Implementations

### `impl Debug for Tracker`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Tracker`

#### `default`

```rust
fn default() -> Tracker
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations

- `impl Freeze for Tracker`
- `impl !RefUnwindSafe for Tracker`
- `impl Send for Tracker`
- `impl Sync for Tracker`
- `impl Unpin for Tracker`
- `impl UnsafeUnpin for Tracker`
- `impl !UnwindSafe for Tracker`
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
