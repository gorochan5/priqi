---
title: "Transformation"
kind: struct
path: "iced_core::Transformation"
crate: "iced_core"
module: "iced_core"
file: "iced_core/struct.Transformation.md"
docs_url: "https://docs.iced.rs/iced_core/struct.Transformation.html"
source_file: "src/iced_core/transformation.rs"
source_lines: "8"
---

# `iced_core::Transformation`

```rust
pub struct Transformation(/* private fields */);
```

A 2D transformation matrix.

**Source:** [transformation.rs](../src/iced_core/transformation.rs.md) (lines 8)

## Methods

### `impl Transformation`

#### `IDENTITY`

```rust
pub const IDENTITY: Self
```

A [`Transformation`](struct.Transformation.md) that preserves whatever is transformed.

#### `orthographic`

```rust
pub fn orthographic(width: u32, height: u32) -> Self
```

Creates an orthographic projection.

#### `translate`

```rust
pub fn translate(x: f32, y: f32) -> Self
```

Creates a translate transformation.

#### `scale`

```rust
pub fn scale(scaling: f32) -> Self
```

Creates a uniform scaling transformation.

#### `inverse`

```rust
pub fn inverse(self) -> Self
```

Returns the inverse of the [`Transformation`](struct.Transformation.md).

#### `scale_factor`

```rust
pub fn scale_factor(&self) -> f32
```

Returns the scale factor of the [`Transformation`](struct.Transformation.md).

#### `translation`

```rust
pub fn translation(&self) -> Vector
```

Returns the translation of the [`Transformation`](struct.Transformation.md).

## Trait Implementations

### `impl AsRef<[f32; 16]> for Transformation`

#### `as_ref`

```rust
fn as_ref(&self) -> &[f32; 16]
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl Clone for Transformation`

#### `clone`

```rust
fn clone(&self) -> Transformation
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Transformation`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Transformation`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl From<Transformation> for [f32; 16]`

#### `from`

```rust
fn from(t: Transformation) -> [f32; 16]
```

Converts to this type from the input type.

### `impl From<Transformation> for Mat4`

#### `from`

```rust
fn from(transformation: Transformation) -> Self
```

Converts to this type from the input type.

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

### `impl Mul<Transformation> for Point`

#### `Output`

```rust
type Output = Point
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, transformation: Transformation) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl Mul<Transformation> for Rectangle`

#### `Output`

```rust
type Output = Rectangle
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, transformation: Transformation) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl Mul<Transformation> for Size`

#### `Output`

```rust
type Output = Size
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, transformation: Transformation) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl Mul<Transformation> for Vector`

#### `Output`

```rust
type Output = Vector
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, transformation: Transformation) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl Mul for Transformation`

#### `Output`

```rust
type Output = Transformation
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, rhs: Self) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl PartialEq for Transformation`

#### `eq`

```rust
fn eq(&self, other: &Transformation) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Transformation`

### `impl StructuralPartialEq for Transformation`

## Auto Trait Implementations

- `impl Freeze for Transformation`
- `impl RefUnwindSafe for Transformation`
- `impl Send for Transformation`
- `impl Sync for Transformation`
- `impl Unpin for Transformation`
- `impl UnsafeUnpin for Transformation`
- `impl UnwindSafe for Transformation`
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
