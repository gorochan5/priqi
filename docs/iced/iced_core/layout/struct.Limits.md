---
title: "Limits"
kind: struct
path: "iced_core::layout::Limits"
crate: "iced_core"
module: "iced_core::layout"
file: "iced_core/layout/struct.Limits.md"
docs_url: "https://docs.iced.rs/iced_core/layout/struct.Limits.html"
source_file: "src/iced_core/layout/limits.rs"
source_lines: "7-17"
---

# `iced_core::layout::Limits`

```rust
pub struct Limits {
    pub min: Size,
    pub max: Size,
    pub compression: Size<bool>,
    pub infinite: Size<bool>,
}
```

A set of size constraints for layouting.

**Source:** [limits.rs](../../src/iced_core/layout/limits.rs.md) (lines 7-17)

## Fields

### `min: Size`

The minimum bounds.

### `max: Size`

The maximum bounds.

### `compression: Size<bool>`

Whether fluid lengths should be compressed to intrinsic dimensions.

### `infinite: Size<bool>`

Whether intrinsic dimensions can exceed maximum bounds. The maximum boundary becomes a hint in this case.

## Methods

### `impl Limits`

#### `new`

```rust
pub const fn new(min: Size, max: Size) -> Limits
```

Creates new [`Limits`](struct.Limits.md) with the given minimum and maximum [`Size`](../struct.Size.md).

#### `with_flags`

```rust
pub const fn with_flags( min: Size, max: Size, compression: Size<bool>, infinite: Size<bool>, ) -> Self
```

Creates new [`Limits`](struct.Limits.md) with the given minimum and maximum [`Size`](../struct.Size.md), whether fluid lengths should be compressed to intrinsic dimensions, and whether the upper bounds may grow indefinitely.

#### `bounds`

```rust
pub fn bounds(&self) -> Size
```

Returns the maximum [`Size`](../struct.Size.md) of the [`Limits`](struct.Limits.md).

On axes with [`infinite`](struct.Limits.md) bounds, the value is [`f32::INFINITY`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#associatedconstant.INFINITY), as the maximum boundary becomes a hint in this case.

#### `width`

```rust
pub fn width(self, width: impl Into<Length>) -> Limits
```

Applies a width constraint to the current [`Limits`](struct.Limits.md).

#### `height`

```rust
pub fn height(self, height: impl Into<Length>) -> Limits
```

Applies a height constraint to the current [`Limits`](struct.Limits.md).

#### `shrink`

```rust
pub fn shrink(&self, size: impl Into<Size>) -> Limits
```

Shrinks the current [`Limits`](struct.Limits.md) by the given [`Size`](../struct.Size.md).

#### `loose`

```rust
pub fn loose(&self) -> Limits
```

Removes the minimum [`Size`](../struct.Size.md) constraint for the current [`Limits`](struct.Limits.md).

#### `resolve`

```rust
pub fn resolve( &self, width: impl Into<Length>, height: impl Into<Length>, intrinsic_size: Size, ) -> Size
```

Computes the resulting [`Size`](../struct.Size.md) that fits the [`Limits`](struct.Limits.md) given some width and height requirements and the intrinsic size of some content.

#### `resolve_width`

```rust
pub fn resolve_width( &self, width: impl Into<Length>, intrinsic_width: f32, ) -> f32
```

[Resolves](struct.Limits.md) only the width of the [`Limits`](struct.Limits.md).

#### `resolve_height`

```rust
pub fn resolve_height( &self, height: impl Into<Length>, intrinsic_height: f32, ) -> f32
```

[Resolves](struct.Limits.md) only the height of the [`Limits`](struct.Limits.md).

## Trait Implementations

### `impl Clone for Limits`

#### `clone`

```rust
fn clone(&self) -> Limits
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Limits`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl PartialEq for Limits`

#### `eq`

```rust
fn eq(&self, other: &Limits) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Limits`

### `impl StructuralPartialEq for Limits`

## Auto Trait Implementations

- `impl Freeze for Limits`
- `impl RefUnwindSafe for Limits`
- `impl Send for Limits`
- `impl Sync for Limits`
- `impl Unpin for Limits`
- `impl UnsafeUnpin for Limits`
- `impl UnwindSafe for Limits`
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
