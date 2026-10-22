---
title: "Outcome"
kind: enum
path: "iced_core::widget::operation::Outcome"
crate: "iced_core"
module: "iced_core::widget::operation"
file: "iced_core/widget/operation/enum.Outcome.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/enum.Outcome.html"
source_file: "src/iced_core/widget/operation.rs"
source_lines: "108-117"
---

# `iced_core::widget::operation::Outcome`

```rust
pub enum Outcome<T> {
    None,
    Some(T),
    Chain(Box<dyn Operation<T>>),
}
```

The result of an [`Operation`](trait.Operation.md).

**Source:** [operation.rs](../../../src/iced_core/widget/operation.rs.md) (lines 108-117)

## Variants

### `None`

The [`Operation`](trait.Operation.md) produced no result.

### `Some(T)`

The [`Operation`](trait.Operation.md) produced some result.

### `Chain(Box<dyn Operation<T>>)`

The [`Operation`](trait.Operation.md) needs to be followed by another [`Operation`](trait.Operation.md).

## Trait Implementations

### `impl<T> Debug for Outcome<T> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl<T> Freeze for Outcome<T> where T: Freeze,`
- `impl<T> !RefUnwindSafe for Outcome<T>`
- `impl<T> Send for Outcome<T> where T: Send,`
- `impl<T> !Sync for Outcome<T>`
- `impl<T> Unpin for Outcome<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Outcome<T> where T: UnsafeUnpin,`
- `impl<T> !UnwindSafe for Outcome<T>`
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
