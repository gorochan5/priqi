---
title: "Border"
kind: struct
path: "iced_core::border::Border"
crate: "iced_core"
module: "iced_core::border"
file: "iced_core/border/struct.Border.md"
docs_url: "https://docs.iced.rs/iced_core/border/struct.Border.html"
source_file: "src/iced_core/border.rs"
source_lines: "6-15"
---

# `iced_core::border::Border`

```rust
pub struct Border {
    pub color: Color,
    pub width: f32,
    pub radius: Radius,
}
```

A border.

**Source:** [border.rs](../../src/iced_core/border.rs.md) (lines 6-15)

## Fields

### `color: Color`

The color of the border.

### `width: f32`

The width of the border.

### `radius: Radius`

The [`Radius`](struct.Radius.md) of the border.

## Methods

### `impl Border`

#### `color`

```rust
pub fn color(self, color: impl Into<Color>) -> Self
```

Sets the [`Color`](../color/struct.Color.md) of the [`Border`](struct.Border.md).

#### `rounded`

```rust
pub fn rounded(self, radius: impl Into<Radius>) -> Self
```

Sets the [`Radius`](struct.Radius.md) of the [`Border`](struct.Border.md).

#### `width`

```rust
pub fn width(self, width: impl Into<Pixels>) -> Self
```

Sets the width of the [`Border`](struct.Border.md).

## Trait Implementations

### `impl Clone for Border`

#### `clone`

```rust
fn clone(&self) -> Border
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Border`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Border`

#### `default`

```rust
fn default() -> Border
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl PartialEq for Border`

#### `eq`

```rust
fn eq(&self, other: &Border) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Border`

### `impl StructuralPartialEq for Border`

## Auto Trait Implementations

- `impl Freeze for Border`
- `impl RefUnwindSafe for Border`
- `impl Send for Border`
- `impl Sync for Border`
- `impl Unpin for Border`
- `impl UnsafeUnpin for Border`
- `impl UnwindSafe for Border`
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
