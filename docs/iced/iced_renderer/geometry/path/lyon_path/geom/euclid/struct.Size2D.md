---
title: "Size2D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Size2D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Size2D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Size2D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Size2D`

```rust
pub struct Size2D<T, U> {
    pub width: T,
    pub height: T,
    /* private fields */
}
```

A 2d size tagged with a unit.

## Fields

### `width: T`

The extent of the element in the `U` units along the `x` axis (usually horizontal).

### `height: T`

The extent of the element in the `U` units along the `y` axis (usually vertical).

## Methods

### `impl<T, U> Size2D<T, U>`

#### `zero`

```rust
pub fn zero() -> Size2D<T, U> where T: Zero,
```

The same as [`Zero::zero`](num/trait.Zero.md) but available without importing trait.

#### `new`

```rust
pub const fn new(width: T, height: T) -> Size2D<T, U>
```

Constructor taking scalar values.

#### `from_lengths`

```rust
pub fn from_lengths(width: Length<T, U>, height: Length<T, U>) -> Size2D<T, U>
```

Constructor taking scalar strongly typed lengths.

#### `splat`

```rust
pub fn splat(v: T) -> Size2D<T, U> where T: Clone,
```

Constructor setting all components to the same value.

#### `from_untyped`

```rust
pub fn from_untyped(p: Size2D<T, UnknownUnit>) -> Size2D<T, U>
```

Tag a unitless value with units.

### `impl<T, U> Size2D<T, U> where T: Copy,`

#### `to_array`

```rust
pub fn to_array(self) -> [T; 2]
```

Return this size as an array of two elements (width, then height).

#### `to_tuple`

```rust
pub fn to_tuple(self) -> (T, T)
```

Return this size as a tuple of two elements (width, then height).

#### `to_vector`

```rust
pub fn to_vector(self) -> Vector2D<T, U>
```

Return this size as a vector with width and height.

#### `to_untyped`

```rust
pub fn to_untyped(self) -> Size2D<T, UnknownUnit>
```

Drop the units, preserving only the numeric value.

#### `cast_unit`

```rust
pub fn cast_unit<V>(self) -> Size2D<T, V>
```

Cast the unit

#### `round`

```rust
pub fn round(self) -> Size2D<T, U> where T: Round,
```

Rounds each component to the nearest integer value.

This behavior is preserved for negative values (unlike the basic cast).

```rust
enum Mm {}

assert_eq!(size2::<_, Mm>(-0.1, -0.8).round(), size2::<_, Mm>(0.0, -1.0))
```

#### `ceil`

```rust
pub fn ceil(self) -> Size2D<T, U> where T: Ceil,
```

Rounds each component to the smallest integer equal or greater than the original value.

This behavior is preserved for negative values (unlike the basic cast).

```rust
enum Mm {}

assert_eq!(size2::<_, Mm>(-0.1, -0.8).ceil(), size2::<_, Mm>(0.0, 0.0))
```

#### `floor`

```rust
pub fn floor(self) -> Size2D<T, U> where T: Floor,
```

Rounds each component to the biggest integer equal or lower than the original value.

This behavior is preserved for negative values (unlike the basic cast).

```rust
enum Mm {}

assert_eq!(size2::<_, Mm>(-0.1, -0.8).floor(), size2::<_, Mm>(-1.0, -1.0))
```

#### `area`

```rust
pub fn area(self) -> <T as Mul>::Output where T: Mul,
```

Returns result of multiplication of both components

#### `lerp`

```rust
pub fn lerp(self, other: Size2D<T, U>, t: T) -> Size2D<T, U> where T: One + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
```

Linearly interpolate each component between this size and another size.

###### Example

```rust
use euclid::size2;
use euclid::default::Size2D;

let from: Size2D<_> = size2(0.0, 10.0);
let to:  Size2D<_> = size2(8.0, -4.0);

assert_eq!(from.lerp(to, -1.0), size2(-8.0,  24.0));
assert_eq!(from.lerp(to,  0.0), size2( 0.0,  10.0));
assert_eq!(from.lerp(to,  0.5), size2( 4.0,   3.0));
assert_eq!(from.lerp(to,  1.0), size2( 8.0,  -4.0));
assert_eq!(from.lerp(to,  2.0), size2(16.0, -18.0));
```

### `impl<T, U> Size2D<T, U> where T: NumCast + Copy,`

#### `cast`

```rust
pub fn cast<NewT>(self) -> Size2D<NewT, U> where NewT: NumCast,
```

Cast from one numeric representation to another, preserving the units.

When casting from floating point to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.

#### `try_cast`

```rust
pub fn try_cast<NewT>(self) -> Option<Size2D<NewT, U>> where NewT: NumCast,
```

Fallible cast from one numeric representation to another, preserving the units.

When casting from floating point to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.

#### `to_f32`

```rust
pub fn to_f32(self) -> Size2D<f32, U>
```

Cast into an `f32` size.

#### `to_f64`

```rust
pub fn to_f64(self) -> Size2D<f64, U>
```

Cast into an `f64` size.

#### `to_usize`

```rust
pub fn to_usize(self) -> Size2D<usize, U>
```

Cast into an `uint` size, truncating decimals if any.

When casting from floating point sizes, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_u32`

```rust
pub fn to_u32(self) -> Size2D<u32, U>
```

Cast into an `u32` size, truncating decimals if any.

When casting from floating point sizes, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_u64`

```rust
pub fn to_u64(self) -> Size2D<u64, U>
```

Cast into an `u64` size, truncating decimals if any.

When casting from floating point sizes, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_i32`

```rust
pub fn to_i32(self) -> Size2D<i32, U>
```

Cast into an `i32` size, truncating decimals if any.

When casting from floating point sizes, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_i64`

```rust
pub fn to_i64(self) -> Size2D<i64, U>
```

Cast into an `i64` size, truncating decimals if any.

When casting from floating point sizes, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

### `impl<T, U> Size2D<T, U> where T: Float,`

#### `is_finite`

```rust
pub fn is_finite(self) -> bool
```

*Available on crate features std or libm only.*

Returns `true` if all members are finite.

### `impl<T, U> Size2D<T, U> where T: Signed,`

#### `abs`

```rust
pub fn abs(self) -> Size2D<T, U>
```

Computes the absolute value of each component.

For `f32` and `f64`, `NaN` will be returned for component if the component is `NaN`.

For signed integers, `::MIN` will be returned for component if the component is `::MIN`.

#### `is_positive`

```rust
pub fn is_positive(self) -> bool
```

Returns `true` if both components is positive and `false` any component is zero or negative.

### `impl<T, U> Size2D<T, U> where T: PartialOrd,`

#### `min`

```rust
pub fn min(self, other: Size2D<T, U>) -> Size2D<T, U>
```

Returns the size each component of which are minimum of this size and another.

#### `max`

```rust
pub fn max(self, other: Size2D<T, U>) -> Size2D<T, U>
```

Returns the size each component of which are maximum of this size and another.

#### `clamp`

```rust
pub fn clamp(self, start: Size2D<T, U>, end: Size2D<T, U>) -> Size2D<T, U> where T: Copy,
```

Returns the size each component of which clamped by corresponding components of `start` and `end`.

Shortcut for `self.max(start).min(end)`.

#### `contains`

```rust
pub fn contains(self, other: Size2D<T, U>) -> bool
```

#### `greater_than`

```rust
pub fn greater_than(self, other: Size2D<T, U>) -> BoolVector2D
```

Returns vector with results of “greater then” operation on each component.

#### `lower_than`

```rust
pub fn lower_than(self, other: Size2D<T, U>) -> BoolVector2D
```

Returns vector with results of “lower then” operation on each component.

#### `is_empty`

```rust
pub fn is_empty(self) -> bool where T: Zero,
```

Returns `true` if any component of size is zero, negative, or NaN.

### `impl<T, U> Size2D<T, U> where T: PartialEq,`

#### `equal`

```rust
pub fn equal(self, other: Size2D<T, U>) -> BoolVector2D
```

Returns vector with results of “equal” operation on each component.

#### `not_equal`

```rust
pub fn not_equal(self, other: Size2D<T, U>) -> BoolVector2D
```

Returns vector with results of “not equal” operation on each component.

## Trait Implementations

### `impl<T, U> Add<&Size2D<T, U>> for Size2D<T, U> where T: Copy + Add<Output = T>,`

#### `Output`

```rust
type Output = Size2D<T, U>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, other: &Size2D<T, U>) -> Size2D<T, U>
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T, U> Add<Size2D<T, U>> for Point2D<T, U> where T: Add,`

#### `Output`

```rust
type Output = Point2D<<T as Add>::Output, U>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add( self, other: Size2D<T, U>, ) -> <Point2D<T, U> as Add<Size2D<T, U>>>::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T, U> Add for Size2D<T, U> where T: Add,`

#### `Output`

```rust
type Output = Size2D<<T as Add>::Output, U>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, other: Size2D<T, U>) -> <Size2D<T, U> as Add>::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T, U> AddAssign<Size2D<T, U>> for Point2D<T, U> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Size2D<T, U>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T, U> AddAssign for Size2D<T, U> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Size2D<T, U>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T, U> Ceil for Size2D<T, U> where T: Ceil,`

#### `ceil`

```rust
fn ceil(self) -> Size2D<T, U>
```

See [`Size2D::ceil`](struct.Size2D.md).

### `impl<T, U> Clone for Size2D<T, U> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Size2D<T, U>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, U> Debug for Size2D<T, U> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, U> Default for Size2D<T, U> where T: Default,`

#### `default`

```rust
fn default() -> Size2D<T, U>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for Size2D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = Size2D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <Size2D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U> Div<T> for Size2D<T, U> where T: Copy + Div,`

#### `Output`

```rust
type Output = Size2D<<T as Div>::Output, U>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, scale: T) -> <Size2D<T, U> as Div<T>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Size2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<T> for Size2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, other: T)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> Floor for Size2D<T, U> where T: Floor,`

#### `floor`

```rust
fn floor(self) -> Size2D<T, U>
```

See [`Size2D::floor`](struct.Size2D.md).

### `impl<T, U> From<[T; 2]> for Size2D<T, U>`

#### `from`

```rust
fn from(_: [T; 2]) -> Size2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<(T, T)> for Size2D<T, U>`

#### `from`

```rust
fn from(tuple: (T, T)) -> Size2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<Size2D<T, U>> for Box2D<T, U> where T: Copy + Zero + PartialOrd,`

#### `from`

```rust
fn from(b: Size2D<T, U>) -> Box2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<Size2D<T, U>> for Rect<T, U> where T: Zero,`

#### `from`

```rust
fn from(size: Size2D<T, U>) -> Rect<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<Size2D<T, U>> for Vector2D<T, U>`

#### `from`

```rust
fn from(s: Size2D<T, U>) -> Vector2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<Vector2D<T, U>> for Size2D<T, U>`

#### `from`

```rust
fn from(v: Vector2D<T, U>) -> Size2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> Hash for Size2D<T, U> where T: Hash,`

#### `hash`

```rust
fn hash<H>(&self, h: &mut H) where H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for Size2D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Size2D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <Size2D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U> Mul<T> for Size2D<T, U> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Size2D<<T as Mul>::Output, U>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: T) -> <Size2D<T, U> as Mul<T>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Size2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<T> for Size2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, other: T)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> Neg for Size2D<T, U> where T: Neg,`

#### `Output`

```rust
type Output = Size2D<<T as Neg>::Output, U>
```

The resulting type after applying the `-` operator.

#### `neg`

```rust
fn neg(self) -> <Size2D<T, U> as Neg>::Output
```

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

### `impl<T, U> PartialEq for Size2D<T, U> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Size2D<T, U>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, U> Round for Size2D<T, U> where T: Round,`

#### `round`

```rust
fn round(self) -> Size2D<T, U>
```

See [`Size2D::round`](struct.Size2D.md).

### `impl<T, U> Sub<Size2D<T, U>> for Point2D<T, U> where T: Sub,`

#### `Output`

```rust
type Output = Point2D<<T as Sub>::Output, U>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub( self, other: Size2D<T, U>, ) -> <Point2D<T, U> as Sub<Size2D<T, U>>>::Output
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T, U> Sub for Size2D<T, U> where T: Sub,`

#### `Output`

```rust
type Output = Size2D<<T as Sub>::Output, U>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, other: Size2D<T, U>) -> <Size2D<T, U> as Sub>::Output
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T, U> SubAssign<Size2D<T, U>> for Point2D<T, U> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Size2D<T, U>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<T, U> SubAssign for Size2D<T, U> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Size2D<T, U>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<'a, T, U> Sum<&'a Size2D<T, U>> for Size2D<T, U> where T: 'a + Add<Output = T> + Copy + Zero, U: 'a,`

#### `sum`

```rust
fn sum<I>(iter: I) -> Size2D<T, U> where I: Iterator<Item = &'a Size2D<T, U>>,
```

Takes an iterator and generates `Self` from the elements by “summing up” the items.

### `impl<T, U> Sum for Size2D<T, U> where T: Add<Output = T> + Zero,`

#### `sum`

```rust
fn sum<I>(iter: I) -> Size2D<T, U> where I: Iterator<Item = Size2D<T, U>>,
```

Takes an iterator and generates `Self` from the elements by “summing up” the items.

### `impl<T, U> Zero for Size2D<T, U> where T: Zero,`

#### `zero`

```rust
fn zero() -> Size2D<T, U>
```

### `impl<T, U> Copy for Size2D<T, U> where T: Copy,`

### `impl<T, U> Eq for Size2D<T, U> where T: Eq,`

## Auto Trait Implementations

- `impl<T, U> Freeze for Size2D<T, U> where T: Freeze,`
- `impl<T, U> RefUnwindSafe for Size2D<T, U> where T: RefUnwindSafe, U: RefUnwindSafe,`
- `impl<T, U> Send for Size2D<T, U> where T: Send, U: Send,`
- `impl<T, U> Sync for Size2D<T, U> where T: Sync, U: Sync,`
- `impl<T, U> Unpin for Size2D<T, U> where T: Unpin, U: Unpin,`
- `impl<T, U> UnsafeUnpin for Size2D<T, U> where T: UnsafeUnpin,`
- `impl<T, U> UnwindSafe for Size2D<T, U> where T: UnwindSafe, U: UnwindSafe,`
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

### `impl<T> Downcast<T> for T`

#### `downcast`

```rust
fn downcast(&self) -> &T
```

### `impl<T> Downcast for T where T: Any,`

#### `into_any`

```rust
fn into_any(self: Box<T>) -> Box<dyn Any>
```

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `into_any_rc`

```rust
fn into_any_rc(self: Rc<T>) -> Rc<dyn Any>
```

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `as_any`

```rust
fn as_any(&self) -> &(dyn Any + 'static)
```

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

#### `as_any_mut`

```rust
fn as_any_mut(&mut self) -> &mut (dyn Any + 'static)
```

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

### `impl<T> DowncastSync for T where T: Any + Send + Sync,`

#### `into_any_arc`

```rust
fn into_any_arc(self: Arc<T>) -> Arc<dyn Any + Sync + Send>
```

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Checks if this value is equivalent to the given key. Read more

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Compare self to `key` and return `true` if they are equal.

### `impl<T> From<T> for T`

#### `from`

```rust
fn from(t: T) -> T
```

Returns the argument unchanged.

### `impl<T> Instrument for T`

#### `instrument`

```rust
fn instrument(self, span: Span) -> Instrumented<Self>
```

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

#### `in_current_span`

```rust
fn in_current_span(self) -> Instrumented<Self>
```

Instruments this type with the current `Span`, returning an `Instrumented` wrapper. Read more

### `impl<T, U> Into<U> for T where U: From<T>,`

#### `into`

```rust
fn into(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

### `impl<T> IntoEither for T`

#### `into_either`

```rust
fn into_either(self, into_left: bool) -> Either<Self, Self>
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either)

#### `into_either_with`

```rust
fn into_either_with<F>(self, into_left: F) -> Either<Self, Self> where F: FnOnce(&Self) -> bool,
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either_with)

### `impl<T> Pointable for T`

#### `ALIGN`

```rust
const ALIGN: usize
```

The alignment of pointer.

#### `Init`

```rust
type Init = T
```

The type for initializers.

#### `init`

```rust
unsafe fn init(init: <T as Pointable>::Init) -> usize
```

Initializes a with the given initializer. Read more

#### `deref`

```rust
unsafe fn deref<'a>(ptr: usize) -> &'a T
```

Dereferences the given pointer. Read more

#### `deref_mut`

```rust
unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T
```

Mutably dereferences the given pointer. Read more

#### `drop`

```rust
unsafe fn drop(ptr: usize)
```

Drops the object pointed to by the given pointer. Read more

### `impl<R, P> ReadPrimitive<R> for P where R: Read + ReadEndian<P>, P: Default,`

#### `read_from_little_endian`

```rust
fn read_from_little_endian(read: &mut R) -> Result<Self, Error>
```

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

#### `read_from_big_endian`

```rust
fn read_from_big_endian(read: &mut R) -> Result<Self, Error>
```

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

#### `read_from_native_endian`

```rust
fn read_from_native_endian(read: &mut R) -> Result<Self, Error>
```

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

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

### `impl<T> Upcast<T> for T`

#### `upcast`

```rust
fn upcast(&self) -> Option<&T>
```

### `impl<T> WithSubscriber for T`

#### `with_subscriber`

```rust
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> where S: Into<Dispatch>,
```

Attaches the provided `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

#### `with_current_subscriber`

```rust
fn with_current_subscriber(self) -> WithDispatch<Self>
```

Attaches the current default `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
