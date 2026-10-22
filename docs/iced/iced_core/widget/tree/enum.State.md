---
title: "State"
kind: enum
path: "iced_core::widget::tree::State"
crate: "iced_core"
module: "iced_core::widget::tree"
file: "iced_core/widget/tree/enum.State.md"
docs_url: "https://docs.iced.rs/iced_core/widget/tree/enum.State.html"
source_file: "src/iced_core/widget/tree.rs"
source_lines: "231-237"
---

# `iced_core::widget::tree::State`

```rust
pub enum State {
    None,
    Some(Box<dyn Any>),
}
```

The internal [`State`](enum.State.md) of a widget.

**Source:** [tree.rs](../../../src/iced_core/widget/tree.rs.md) (lines 231-237)

## Variants

### `None`

No meaningful internal state.

### `Some(Box<dyn Any>)`

Some meaningful internal state.

## Methods

### `impl State`

#### `new`

```rust
pub fn new<T>(state: T) -> Self where T: 'static,
```

Creates a new [`State`](enum.State.md).

#### `downcast_ref`

```rust
pub fn downcast_ref<T>(&self) -> &T where T: 'static,
```

Downcasts the [`State`](enum.State.md) to `T` and returns a reference to it.

###### Panics

This method will panic if the downcast fails or the [`State`](enum.State.md) is [`State::None`](enum.State.md).

#### `downcast_mut`

```rust
pub fn downcast_mut<T>(&mut self) -> &mut T where T: 'static,
```

Downcasts the [`State`](enum.State.md) to `T` and returns a mutable reference to it.

###### Panics

This method will panic if the downcast fails or the [`State`](enum.State.md) is [`State::None`](enum.State.md).

## Trait Implementations

### `impl Debug for State`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl Freeze for State`
- `impl !RefUnwindSafe for State`
- `impl !Send for State`
- `impl !Sync for State`
- `impl Unpin for State`
- `impl UnsafeUnpin for State`
- `impl !UnwindSafe for State`
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
