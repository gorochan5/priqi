---
title: "Pixels"
kind: struct
path: "iced_core::Pixels"
crate: "iced_core"
module: "iced_core"
file: "iced_core/struct.Pixels.md"
docs_url: "https://docs.iced.rs/iced_core/struct.Pixels.html"
source_file: "src/iced_core/pixels.rs"
source_lines: "10"
---

# `iced_core::Pixels`

```rust
pub struct Pixels(pub f32);
```

An amount of logical pixels.

Normally used to represent an amount of space, or the size of something.

This type is normally asked as an argument in a generic way (e.g. `impl Into<Pixels>`) and, since `Pixels` implements `From` both for `f32` and `u16`, you should be able to provide both integers and float literals as needed.

**Source:** [pixels.rs](../src/iced_core/pixels.rs.md) (lines 10)

## Fields

### `0: f32`

## Methods

### `impl Pixels`

#### `ZERO`

```rust
pub const ZERO: Self
```

Zero pixels.

## Trait Implementations

### `impl Add<f32> for Pixels`

#### `Output`

```rust
type Output = Pixels
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, rhs: f32) -> Self
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl Add for Pixels`

#### `Output`

```rust
type Output = Pixels
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, rhs: Self) -> Self
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl Clone for Pixels`

#### `clone`

```rust
fn clone(&self) -> Pixels
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Pixels`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Pixels`

#### `default`

```rust
fn default() -> Pixels
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Div<f32> for Pixels`

#### `Output`

```rust
type Output = Pixels
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, rhs: f32) -> Self
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl Div<u32> for Pixels`

#### `Output`

```rust
type Output = Pixels
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, rhs: u32) -> Self
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl Div for Pixels`

#### `Output`

```rust
type Output = Pixels
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, rhs: Self) -> Self
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl From<Pixels> for Length`

#### `from`

```rust
fn from(amount: Pixels) -> Self
```

Converts to this type from the input type.

### `impl From<Pixels> for LineHeight`

#### `from`

```rust
fn from(pixels: Pixels) -> Self
```

Converts to this type from the input type.

### `impl From<Pixels> for Padding`

#### `from`

```rust
fn from(pixels: Pixels) -> Self
```

Converts to this type from the input type.

### `impl From<Pixels> for f32`

#### `from`

```rust
fn from(pixels: Pixels) -> Self
```

Converts to this type from the input type.

### `impl From<f32> for Pixels`

#### `from`

```rust
fn from(amount: f32) -> Self
```

Converts to this type from the input type.

### `impl From<u32> for Pixels`

#### `from`

```rust
fn from(amount: u32) -> Self
```

Converts to this type from the input type.

### `impl Mul<f32> for Pixels`

#### `Output`

```rust
type Output = Pixels
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, rhs: f32) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl Mul for Pixels`

#### `Output`

```rust
type Output = Pixels
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, rhs: Self) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl PartialEq for Pixels`

#### `eq`

```rust
fn eq(&self, other: &Pixels) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd for Pixels`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Pixels) -> Option<Ordering>
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

### `impl Copy for Pixels`

### `impl StructuralPartialEq for Pixels`

## Auto Trait Implementations

- `impl Freeze for Pixels`
- `impl RefUnwindSafe for Pixels`
- `impl Send for Pixels`
- `impl Sync for Pixels`
- `impl Unpin for Pixels`
- `impl UnsafeUnpin for Pixels`
- `impl UnwindSafe for Pixels`
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
