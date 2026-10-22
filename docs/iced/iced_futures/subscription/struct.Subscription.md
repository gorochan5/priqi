---
title: "Subscription"
kind: struct
path: "iced_futures::subscription::Subscription"
crate: "iced_futures"
module: "iced_futures::subscription"
file: "iced_futures/subscription/struct.Subscription.md"
docs_url: "https://docs.iced.rs/iced_futures/subscription/struct.Subscription.html"
source_file: "src/iced_futures/subscription.rs"
source_lines: "121-123"
---

# `iced_futures::subscription::Subscription`

```rust
pub struct Subscription<T> { /* private fields */ }
```

A request to listen to external events.

Besides performing async actions on demand with `Task`, most applications also need to listen to external events passively.

A [`Subscription`](struct.Subscription.md) is normally provided to some runtime, like a `Task`, and it will generate events as long as the user keeps requesting it.

For instance, you can use a [`Subscription`](struct.Subscription.md) to listen to a `WebSocket` connection, keyboard presses, mouse events, time ticks, etc.

## The Lifetime of a [`Subscription`](struct.Subscription.md)

Much like a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) or a [`Stream`], a [`Subscription`](struct.Subscription.md) does not produce any effects on its own. For a [`Subscription`](struct.Subscription.md) to run, it must be returned to the iced runtime—normally in the `subscription` function of an `application` or a `daemon`.

When a [`Subscription`](struct.Subscription.md) is provided to the runtime for the first time, the runtime will start running it asynchronously. Running a [`Subscription`](struct.Subscription.md) consists in building its underlying [`Stream`] and executing it in an async runtime.

Therefore, you can think of a [`Subscription`](struct.Subscription.md) as a “stream builder”. It simply represents a way to build a certain [`Stream`] together with some way to *identify* it.

Identification is important because when a specific [`Subscription`](struct.Subscription.md) stops being returned to the iced runtime, the runtime will kill its associated [`Stream`]. The runtime uses the identity of a [`Subscription`](struct.Subscription.md) to keep track of it.

This way, iced allows you to declaratively **subscribe** to particular streams of data temporarily and whenever necessary.

```rust
use iced::time::{self, Duration, Instant};
use iced::Subscription;

struct State {
    timer_enabled: bool,
}

fn subscription(state: &State) -> Subscription<Instant> {
    if state.timer_enabled {
        time::every(Duration::from_secs(1))
    } else {
        Subscription::none()
    }
}
```

**Source:** [subscription.rs](../../src/iced_futures/subscription.rs.md) (lines 121-123)

## Methods

### `impl<T> Subscription<T>`

#### `none`

```rust
pub fn none() -> Self
```

Returns an empty [`Subscription`](struct.Subscription.md) that will not produce any output.

#### `run`

```rust
pub fn run<S>(builder: fn() -> S) -> Self where S: Stream<Item = T> + MaybeSend + 'static, T: 'static,
```

Returns a [`Subscription`](struct.Subscription.md) that will call the given function to create and asynchronously run the given [`Stream`].

###### Creating an asynchronous worker with bidirectional communication

You can leverage this helper to create a [`Subscription`](struct.Subscription.md) that spawns an asynchronous worker in the background and establish a channel of communication with an `iced` application.

You can achieve this by creating an `mpsc` channel inside the closure and returning the `Sender` as a `Message` for the `Application`:

```rust
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::futures::Stream;
use iced::stream;
use iced::Subscription;

pub enum Event {
    Ready(mpsc::Sender<Input>),
    WorkFinished,
    // ...
}

enum Input {
    DoSomeWork,
    // ...
}

fn some_worker() -> impl Stream<Item = Event> {
    stream::channel(100, async |mut output| {
        // Create channel
        let (sender, mut receiver) = mpsc::channel(100);

        // Send the sender back to the application
        output.send(Event::Ready(sender)).await;

        loop {
            use iced_futures::futures::StreamExt;

            // Read next input sent from `Application`
            let input = receiver.select_next_some().await;

            match input {
                Input::DoSomeWork => {
                    // Do some async work...

                    // Finally, we can optionally produce a message to tell the
                    // `Application` the work is done
                    output.send(Event::WorkFinished).await;
                }
            }
        }
    })
}

fn subscription() -> Subscription<Event> {
    Subscription::run(some_worker)
}
```

Check out the [`websocket`](https://github.com/iced-rs/iced/tree/master/examples/websocket) example, which showcases this pattern to maintain a `WebSocket` connection open.

#### `run_with`

```rust
pub fn run_with<D, S>(data: D, builder: fn(&D) -> S) -> Self where D: Hash + 'static, S: Stream<Item = T> + MaybeSend + 'static, T: 'static,
```

Returns a [`Subscription`](struct.Subscription.md) that will create and asynchronously run the given [`Stream`].

Both the `data` and the function pointer will be used to uniquely identify the [`Subscription`](struct.Subscription.md).

#### `batch`

```rust
pub fn batch(subscriptions: impl IntoIterator<Item = Subscription<T>>) -> Self
```

Batches all the provided subscriptions and returns the resulting [`Subscription`](struct.Subscription.md).

#### `with`

```rust
pub fn with<A>(self, value: A) -> Subscription<(A, T)> where T: 'static, A: Hash + Clone + Send + Sync + 'static,
```

Adds a value to the [`Subscription`](struct.Subscription.md) context.

The value will be part of the identity of a [`Subscription`](struct.Subscription.md).

#### `map`

```rust
pub fn map<F, A>(self, f: F) -> Subscription<A> where T: 'static, F: Fn(T) -> A + MaybeSend + Clone + 'static, A: 'static,
```

Transforms the [`Subscription`](struct.Subscription.md) output with the given function.

The closure provided must be a non-capturing closure.

#### `filter_map`

```rust
pub fn filter_map<F, A>(self, f: F) -> Subscription<A> where T: MaybeSend + 'static, F: Fn(T) -> Option<A> + MaybeSend + Clone + 'static, A: MaybeSend + 'static,
```

Transforms the [`Subscription`](struct.Subscription.md) output with the given function, yielding only values only when the function returns `Some(A)`.

The closure provided must be a non-capturing closure.

#### `units`

```rust
pub fn units(&self) -> usize
```

Returns the amount of recipe units in this [`Subscription`](struct.Subscription.md).

## Trait Implementations

### `impl<T> Debug for Subscription<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T> Default for Subscription<T>`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations

- `impl<T> Freeze for Subscription<T>`
- `impl<T> !RefUnwindSafe for Subscription<T>`
- `impl<T> !Send for Subscription<T>`
- `impl<T> !Sync for Subscription<T>`
- `impl<T> Unpin for Subscription<T>`
- `impl<T> UnsafeUnpin for Subscription<T>`
- `impl<T> !UnwindSafe for Subscription<T>`
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
