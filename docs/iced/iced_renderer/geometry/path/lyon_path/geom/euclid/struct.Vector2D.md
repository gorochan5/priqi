---
title: "Vector2D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Vector2D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Vector2D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Vector2D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Vector2D`

```rust
pub struct Vector2D<T, U> {
    pub x: T,
    pub y: T,
    /* private fields */
}
```

A 2d Vector tagged with a unit.

## Fields

### `x: T`

The `x` (traditionally, horizontal) coordinate.

### `y: T`

The `y` (traditionally, vertical) coordinate.

## Methods

### `impl<T, U> Vector2D<T, U>`

#### `zero`

```rust
pub fn zero() -> Vector2D<T, U> where T: Zero,
```

Constructor, setting all components to zero.

#### `one`

```rust
pub fn one() -> Vector2D<T, U> where T: One,
```

Constructor, setting all components to one.

#### `new`

```rust
pub const fn new(x: T, y: T) -> Vector2D<T, U>
```

Constructor taking scalar values directly.

#### `splat`

```rust
pub fn splat(v: T) -> Vector2D<T, U> where T: Clone,
```

Constructor setting all components to the same value.

#### `from_lengths`

```rust
pub fn from_lengths(x: Length<T, U>, y: Length<T, U>) -> Vector2D<T, U>
```

Constructor taking properly Lengths instead of scalar values.

#### `from_untyped`

```rust
pub fn from_untyped(p: Vector2D<T, UnknownUnit>) -> Vector2D<T, U>
```

Tag a unit-less value with units.

#### `map`

```rust
pub fn map<V, F>(self, f: F) -> Vector2D<V, U> where F: FnMut(T) -> V,
```

Apply the function `f` to each component of this vector.

###### Example

This may be used to perform unusual arithmetic which is not already offered as methods.

```rust
use euclid::default::Vector2D;

let p = Vector2D::<u32>::new(5, 11);
assert_eq!(p.map(|coord| coord.saturating_sub(10)), Vector2D::new(0, 1));
```

#### `zip`

```rust
pub fn zip<V, F>(self, rhs: Vector2D<T, U>, f: F) -> Vector2D<V, U> where F: FnMut(T, T) -> V,
```

Apply the function `f` to each pair of components of this point and `rhs`.

###### Example

This may be used to perform unusual arithmetic which is not already offered as methods.

```rust
use euclid::default::Vector2D;

let a: Vector2D<u8> = Vector2D::new(50, 200);
let b: Vector2D<u8> = Vector2D::new(100, 100);
assert_eq!(a.zip(b, u8::saturating_add), Vector2D::new(150, 255));
```

#### `abs`

```rust
pub fn abs(self) -> Vector2D<T, U> where T: Signed,
```

Computes the vector with absolute values of each component.

###### Example

```rust
enum U {}

assert_eq!(vec2::<_, U>(-1, 2).abs(), vec2(1, 2));

let vec = vec2::<_, U>(f32::NAN, -f32::MAX).abs();
assert!(vec.x.is_nan());
assert_eq!(vec.y, f32::MAX);
```

###### Panics

The behavior for each component follows the scalar type’s implementation of `num_traits::Signed::abs`.

#### `dot`

```rust
pub fn dot(self, other: Vector2D<T, U>) -> T where T: Add<Output = T> + Mul<Output = T>,
```

Dot product.

#### `cross`

```rust
pub fn cross(self, other: Vector2D<T, U>) -> T where T: Sub<Output = T> + Mul<Output = T>,
```

Returns the norm of the cross product [self.x, self.y, 0] x [other.x, other.y, 0].

#### `component_mul`

```rust
pub fn component_mul(self, other: Vector2D<T, U>) -> Vector2D<T, U> where T: Mul<Output = T>,
```

Returns the component-wise multiplication of the two vectors.

#### `component_div`

```rust
pub fn component_div(self, other: Vector2D<T, U>) -> Vector2D<T, U> where T: Div<Output = T>,
```

Returns the component-wise division of the two vectors.

### `impl<T, U> Vector2D<T, U> where T: Copy,`

#### `extend`

```rust
pub fn extend(self, z: T) -> Vector3D<T, U>
```

Create a 3d vector from this one, using the specified z value.

#### `to_point`

```rust
pub fn to_point(self) -> Point2D<T, U>
```

Cast this vector into a point.

Equivalent to adding this vector to the origin.

#### `yx`

```rust
pub fn yx(self) -> Vector2D<T, U>
```

Swap x and y.

#### `to_size`

```rust
pub fn to_size(self) -> Size2D<T, U>
```

Cast this vector into a size.

#### `to_untyped`

```rust
pub fn to_untyped(self) -> Vector2D<T, UnknownUnit>
```

Drop the units, preserving only the numeric value.

#### `cast_unit`

```rust
pub fn cast_unit<V>(self) -> Vector2D<T, V>
```

Cast the unit.

#### `to_array`

```rust
pub fn to_array(self) -> [T; 2]
```

Cast into an array with x and y.

#### `to_tuple`

```rust
pub fn to_tuple(self) -> (T, T)
```

Cast into a tuple with x and y.

#### `to_3d`

```rust
pub fn to_3d(self) -> Vector3D<T, U> where T: Zero,
```

Convert into a 3d vector with `z` coordinate equals to `T::zero()`.

#### `round`

```rust
pub fn round(self) -> Vector2D<T, U> where T: Round,
```

Rounds each component to the nearest integer value.

This behavior is preserved for negative values (unlike the basic cast).

```rust
enum Mm {}

assert_eq!(vec2::<_, Mm>(-0.1, -0.8).round(), vec2::<_, Mm>(0.0, -1.0))
```

#### `ceil`

```rust
pub fn ceil(self) -> Vector2D<T, U> where T: Ceil,
```

Rounds each component to the smallest integer equal or greater than the original value.

This behavior is preserved for negative values (unlike the basic cast).

```rust
enum Mm {}

assert_eq!(vec2::<_, Mm>(-0.1, -0.8).ceil(), vec2::<_, Mm>(0.0, 0.0))
```

#### `floor`

```rust
pub fn floor(self) -> Vector2D<T, U> where T: Floor,
```

Rounds each component to the biggest integer equal or lower than the original value.

This behavior is preserved for negative values (unlike the basic cast).

```rust
enum Mm {}

assert_eq!(vec2::<_, Mm>(-0.1, -0.8).floor(), vec2::<_, Mm>(-1.0, -1.0))
```

#### `to_transform`

```rust
pub fn to_transform(self) -> Transform2D<T, U, U> where T: Zero + One,
```

Creates translation by this vector in vector units.

### `impl<T, U> Vector2D<T, U> where T: Copy + Mul<Output = T> + Add<Output = T>,`

#### `square_length`

```rust
pub fn square_length(self) -> T
```

Returns the vector’s length squared.

#### `project_onto_vector`

```rust
pub fn project_onto_vector(self, onto: Vector2D<T, U>) -> Vector2D<T, U> where T: Sub<Output = T> + Div<Output = T>,
```

Returns this vector projected onto another one.

Projecting onto a nil vector will cause a division by zero.

### `impl<T, U> Vector2D<T, U> where T: Float,`

#### `robust_normalize`

```rust
pub fn robust_normalize(self) -> Vector2D<T, U>
```

Return the normalized vector even if the length is larger than the max value of Float.

#### `is_finite`

```rust
pub fn is_finite(self) -> bool
```

Returns true if all members are finite.

### `impl<T, U> Vector2D<T, U> where T: Real,`

#### `length`

```rust
pub fn length(self) -> T
```

Returns the vector length.

#### `normalize`

```rust
pub fn normalize(self) -> Vector2D<T, U>
```

Returns the vector with length of one unit.

#### `try_normalize`

```rust
pub fn try_normalize(self) -> Option<Vector2D<T, U>>
```

Returns the vector with length of one unit.

Unlike `Vector2D::normalize`, this returns None in the case that the length of the vector is zero.

#### `with_length`

```rust
pub fn with_length(self, length: T) -> Vector2D<T, U>
```

Return this vector scaled to fit the provided length.

#### `with_max_length`

```rust
pub fn with_max_length(self, max_length: T) -> Vector2D<T, U>
```

Return this vector capped to a maximum length.

#### `with_min_length`

```rust
pub fn with_min_length(self, min_length: T) -> Vector2D<T, U>
```

Return this vector with a minimum length applied.

#### `clamp_length`

```rust
pub fn clamp_length(self, min: T, max: T) -> Vector2D<T, U>
```

Return this vector with minimum and maximum lengths applied.

### `impl<T, U> Vector2D<T, U>`

#### `from_angle_and_length`

```rust
pub fn from_angle_and_length(angle: Angle<T>, length: T) -> Vector2D<T, U> where T: Trig + Mul<Output = T> + Copy,
```

Constructor taking angle and length

#### `angle_from_x_axis`

```rust
pub fn angle_from_x_axis(self) -> Angle<T> where T: Trig,
```

Returns the signed angle between this vector and the x axis. Positive values counted counterclockwise, where 0 is `+x` axis, `PI/2` is `+y` axis.

The returned angle is between -PI and PI.

#### `angle_to`

```rust
pub fn angle_to(self, other: Vector2D<T, U>) -> Angle<T> where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Trig,
```

Returns the signed angle between this vector and another vector.

The returned angle is between -PI and PI.

### `impl<T, U> Vector2D<T, U> where T: One + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,`

#### `lerp`

```rust
pub fn lerp(self, other: Vector2D<T, U>, t: T) -> Vector2D<T, U>
```

Linearly interpolate each component between this vector and another vector.

###### Example

```rust
use euclid::vec2;
use euclid::default::Vector2D;

let from: Vector2D<_> = vec2(0.0, 10.0);
let to:  Vector2D<_> = vec2(8.0, -4.0);

assert_eq!(from.lerp(to, -1.0), vec2(-8.0,  24.0));
assert_eq!(from.lerp(to,  0.0), vec2( 0.0,  10.0));
assert_eq!(from.lerp(to,  0.5), vec2( 4.0,   3.0));
assert_eq!(from.lerp(to,  1.0), vec2( 8.0,  -4.0));
assert_eq!(from.lerp(to,  2.0), vec2(16.0, -18.0));
```

#### `reflect`

```rust
pub fn reflect(self, normal: Vector2D<T, U>) -> Vector2D<T, U>
```

Returns a reflection vector using an incident ray and a surface normal.

### `impl<T, U> Vector2D<T, U> where T: PartialOrd,`

#### `min`

```rust
pub fn min(self, other: Vector2D<T, U>) -> Vector2D<T, U>
```

Returns the vector each component of which are minimum of this vector and another.

#### `max`

```rust
pub fn max(self, other: Vector2D<T, U>) -> Vector2D<T, U>
```

Returns the vector each component of which are maximum of this vector and another.

#### `clamp`

```rust
pub fn clamp(self, start: Vector2D<T, U>, end: Vector2D<T, U>) -> Vector2D<T, U> where T: Copy,
```

Returns the vector each component of which is clamped by corresponding components of `start` and `end`.

Shortcut for `self.max(start).min(end)`.

#### `greater_than`

```rust
pub fn greater_than(self, other: Vector2D<T, U>) -> BoolVector2D
```

Returns vector with results of “greater than” operation on each component.

#### `lower_than`

```rust
pub fn lower_than(self, other: Vector2D<T, U>) -> BoolVector2D
```

Returns vector with results of “lower than” operation on each component.

### `impl<T, U> Vector2D<T, U> where T: PartialEq,`

#### `equal`

```rust
pub fn equal(self, other: Vector2D<T, U>) -> BoolVector2D
```

Returns vector with results of “equal” operation on each component.

#### `not_equal`

```rust
pub fn not_equal(self, other: Vector2D<T, U>) -> BoolVector2D
```

Returns vector with results of “not equal” operation on each component.

### `impl<T, U> Vector2D<T, U> where T: NumCast + Copy,`

#### `cast`

```rust
pub fn cast<NewT>(self) -> Vector2D<NewT, U> where NewT: NumCast,
```

Cast from one numeric representation to another, preserving the units.

When casting from floating vector to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.

#### `try_cast`

```rust
pub fn try_cast<NewT>(self) -> Option<Vector2D<NewT, U>> where NewT: NumCast,
```

Fallible cast from one numeric representation to another, preserving the units.

When casting from floating vector to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.

#### `to_f32`

```rust
pub fn to_f32(self) -> Vector2D<f32, U>
```

Cast into an `f32` vector.

#### `to_f64`

```rust
pub fn to_f64(self) -> Vector2D<f64, U>
```

Cast into an `f64` vector.

#### `to_usize`

```rust
pub fn to_usize(self) -> Vector2D<usize, U>
```

Cast into an `usize` vector, truncating decimals if any.

When casting from floating vector vectors, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_isize`

```rust
pub fn to_isize(self) -> Vector2D<isize, U>
```

Cast into an `isize` vector, truncating decimals if any.

When casting from floating vector vectors, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_u32`

```rust
pub fn to_u32(self) -> Vector2D<u32, U>
```

Cast into an `u32` vector, truncating decimals if any.

When casting from floating vector vectors, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_i32`

```rust
pub fn to_i32(self) -> Vector2D<i32, U>
```

Cast into an i32 vector, truncating decimals if any.

When casting from floating vector vectors, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_i64`

```rust
pub fn to_i64(self) -> Vector2D<i64, U>
```

Cast into an i64 vector, truncating decimals if any.

When casting from floating vector vectors, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

## Trait Implementations

### `impl<T, U> Add<&Vector2D<T, U>> for Vector2D<T, U> where T: Add + Copy,`

#### `Output`

```rust
type Output = Vector2D<<T as Add>::Output, U>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add( self, other: &Vector2D<T, U>, ) -> <Vector2D<T, U> as Add<&Vector2D<T, U>>>::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T, U> Add<Vector2D<T, U>> for Point2D<T, U> where T: Add,`

#### `Output`

```rust
type Output = Point2D<<T as Add>::Output, U>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add( self, other: Vector2D<T, U>, ) -> <Point2D<T, U> as Add<Vector2D<T, U>>>::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T, U> Add for Vector2D<T, U> where T: Add,`

#### `Output`

```rust
type Output = Vector2D<<T as Add>::Output, U>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, other: Vector2D<T, U>) -> <Vector2D<T, U> as Add>::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T, U> AddAssign<Vector2D<T, U>> for Point2D<T, U> where T: Copy + Add<Output = T>,`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Vector2D<T, U>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T, U> AddAssign for Vector2D<T, U> where T: Copy + Add<Output = T>,`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Vector2D<T, U>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T, U> ApproxEq<Vector2D<T, U>> for Vector2D<T, U> where T: ApproxEq<T>,`

#### `approx_epsilon`

```rust
fn approx_epsilon() -> Vector2D<T, U>
```

Default epsilon value

#### `approx_eq_eps`

```rust
fn approx_eq_eps(&self, other: &Vector2D<T, U>, eps: &Vector2D<T, U>) -> bool
```

Returns `true` if this object is approximately equal to the other one, using a provided epsilon value.

#### `approx_eq`

```rust
fn approx_eq(&self, other: &Self) -> bool
```

Returns `true` if this object is approximately equal to the other one, using the [`approx_epsilon`](approxeq/trait.ApproxEq.md) epsilon value.

### `impl<T, U> Ceil for Vector2D<T, U> where T: Ceil,`

#### `ceil`

```rust
fn ceil(self) -> Vector2D<T, U>
```

See [`Vector2D::ceil`](struct.Vector2D.md).

### `impl<T, U> Clone for Vector2D<T, U> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Vector2D<T, U>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, U> Debug for Vector2D<T, U> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, U> Default for Vector2D<T, U> where T: Default,`

#### `default`

```rust
fn default() -> Vector2D<T, U>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for Vector2D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = Vector2D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <Vector2D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U> Div<T> for Vector2D<T, U> where T: Copy + Div,`

#### `Output`

```rust
type Output = Vector2D<<T as Div>::Output, U>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, scale: T) -> <Vector2D<T, U> as Div<T>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Vector2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<T> for Vector2D<T, U> where T: Copy + Div<Output = T>,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: T)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> Floor for Vector2D<T, U> where T: Floor,`

#### `floor`

```rust
fn floor(self) -> Vector2D<T, U>
```

See [`Vector2D::floor`](struct.Vector2D.md).

### `impl<T, U> From<[T; 2]> for Vector2D<T, U>`

#### `from`

```rust
fn from(_: [T; 2]) -> Vector2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<(T, T)> for Vector2D<T, U>`

#### `from`

```rust
fn from(tuple: (T, T)) -> Vector2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<Size2D<T, U>> for Vector2D<T, U>`

#### `from`

```rust
fn from(s: Size2D<T, U>) -> Vector2D<T, U>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Translation2D<T, Src, Dst>> for Vector2D<T, Src>`

#### `from`

```rust
fn from(t: Translation2D<T, Src, Dst>) -> Vector2D<T, Src>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Vector2D<T, Src>> for Translation2D<T, Src, Dst>`

#### `from`

```rust
fn from(v: Vector2D<T, Src>) -> Translation2D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, U> From<Vector2D<T, U>> for HomogeneousVector<T, U> where T: Zero,`

#### `from`

```rust
fn from(v: Vector2D<T, U>) -> HomogeneousVector<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<Vector2D<T, U>> for Size2D<T, U>`

#### `from`

```rust
fn from(v: Vector2D<T, U>) -> Size2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> Hash for Vector2D<T, U> where T: Hash,`

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

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for Vector2D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Vector2D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <Vector2D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U> Mul<T> for Vector2D<T, U> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Vector2D<<T as Mul>::Output, U>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: T) -> <Vector2D<T, U> as Mul<T>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Vector2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<T> for Vector2D<T, U> where T: Copy + Mul<Output = T>,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: T)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> Neg for Vector2D<T, U> where T: Neg,`

#### `Output`

```rust
type Output = Vector2D<<T as Neg>::Output, U>
```

The resulting type after applying the `-` operator.

#### `neg`

```rust
fn neg(self) -> <Vector2D<T, U> as Neg>::Output
```

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

### `impl<T, U> PartialEq for Vector2D<T, U> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Vector2D<T, U>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, U> Round for Vector2D<T, U> where T: Round,`

#### `round`

```rust
fn round(self) -> Vector2D<T, U>
```

See [`Vector2D::round`](struct.Vector2D.md).

### `impl<T, U> Sub<Vector2D<T, U>> for Point2D<T, U> where T: Sub,`

#### `Output`

```rust
type Output = Point2D<<T as Sub>::Output, U>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub( self, other: Vector2D<T, U>, ) -> <Point2D<T, U> as Sub<Vector2D<T, U>>>::Output
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T, U> Sub for Vector2D<T, U> where T: Sub,`

#### `Output`

```rust
type Output = Vector2D<<T as Sub>::Output, U>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, other: Vector2D<T, U>) -> <Vector2D<T, U> as Sub>::Output
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T, U> SubAssign<Vector2D<T, U>> for Point2D<T, U> where T: Copy + Sub<Output = T>,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Vector2D<T, U>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<T, U> SubAssign for Vector2D<T, U> where T: Copy + Sub<Output = T>,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Vector2D<T, U>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<'a, T, U> Sum<&'a Vector2D<T, U>> for Vector2D<T, U> where T: 'a + Add<Output = T> + Copy + Zero, U: 'a,`

#### `sum`

```rust
fn sum<I>(iter: I) -> Vector2D<T, U> where I: Iterator<Item = &'a Vector2D<T, U>>,
```

Takes an iterator and generates `Self` from the elements by “summing up” the items.

### `impl<T, U> Sum for Vector2D<T, U> where T: Add<Output = T> + Zero,`

#### `sum`

```rust
fn sum<I>(iter: I) -> Vector2D<T, U> where I: Iterator<Item = Vector2D<T, U>>,
```

Takes an iterator and generates `Self` from the elements by “summing up” the items.

### `impl<T, U> Zero for Vector2D<T, U> where T: Zero,`

#### `zero`

```rust
fn zero() -> Vector2D<T, U>
```

Constructor, setting all components to zero.

### `impl<T, U> Copy for Vector2D<T, U> where T: Copy,`

### `impl<T, U> Eq for Vector2D<T, U> where T: Eq,`

## Auto Trait Implementations

- `impl<T, U> Freeze for Vector2D<T, U> where T: Freeze,`
- `impl<T, U> RefUnwindSafe for Vector2D<T, U> where T: RefUnwindSafe, U: RefUnwindSafe,`
- `impl<T, U> Send for Vector2D<T, U> where T: Send, U: Send,`
- `impl<T, U> Sync for Vector2D<T, U> where T: Sync, U: Sync,`
- `impl<T, U> Unpin for Vector2D<T, U> where T: Unpin, U: Unpin,`
- `impl<T, U> UnsafeUnpin for Vector2D<T, U> where T: UnsafeUnpin,`
- `impl<T, U> UnwindSafe for Vector2D<T, U> where T: UnwindSafe, U: UnwindSafe,`
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
