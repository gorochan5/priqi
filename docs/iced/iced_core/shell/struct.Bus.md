---
title: "Bus"
kind: struct
path: "iced_core::shell::Bus"
crate: "iced_core"
module: "iced_core::shell"
file: "iced_core/shell/struct.Bus.md"
docs_url: "https://docs.iced.rs/iced_core/shell/struct.Bus.html"
source_file: "src/iced_core/shell.rs"
source_lines: "296-298"
---

# `iced_core::shell::Bus`

```rust
pub struct Bus<T> { /* private fields */ }
```

A channel of messages published by a [`Shell`](struct.Shell.md).

**Source:** [shell.rs](../../src/iced_core/shell.rs.md) (lines 296-298)

## Methods

### `impl<T> Bus<T>`

#### `new`

```rust
pub fn new() -> Self
```

Creates an empty [`Bus`](struct.Bus.md).

#### `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

Returns `true` if the [`Bus`](struct.Bus.md) has no messages pending.

#### `len`

```rust
pub fn len(&self) -> usize
```

Returns the amount of messages pending in the [`Bus`](struct.Bus.md).

#### `push`

```rust
pub fn push(&mut self, message: T) -> Tracking
```

Pushes a new message to the [`Bus`](struct.Bus.md).

The returned [`Tracking`](struct.Tracking.md) can be used to determine if the message was processed.

#### `forward`

```rust
pub fn forward(&mut self, message: T, receipt: Receipt)
```

Forward a new message to the [`Bus`](struct.Bus.md) with the given [`Receipt`](struct.Receipt.md).

#### `drain`

```rust
pub fn drain(&mut self) -> impl Iterator<Item = (T, Receipt)>
```

Drains the [`Bus`](struct.Bus.md).

## Trait Implementations

### `impl<T: Debug> Debug for Bus<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T> Default for Bus<T>`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T> IntoIterator for Bus<T>`

#### `Item`

```rust
type Item = T
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = IntoIter<T>
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> Self::IntoIter
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

## Auto Trait Implementations

- `impl<T> Freeze for Bus<T>`
- `impl<T> RefUnwindSafe for Bus<T> where T: RefUnwindSafe,`
- `impl<T> !Send for Bus<T>`
- `impl<T> !Sync for Bus<T>`
- `impl<T> Unpin for Bus<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Bus<T>`
- `impl<T> UnwindSafe for Bus<T> where T: UnwindSafe,`
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
