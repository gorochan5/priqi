---
title: "Degrees"
kind: struct
path: "iced_core::Degrees"
crate: "iced_core"
module: "iced_core"
file: "iced_core/struct.Degrees.md"
docs_url: "https://docs.iced.rs/iced_core/struct.Degrees.html"
source_file: "src/iced_core/angle.rs"
source_lines: "7"
---

# `iced_core::Degrees`

```rust
pub struct Degrees(pub f32);
```

Degrees

**Source:** [angle.rs](../src/iced_core/angle.rs.md) (lines 7)

## Fields

### `0: f32`

## Methods

### `impl Degrees`

#### `RANGE`

```rust
pub const RANGE: RangeInclusive<Self>
```

The range of degrees of a circle.

## Trait Implementations

### `impl Add<Degrees> for Radians`

#### `Output`

```rust
type Output = Radians
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, rhs: Degrees) -> Self::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl AsPrimitive<f32> for Degrees`

#### `as_`

```rust
fn as_(self) -> f32
```

Convert a value to another, using the `as` operator.

### `impl AsPrimitive<f64> for Degrees`

#### `as_`

```rust
fn as_(self) -> f64
```

Convert a value to another, using the `as` operator.

### `impl Clone for Degrees`

#### `clone`

```rust
fn clone(&self) -> Degrees
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Degrees`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<Degrees> for Radians`

#### `from`

```rust
fn from(degrees: Degrees) -> Self
```

Converts to this type from the input type.

### `impl From<Degrees> for f32`

#### `from`

```rust
fn from(degrees: Degrees) -> Self
```

Converts to this type from the input type.

### `impl From<Degrees> for f64`

#### `from`

```rust
fn from(degrees: Degrees) -> Self
```

Converts to this type from the input type.

### `impl From<f32> for Degrees`

#### `from`

```rust
fn from(degrees: f32) -> Self
```

Converts to this type from the input type.

### `impl From<u8> for Degrees`

#### `from`

```rust
fn from(degrees: u8) -> Self
```

Converts to this type from the input type.

### `impl FromPrimitive for Degrees`

#### `from_i64`

```rust
fn from_i64(n: i64) -> Option<Self>
```

Converts an `i64` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_u64`

```rust
fn from_u64(n: u64) -> Option<Self>
```

Converts an `u64` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_f64`

```rust
fn from_f64(n: f64) -> Option<Self>
```

Converts a `f64` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned. [Read more](https://docs.rs/num-traits/0.2/num_traits/cast/trait.FromPrimitive.html#method.from_f64)

#### `from_isize`

```rust
fn from_isize(n: isize) -> Option<Self>
```

Converts an `isize` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_i8`

```rust
fn from_i8(n: i8) -> Option<Self>
```

Converts an `i8` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_i16`

```rust
fn from_i16(n: i16) -> Option<Self>
```

Converts an `i16` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_i32`

```rust
fn from_i32(n: i32) -> Option<Self>
```

Converts an `i32` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_i128`

```rust
fn from_i128(n: i128) -> Option<Self>
```

Converts an `i128` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned. [Read more](https://docs.rs/num-traits/0.2/num_traits/cast/trait.FromPrimitive.html#method.from_i128)

#### `from_usize`

```rust
fn from_usize(n: usize) -> Option<Self>
```

Converts a `usize` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_u8`

```rust
fn from_u8(n: u8) -> Option<Self>
```

Converts an `u8` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_u16`

```rust
fn from_u16(n: u16) -> Option<Self>
```

Converts an `u16` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_u32`

```rust
fn from_u32(n: u32) -> Option<Self>
```

Converts an `u32` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

#### `from_u128`

```rust
fn from_u128(n: u128) -> Option<Self>
```

Converts an `u128` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned. [Read more](https://docs.rs/num-traits/0.2/num_traits/cast/trait.FromPrimitive.html#method.from_u128)

#### `from_f32`

```rust
fn from_f32(n: f32) -> Option<Self>
```

Converts a `f32` to return an optional value of this type. If the value cannot be represented by this type, then `None` is returned.

### `impl Mul<f32> for Degrees`

#### `Output`

```rust
type Output = Degrees
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, rhs: f32) -> Self::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl PartialEq<f32> for Degrees`

#### `eq`

```rust
fn eq(&self, other: &f32) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq for Degrees`

#### `eq`

```rust
fn eq(&self, other: &Degrees) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd<f32> for Degrees`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &f32) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, other: &Rhs) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, other: &Rhs) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `gt`

```rust
fn gt(&self, other: &Rhs) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

#### `ge`

```rust
fn ge(&self, other: &Rhs) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

### `impl PartialOrd for Degrees`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Degrees) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, other: &Rhs) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, other: &Rhs) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `gt`

```rust
fn gt(&self, other: &Rhs) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

#### `ge`

```rust
fn ge(&self, other: &Rhs) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

### `impl Copy for Degrees`

### `impl StructuralPartialEq for Degrees`

## Auto Trait Implementations

- `impl Freeze for Degrees`
- `impl RefUnwindSafe for Degrees`
- `impl Send for Degrees`
- `impl Sync for Degrees`
- `impl Unpin for Degrees`
- `impl UnsafeUnpin for Degrees`
- `impl UnwindSafe for Degrees`
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
