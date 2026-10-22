---
title: "Vector"
kind: struct
path: "iced_core::Vector"
crate: "iced_core"
module: "iced_core"
file: "iced_core/struct.Vector.md"
docs_url: "https://docs.iced.rs/iced_core/struct.Vector.html"
source_file: "src/iced_core/vector.rs"
source_lines: "3-9"
---

# `iced_core::Vector`

```rust
pub struct Vector<T = f32> {
    pub x: T,
    pub y: T,
}
```

A 2D vector.

**Source:** [vector.rs](../src/iced_core/vector.rs.md) (lines 3-9)

## Fields

### `x: T`

The X component of the [`Vector`](struct.Vector.md)

### `y: T`

The Y component of the [`Vector`](struct.Vector.md)

## Methods

### `impl<T> Vector<T>`

#### `new`

```rust
pub const fn new(x: T, y: T) -> Self
```

Creates a new [`Vector`](struct.Vector.md) with the given components.

### `impl Vector`

#### `ZERO`

```rust
pub const ZERO: Self
```

The zero [`Vector`](struct.Vector.md).

#### `round`

```rust
pub fn round(self) -> Self
```

Rounds the [`Vector`](struct.Vector.md).

#### `hint`

```rust
pub fn hint(self, scale_factor: f32) -> Self
```

Computes the logical [`Vector`](struct.Vector.md) that ends up rounded after the given `scale_factor` is applied to it.

## Trait Implementations

### `impl<T> Add<Vector<T>> for Point<T> where T: Add<Output = T>,`

#### `Output`

```rust
type Output = Point<T>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, vector: Vector<T>) -> Self
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T> Add<Vector<T>> for Rectangle<T> where T: Add<Output = T>,`

#### `Output`

```rust
type Output = Rectangle<T>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, translation: Vector<T>) -> Self
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl Add<Vector> for AbsoluteOffset`

#### `Output`

```rust
type Output = AbsoluteOffset
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, translation: Vector) -> Self::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

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

### `impl<T> Add for Vector<T> where T: Add<Output = T>,`

#### `Output`

```rust
type Output = Vector<T>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, b: Self) -> Self
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T> AddAssign<Vector<T>> for Point<T> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, vector: Vector<T>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T> AddAssign<Vector<T>> for Rectangle<T> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, translation: Vector<T>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T> AddAssign for Vector<T> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, b: Self)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T: Clone> Clone for Vector<T>`

#### `clone`

```rust
fn clone(&self) -> Vector<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T: Debug> Debug for Vector<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T: Default> Default for Vector<T>`

#### `default`

```rust
fn default() -> Vector<T>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T> Div<T> for Vector<T> where T: Div<Output = T> + Copy,`

#### `Output`

```rust
type Output = Vector<T>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, scale: T) -> Self
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T> DivAssign<T> for Vector<T> where T: DivAssign + Copy,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: T)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T> From<[T; 2]> for Vector<T>`

#### `from`

```rust
fn from([x, y]: [T; 2]) -> Self
```

Converts to this type from the input type.

### `impl<T> From<Size<T>> for Vector<T>`

#### `from`

```rust
fn from(size: Size<T>) -> Self
```

Converts to this type from the input type.

### `impl<T> From<Vector<T>> for [T; 2] where T: Copy,`

#### `from`

```rust
fn from(other: Vector<T>) -> Self
```

Converts to this type from the input type.

### `impl<T> From<Vector<T>> for Size<T>`

#### `from`

```rust
fn from(vector: Vector<T>) -> Self
```

Converts to this type from the input type.

### `impl<T> Mul<T> for Vector<T> where T: Mul<Output = T> + Copy,`

#### `Output`

```rust
type Output = Vector<T>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: T) -> Self
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

### `impl<T> Mul<Vector<T>> for Size<T> where T: Mul<Output = T> + Copy,`

#### `Output`

```rust
type Output = Size<T>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: Vector<T>) -> Self::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T> MulAssign<T> for Vector<T> where T: MulAssign + Copy,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: T)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T> Neg for Vector<T> where T: Neg<Output = T>,`

#### `Output`

```rust
type Output = Vector<T>
```

The resulting type after applying the `-` operator.

#### `neg`

```rust
fn neg(self) -> Self::Output
```

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

### `impl<T: PartialEq> PartialEq for Vector<T>`

#### `eq`

```rust
fn eq(&self, other: &Vector<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T> Sub<Vector<T>> for Point<T> where T: Sub<Output = T>,`

#### `Output`

```rust
type Output = Point<T>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, vector: Vector<T>) -> Self
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T> Sub<Vector<T>> for Rectangle<T> where T: Sub<Output = T>,`

#### `Output`

```rust
type Output = Rectangle<T>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, translation: Vector<T>) -> Self
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

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

### `impl<T> Sub for Vector<T> where T: Sub<Output = T>,`

#### `Output`

```rust
type Output = Vector<T>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, b: Self) -> Self
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T> SubAssign<Vector<T>> for Point<T> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, vector: Vector<T>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<T> SubAssign<Vector<T>> for Rectangle<T> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, translation: Vector<T>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<T> SubAssign for Vector<T> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, b: Self)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<T: Copy> Copy for Vector<T>`

### `impl<T: Eq> Eq for Vector<T>`

### `impl<T> StructuralPartialEq for Vector<T>`

## Auto Trait Implementations

- `impl<T> Freeze for Vector<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for Vector<T> where T: RefUnwindSafe,`
- `impl<T> Send for Vector<T> where T: Send,`
- `impl<T> Sync for Vector<T> where T: Sync,`
- `impl<T> Unpin for Vector<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Vector<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for Vector<T> where T: UnwindSafe,`
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
