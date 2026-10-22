---
title: "Executor"
kind: struct
path: "iced_futures::backend::null::Executor"
crate: "iced_futures"
module: "iced_futures::backend::null"
file: "iced_futures/backend/null/struct.Executor.md"
docs_url: "https://docs.iced.rs/iced_futures/backend/null/struct.Executor.html"
source_file: "src/iced_futures/backend/null.rs"
source_lines: "6"
---

# `iced_futures::backend::null::Executor`

```rust
pub struct Executor;
```

An executor that drops all the futures, instead of spawning them.

**Source:** [null.rs](../../../src/iced_futures/backend/null.rs.md) (lines 6)

## Trait Implementations

### `impl Debug for Executor`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Executor for Executor`

#### `new`

```rust
fn new() -> Result<Self, Error>
```

Creates a new [`Executor`](../../executor/trait.Executor.md).

#### `spawn`

```rust
fn spawn(&self, _future: impl Future<Output = ()> + MaybeSend + 'static)
```

Spawns a future in the [`Executor`](../../executor/trait.Executor.md).

#### `block_on`

```rust
fn block_on<T>(&self, _future: impl Future<Output = T>) -> T
```

*Available on non-WebAssembly only.*

Runs a future to completion in the current thread within the [`Executor`](../../executor/trait.Executor.md).

#### `enter`

```rust
fn enter<R>(&self, f: impl FnOnce() -> R) -> R
```

Runs the given closure inside the [`Executor`](../../executor/trait.Executor.md). [Read more](../../executor/trait.Executor.md)

## Auto Trait Implementations

- `impl Freeze for Executor`
- `impl RefUnwindSafe for Executor`
- `impl Send for Executor`
- `impl Sync for Executor`
- `impl Unpin for Executor`
- `impl UnsafeUnpin for Executor`
- `impl UnwindSafe for Executor`
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
