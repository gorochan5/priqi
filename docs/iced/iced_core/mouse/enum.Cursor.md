---
title: "Cursor"
kind: enum
path: "iced_core::mouse::Cursor"
crate: "iced_core"
module: "iced_core::mouse"
file: "iced_core/mouse/enum.Cursor.md"
docs_url: "https://docs.iced.rs/iced_core/mouse/enum.Cursor.html"
source_file: "src/iced_core/mouse/cursor.rs"
source_lines: "5-23"
---

# `iced_core::mouse::Cursor`

```rust
pub enum Cursor {
    Available(Point),
    Levitating(Point),
    Obstructed(Point),
    Unavailable,
}
```

The mouse cursor state.

**Source:** [cursor.rs](../../src/iced_core/mouse/cursor.rs.md) (lines 5-23)

## Variants

### `Available(Point)`

The cursor has a defined position.

### `Levitating(Point)`

The cursor has a defined position, but it’s levitating over a layer above.

You can [`land`](enum.Cursor.md) and [`observe`](enum.Cursor.md) it safely, and make it [`Available`](enum.Cursor.md) again.

### `Obstructed(Point)`

The cursor has a defined position, but it’s obstructed by a layer above.

It cannot [`land`](enum.Cursor.md), but you can [`observe`](enum.Cursor.md) it from below.

### `Unavailable`

The cursor is currently unavailable (i.e. out of bounds or busy).

## Methods

### `impl Cursor`

#### `position`

```rust
pub fn position(self) -> Option<Point>
```

Returns the absolute position of the [`Cursor`](enum.Cursor.md), if available.

#### `position_over`

```rust
pub fn position_over(self, bounds: Rectangle) -> Option<Point>
```

Returns the absolute position of the [`Cursor`](enum.Cursor.md), if available and inside the given bounds.

If the [`Cursor`](enum.Cursor.md) is not over the provided bounds, this method will return `None`.

#### `position_in`

```rust
pub fn position_in(self, bounds: Rectangle) -> Option<Point>
```

Returns the relative position of the [`Cursor`](enum.Cursor.md) inside the given bounds, if available.

If the [`Cursor`](enum.Cursor.md) is not over the provided bounds, this method will return `None`.

#### `position_from`

```rust
pub fn position_from(self, origin: Point) -> Option<Point>
```

Returns the relative position of the [`Cursor`](enum.Cursor.md) from the given origin, if available.

#### `is_over`

```rust
pub fn is_over(self, bounds: Rectangle) -> bool
```

Returns true if the [`Cursor`](enum.Cursor.md) is over the given `bounds`.

#### `is_levitating`

```rust
pub fn is_levitating(self) -> bool
```

Returns true if the [`Cursor`](enum.Cursor.md) is levitating over a layer above.

#### `levitate`

```rust
pub fn levitate(self) -> Self
```

Makes the [`Cursor`](enum.Cursor.md) levitate over a layer above.

#### `obstruct`

```rust
pub fn obstruct(self) -> Self
```

Makes the [`Cursor`](enum.Cursor.md) obstructed over a layer above.

#### `land`

```rust
pub fn land(self) -> Self
```

If the [`Cursor`](enum.Cursor.md) is levitating, it makes it available again.

#### `observe`

```rust
pub fn observe(self) -> Self
```

If the [`Cursor`](enum.Cursor.md) is either levitating or obstructed, it makes it available again.

## Trait Implementations

### `impl Add<Vector> for Cursor`

#### `Output`

```rust
type Output = Cursor
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, translation: Vector) -> Self::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl Clone for Cursor`

#### `clone`

```rust
fn clone(&self) -> Cursor
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Cursor`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Cursor`

#### `default`

```rust
fn default() -> Cursor
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Mul<Transformation> for Cursor`

#### `Output`

```rust
type Output = Cursor
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, transformation: Transformation) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl PartialEq for Cursor`

#### `eq`

```rust
fn eq(&self, other: &Cursor) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Sub<Vector> for Cursor`

#### `Output`

```rust
type Output = Cursor
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, translation: Vector) -> Self::Output
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl Copy for Cursor`

### `impl StructuralPartialEq for Cursor`

## Auto Trait Implementations

- `impl Freeze for Cursor`
- `impl RefUnwindSafe for Cursor`
- `impl Send for Cursor`
- `impl Sync for Cursor`
- `impl Unpin for Cursor`
- `impl UnsafeUnpin for Cursor`
- `impl UnwindSafe for Cursor`
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
