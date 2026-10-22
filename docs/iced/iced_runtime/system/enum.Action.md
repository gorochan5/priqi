---
title: "Action"
kind: enum
path: "iced_runtime::system::Action"
crate: "iced_runtime"
module: "iced_runtime::system"
file: "iced_runtime/system/enum.Action.md"
docs_url: "https://docs.iced.rs/iced_runtime/system/enum.Action.html"
source_file: "src/iced_runtime/system.rs"
source_lines: "9-18"
---

# `iced_runtime::system::Action`

```rust
pub enum Action {
    GetInformation(Sender<Information>),
    GetTheme(Sender<Mode>),
    NotifyTheme(Mode),
}
```

An operation to be performed on the system.

**Source:** [system.rs](../../src/iced_runtime/system.rs.md) (lines 9-18)

## Variants

### `GetInformation(Sender<Information>)`

Send available system information.

### `GetTheme(Sender<Mode>)`

Send the current system theme mode.

### `NotifyTheme(Mode)`

Notify to the runtime that the system theme has changed.

## Trait Implementations

### `impl Debug for Action`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl Freeze for Action`
- `impl !RefUnwindSafe for Action`
- `impl Send for Action`
- `impl Sync for Action`
- `impl Unpin for Action`
- `impl UnsafeUnpin for Action`
- `impl !UnwindSafe for Action`
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
