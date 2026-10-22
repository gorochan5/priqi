---
title: "Click"
kind: struct
path: "iced_core::mouse::click::Click"
crate: "iced_core"
module: "iced_core::mouse::click"
file: "iced_core/mouse/click/struct.Click.md"
docs_url: "https://docs.iced.rs/iced_core/mouse/click/struct.Click.html"
source_file: "src/iced_core/mouse/click.rs"
source_lines: "10-15"
---

# `iced_core::mouse::click::Click`

```rust
pub struct Click { /* private fields */ }
```

A mouse click.

**Source:** [click.rs](../../../src/iced_core/mouse/click.rs.md) (lines 10-15)

## Methods

### `impl Click`

#### `new`

```rust
pub fn new(position: Point, button: Button, previous: Option<Click>) -> Click
```

Creates a new [`Click`](struct.Click.md) with the given position and previous last [`Click`](struct.Click.md).

#### `kind`

```rust
pub fn kind(&self) -> Kind
```

Returns the [`Kind`](enum.Kind.md) of [`Click`](struct.Click.md).

#### `position`

```rust
pub fn position(&self) -> Point
```

Returns the position of the [`Click`](struct.Click.md).

## Trait Implementations

### `impl Clone for Click`

#### `clone`

```rust
fn clone(&self) -> Click
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Click`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Mul<Transformation> for Click`

#### `Output`

```rust
type Output = Click
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, transformation: Transformation) -> Click
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl Copy for Click`

## Auto Trait Implementations

- `impl Freeze for Click`
- `impl RefUnwindSafe for Click`
- `impl Send for Click`
- `impl Sync for Click`
- `impl Unpin for Click`
- `impl UnsafeUnpin for Click`
- `impl UnwindSafe for Click`
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
