---
title: "Handle"
kind: struct
path: "iced_runtime::task::Handle"
crate: "iced_runtime"
module: "iced_runtime::task"
file: "iced_runtime/task/struct.Handle.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/struct.Handle.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "284-286"
---

# `iced_runtime::task::Handle`

```rust
pub struct Handle { /* private fields */ }
```

A handle to a [`Task`](struct.Task.md) that can be used for aborting it.

**Source:** [task.rs](../../src/iced_runtime/task.rs.md) (lines 284-286)

## Methods

### `impl Handle`

#### `abort`

```rust
pub fn abort(&self)
```

Aborts the [`Task`](struct.Task.md) of this [`Handle`](struct.Handle.md).

#### `abort_on_drop`

```rust
pub fn abort_on_drop(self) -> Self
```

Returns a new [`Handle`](struct.Handle.md) that will call [`Handle::abort`](struct.Handle.md) whenever all of its instances are dropped.

If a [`Handle`](struct.Handle.md) is cloned, [`Handle::abort`](struct.Handle.md) will only be called once all of the clones are dropped.

This can be really useful if you do not want to worry about calling [`Handle::abort`](struct.Handle.md) yourself.

#### `is_aborted`

```rust
pub fn is_aborted(&self) -> bool
```

Returns `true` if the [`Task`](struct.Task.md) of this [`Handle`](struct.Handle.md) has been aborted.

## Trait Implementations

### `impl Clone for Handle`

#### `clone`

```rust
fn clone(&self) -> Handle
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Handle`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Drop for Handle`

#### `drop`

```rust
fn drop(&mut self)
```

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations

- `impl Freeze for Handle`
- `impl !RefUnwindSafe for Handle`
- `impl Send for Handle`
- `impl Sync for Handle`
- `impl Unpin for Handle`
- `impl UnsafeUnpin for Handle`
- `impl !UnwindSafe for Handle`
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

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`
