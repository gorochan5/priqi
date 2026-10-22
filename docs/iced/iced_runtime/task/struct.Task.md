---
title: "Task"
kind: struct
path: "iced_runtime::task::Task"
crate: "iced_runtime"
module: "iced_runtime::task"
file: "iced_runtime/task/struct.Task.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/struct.Task.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "24-27"
---

# `iced_runtime::task::Task`

```rust
pub struct Task<T> { /* private fields */ }
```

A set of concurrent actions to be performed by the iced runtime.

A [`Task`](struct.Task.md) *may* produce a bunch of values of type `T`.

**Source:** [task.rs](../../src/iced_runtime/task.rs.md) (lines 24-27)

## Methods

### `impl<T> Task<T>`

#### `none`

```rust
pub fn none() -> Self
```

Creates a [`Task`](struct.Task.md) that does nothing.

#### `done`

```rust
pub fn done(value: T) -> Self where T: MaybeSend + 'static,
```

Creates a new [`Task`](struct.Task.md) that instantly produces the given value.

#### `perform`

```rust
pub fn perform<A>( future: impl Future<Output = A> + MaybeSend + 'static, f: impl FnOnce(A) -> T + MaybeSend + 'static, ) -> Self where T: MaybeSend + 'static, A: MaybeSend + 'static,
```

Creates a [`Task`](struct.Task.md) that runs the given [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) to completion and maps its output with the given closure.

#### `run`

```rust
pub fn run<A>( stream: impl Stream<Item = A> + MaybeSend + 'static, f: impl Fn(A) -> T + MaybeSend + 'static, ) -> Self where T: 'static,
```

Creates a [`Task`](struct.Task.md) that runs the given [`Stream`] to completion and maps each item with the given closure.

#### `sip`

```rust
pub fn sip<S>( sipper: S, on_progress: impl FnMut(S::Item) -> T + MaybeSend + 'static, on_output: impl FnOnce(<S as Future>::Output) -> T + MaybeSend + 'static, ) -> Self where S: Core + MaybeSend + 'static, T: MaybeSend + 'static,
```

*Available on crate feature sipper only.*

Creates a [`Task`](struct.Task.md) that runs the given [`Sipper`] to completion, mapping progress with the first closure and the output with the second one.

#### `batch`

```rust
pub fn batch(tasks: impl IntoIterator<Item = Self>) -> Self where T: 'static,
```

Combines the given tasks and produces a single [`Task`](struct.Task.md) that will run all of them in parallel.

#### `map`

```rust
pub fn map<O>(self, f: impl FnMut(T) -> O + MaybeSend + 'static) -> Task<O> where T: MaybeSend + 'static, O: MaybeSend + 'static,
```

Maps the output of a [`Task`](struct.Task.md) with the given closure.

#### `then`

```rust
pub fn then<O>( self, f: impl FnMut(T) -> Task<O> + MaybeSend + 'static, ) -> Task<O> where T: MaybeSend + 'static, O: MaybeSend + 'static,
```

Performs a new [`Task`](struct.Task.md) for every output of the current [`Task`](struct.Task.md) using the given closure.

This is the monadic interface of [`Task`](struct.Task.md)—analogous to [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) and [`Stream`].

#### `chain`

```rust
pub fn chain(self, task: Self) -> Self where T: 'static,
```

Chains a new [`Task`](struct.Task.md) to be performed once the current one finishes completely.

#### `collect`

```rust
pub fn collect(self) -> Task<Vec<T>> where T: MaybeSend + 'static,
```

Creates a new [`Task`](struct.Task.md) that collects all the output of the current one into a [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html).

#### `discard`

```rust
pub fn discard<O>(self) -> Task<O> where T: MaybeSend + 'static, O: MaybeSend + 'static,
```

Creates a new [`Task`](struct.Task.md) that discards the result of the current one.

Useful if you only care about the side effects of a [`Task`](struct.Task.md).

#### `abortable`

```rust
pub fn abortable(self) -> (Self, Handle) where T: 'static,
```

Creates a new [`Task`](struct.Task.md) that can be aborted with the returned [`Handle`](struct.Handle.md).

#### `future`

```rust
pub fn future(future: impl Future<Output = T> + MaybeSend + 'static) -> Self where T: 'static,
```

Creates a new [`Task`](struct.Task.md) that runs the given [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) and produces its output.

#### `stream`

```rust
pub fn stream(stream: impl Stream<Item = T> + MaybeSend + 'static) -> Self where T: 'static,
```

Creates a new [`Task`](struct.Task.md) that runs the given [`Stream`] and produces each of its items.

#### `units`

```rust
pub fn units(&self) -> usize
```

Returns the amount of work “units” of the [`Task`](struct.Task.md).

### `impl<T> Task<Option<T>>`

#### `and_then`

```rust
pub fn and_then<A>( self, f: impl Fn(T) -> Task<A> + MaybeSend + 'static, ) -> Task<A> where T: MaybeSend + 'static, A: MaybeSend + 'static,
```

Executes a new [`Task`](struct.Task.md) after this one, only when it produces `Some` value.

The value is provided to the closure to create the subsequent [`Task`](struct.Task.md).

### `impl<T, E> Task<Result<T, E>>`

#### `and_then`

```rust
pub fn and_then<A>( self, f: impl Fn(T) -> Task<Result<A, E>> + MaybeSend + 'static, ) -> Task<Result<A, E>> where T: MaybeSend + 'static, E: MaybeSend + 'static, A: MaybeSend + 'static,
```

Executes a new [`Task`](struct.Task.md) after this one, only when it succeeds with an `Ok` value.

The success value is provided to the closure to create the subsequent [`Task`](struct.Task.md).

#### `map_err`

```rust
pub fn map_err<E2>( self, f: impl Fn(E) -> E2 + MaybeSend + 'static, ) -> Task<Result<T, E2>> where T: MaybeSend + 'static, E: MaybeSend + 'static, E2: MaybeSend + 'static,
```

Maps the error type of this [`Task`](struct.Task.md) to a different one using the given function.

## Trait Implementations

### `impl<T> Debug for Task<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T> Default for Task<T>`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T> From<()> for Task<T>`

#### `from`

```rust
fn from(_value: ()) -> Self
```

Converts to this type from the input type.

## Auto Trait Implementations

- `impl<T> Freeze for Task<T>`
- `impl<T> !RefUnwindSafe for Task<T>`
- `impl<T> Send for Task<T>`
- `impl<T> !Sync for Task<T>`
- `impl<T> Unpin for Task<T>`
- `impl<T> UnsafeUnpin for Task<T>`
- `impl<T> !UnwindSafe for Task<T>`
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
