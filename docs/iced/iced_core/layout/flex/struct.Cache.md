---
title: "Cache"
kind: struct
path: "iced_core::layout::flex::Cache"
crate: "iced_core"
module: "iced_core::layout::flex"
file: "iced_core/layout/flex/struct.Cache.md"
docs_url: "https://docs.iced.rs/iced_core/layout/flex/struct.Cache.html"
source_file: "src/iced_core/layout/flex.rs"
source_lines: "66-68"
---

# `iced_core::layout::flex::Cache`

```rust
pub struct Cache { /* private fields */ }
```

Per-container retained state for [`resolve`](fn.resolve.md).

[`resolve`](fn.resolve.md) keeps per-child bookkeeping across its multiple passes. To avoid allocating that bookkeeping on every `layout` call, each flex container stores a [`Cache`](struct.Cache.md) in its [`widget::Tree`](../../widget/tree/struct.Tree.md) state, and [`resolve`](fn.resolve.md) reuses the buffer stored there between layout calls.

The type is opaque: its contents are managed entirely by [`resolve`](fn.resolve.md).

**Source:** [flex.rs](../../../src/iced_core/layout/flex.rs.md) (lines 66-68)

## Trait Implementations

### `impl Debug for Cache`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Cache`

#### `default`

```rust
fn default() -> Cache
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations

- `impl Freeze for Cache`
- `impl RefUnwindSafe for Cache`
- `impl Send for Cache`
- `impl Sync for Cache`
- `impl Unpin for Cache`
- `impl UnsafeUnpin for Cache`
- `impl UnwindSafe for Cache`
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
