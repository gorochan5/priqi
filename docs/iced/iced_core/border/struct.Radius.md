---
title: "Radius"
kind: struct
path: "iced_core::border::Radius"
crate: "iced_core"
module: "iced_core::border"
file: "iced_core/border/struct.Radius.md"
docs_url: "https://docs.iced.rs/iced_core/border/struct.Radius.html"
source_file: "src/iced_core/border.rs"
source_lines: "81-90"
---

# `iced_core::border::Radius`

```rust
pub struct Radius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}
```

The border radii for the corners of a graphics primitive in the order: top-left, top-right, bottom-right, bottom-left.

**Source:** [border.rs](../../src/iced_core/border.rs.md) (lines 81-90)

## Fields

### `top_left: f32`

Top left radius

### `top_right: f32`

Top right radius

### `bottom_right: f32`

Bottom right radius

### `bottom_left: f32`

Bottom left radius

## Methods

### `impl Radius`

#### `new`

```rust
pub fn new(value: impl Into<Pixels>) -> Self
```

Creates a new [`Radius`](struct.Radius.md) with the same value for each corner.

#### `top_left`

```rust
pub fn top_left(self, value: impl Into<Pixels>) -> Self
```

Sets the top left value of the [`Radius`](struct.Radius.md).

#### `top_right`

```rust
pub fn top_right(self, value: impl Into<Pixels>) -> Self
```

Sets the top right value of the [`Radius`](struct.Radius.md).

#### `bottom_right`

```rust
pub fn bottom_right(self, value: impl Into<Pixels>) -> Self
```

Sets the bottom right value of the [`Radius`](struct.Radius.md).

#### `bottom_left`

```rust
pub fn bottom_left(self, value: impl Into<Pixels>) -> Self
```

Sets the bottom left value of the [`Radius`](struct.Radius.md).

#### `top`

```rust
pub fn top(self, value: impl Into<Pixels>) -> Self
```

Sets the top left and top right values of the [`Radius`](struct.Radius.md).

#### `bottom`

```rust
pub fn bottom(self, value: impl Into<Pixels>) -> Self
```

Sets the bottom left and bottom right values of the [`Radius`](struct.Radius.md).

#### `left`

```rust
pub fn left(self, value: impl Into<Pixels>) -> Self
```

Sets the top left and bottom left values of the [`Radius`](struct.Radius.md).

#### `right`

```rust
pub fn right(self, value: impl Into<Pixels>) -> Self
```

Sets the top right and bottom right values of the [`Radius`](struct.Radius.md).

## Trait Implementations

### `impl Clone for Radius`

#### `clone`

```rust
fn clone(&self) -> Radius
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Radius`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Radius`

#### `default`

```rust
fn default() -> Radius
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl From<Radius> for [f32; 4]`

#### `from`

```rust
fn from(radi: Radius) -> Self
```

Converts to this type from the input type.

### `impl From<f32> for Radius`

#### `from`

```rust
fn from(radius: f32) -> Self
```

Converts to this type from the input type.

### `impl From<i32> for Radius`

#### `from`

```rust
fn from(w: i32) -> Self
```

Converts to this type from the input type.

### `impl From<u32> for Radius`

#### `from`

```rust
fn from(w: u32) -> Self
```

Converts to this type from the input type.

### `impl From<u8> for Radius`

#### `from`

```rust
fn from(w: u8) -> Self
```

Converts to this type from the input type.

### `impl Mul<f32> for Radius`

#### `Output`

```rust
type Output = Radius
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: f32) -> Self::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl PartialEq for Radius`

#### `eq`

```rust
fn eq(&self, other: &Radius) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Radius`

### `impl StructuralPartialEq for Radius`

## Auto Trait Implementations

- `impl Freeze for Radius`
- `impl RefUnwindSafe for Radius`
- `impl Send for Radius`
- `impl Sync for Radius`
- `impl Unpin for Radius`
- `impl UnsafeUnpin for Radius`
- `impl UnwindSafe for Radius`
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
