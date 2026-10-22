---
title: "Scale"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Scale"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Scale.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Scale.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Scale`

```rust
pub struct Scale<T, Src, Dst>(pub T, _);
```

A scaling factor between two different units of measurement.

This is effectively a type-safe float, intended to be used in combination with other types like `length::Length` to enforce conversion between systems of measurement at compile time.

`Src` and `Dst` represent the units before and after multiplying a value by a `Scale`. They may be types without values, such as empty enums. For example:

```rust
use euclid::Scale;
use euclid::Length;
enum Mm {};
enum Inch {};

let mm_per_inch: Scale<f32, Inch, Mm> = Scale::new(25.4);

let one_foot: Length<f32, Inch> = Length::new(12.0);
let one_foot_in_mm: Length<f32, Mm> = one_foot * mm_per_inch;
```

## Fields

### `0: T`

## Methods

### `impl<T, Src, Dst> Scale<T, Src, Dst>`

#### `new`

```rust
pub const fn new(x: T) -> Scale<T, Src, Dst>
```

#### `identity`

```rust
pub fn identity() -> Scale<T, Src, Dst> where T: One,
```

Creates an identity scale (1.0).

#### `transform_point`

```rust
pub fn transform_point( self, point: Point2D<T, Src>, ) -> Point2D<<T as Mul>::Output, Dst> where T: Copy + Mul,
```

Returns the given point transformed by this scale.

###### Example

```rust
use euclid::{Scale, point2};
enum Mm {};
enum Cm {};

let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);

assert_eq!(to_mm.transform_point(point2(42, -42)), point2(420, -420));
```

#### `transform_point3d`

```rust
pub fn transform_point3d( self, point: Point3D<T, Src>, ) -> Point3D<<T as Mul>::Output, Dst> where T: Copy + Mul,
```

Returns the given point transformed by this scale.

#### `transform_vector`

```rust
pub fn transform_vector( self, vec: Vector2D<T, Src>, ) -> Vector2D<<T as Mul>::Output, Dst> where T: Copy + Mul,
```

Returns the given vector transformed by this scale.

###### Example

```rust
use euclid::{Scale, vec2};
enum Mm {};
enum Cm {};

let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);

assert_eq!(to_mm.transform_vector(vec2(42, -42)), vec2(420, -420));
```

#### `transform_size`

```rust
pub fn transform_size( self, size: Size2D<T, Src>, ) -> Size2D<<T as Mul>::Output, Dst> where T: Copy + Mul,
```

Returns the given size transformed by this scale.

###### Example

```rust
use euclid::{Scale, size2};
enum Mm {};
enum Cm {};

let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);

assert_eq!(to_mm.transform_size(size2(42, -42)), size2(420, -420));
```

#### `transform_rect`

```rust
pub fn transform_rect( self, rect: &Rect<T, Src>, ) -> Rect<<T as Mul>::Output, Dst> where T: Copy + Mul,
```

Returns the given rect transformed by this scale.

###### Example

```rust
use euclid::{Scale, rect};
enum Mm {};
enum Cm {};

let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);

assert_eq!(to_mm.transform_rect(&rect(1, 2, 42, -42)), rect(10, 20, 420, -420));
```

#### `transform_box2d`

```rust
pub fn transform_box2d( self, b: &Box2D<T, Src>, ) -> Box2D<<T as Mul>::Output, Dst> where T: Copy + Mul,
```

Returns the given box transformed by this scale.

#### `transform_box3d`

```rust
pub fn transform_box3d( self, b: &Box3D<T, Src>, ) -> Box3D<<T as Mul>::Output, Dst> where T: Copy + Mul,
```

Returns the given box transformed by this scale.

#### `is_identity`

```rust
pub fn is_identity(self) -> bool where T: PartialEq + One,
```

Returns `true` if this scale has no effect.

###### Example

```rust
use euclid::Scale;
use euclid::num::One;
enum Mm {};
enum Cm {};

let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);
let mm_per_mm: Scale<f32, Mm, Mm> = Scale::new(1.0);

assert_eq!(cm_per_mm.is_identity(), false);
assert_eq!(mm_per_mm.is_identity(), true);
assert_eq!(mm_per_mm, Scale::one());
```

#### `get`

```rust
pub fn get(self) -> T
```

Returns the underlying scalar scale factor.

#### `inverse`

```rust
pub fn inverse(self) -> Scale<<T as Div>::Output, Dst, Src> where T: One + Div,
```

The inverse Scale (1.0 / self).

###### Example

```rust
use euclid::Scale;
enum Mm {};
enum Cm {};

let cm_per_mm: Scale<f32, Cm, Mm> = Scale::new(0.1);

assert_eq!(cm_per_mm.inverse(), Scale::new(10.0));
```

#### `with_source`

```rust
pub fn with_source<NewSrc>(self) -> Scale<T, NewSrc, Dst>
```

Returns the same transform with a different source unit.

#### `with_destination`

```rust
pub fn with_destination<NewDst>(self) -> Scale<T, Src, NewDst>
```

Returns the same transform with a different destination unit.

### `impl<T, Src, Dst> Scale<T, Src, Dst> where T: PartialOrd,`

#### `min`

```rust
pub fn min(self, other: Scale<T, Src, Dst>) -> Scale<T, Src, Dst>
```

#### `max`

```rust
pub fn max(self, other: Scale<T, Src, Dst>) -> Scale<T, Src, Dst>
```

#### `clamp`

```rust
pub fn clamp( self, start: Scale<T, Src, Dst>, end: Scale<T, Src, Dst>, ) -> Scale<T, Src, Dst> where T: Copy,
```

Returns the point each component of which clamped by corresponding components of `start` and `end`.

Shortcut for `self.max(start).min(end)`.

### `impl<T, Src, Dst> Scale<T, Src, Dst> where T: NumCast,`

#### `cast`

```rust
pub fn cast<NewT>(self) -> Scale<NewT, Src, Dst> where NewT: NumCast,
```

Cast from one numeric representation to another, preserving the units.

###### Panics

If the source value cannot be represented by the target type `NewT`, then method panics. Use `try_cast` if that must be case.

###### Example

```rust
use euclid::Scale;
enum Mm {};
enum Cm {};

let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);

assert_eq!(to_mm.cast::<f32>(), Scale::new(10.0));
```

That conversion will panic, because `i32` not enough to store such big numbers:

ⓘ```rust
use euclid::Scale;
enum Mm {};// millimeter = 10^-2 meters
enum Em {};// exameter   = 10^18 meters

// Panics
let to_em: Scale<i32, Mm, Em> = Scale::new(10e20).cast();
```

#### `try_cast`

```rust
pub fn try_cast<NewT>(self) -> Option<Scale<NewT, Src, Dst>> where NewT: NumCast,
```

Fallible cast from one numeric representation to another, preserving the units. If the source value cannot be represented by the target type `NewT`, then `None` is returned.

###### Example

```rust
use euclid::Scale;
enum Mm {};
enum Cm {};
enum Em {};// Exameter = 10^18 meters

let to_mm: Scale<i32, Cm, Mm> = Scale::new(10);
let to_em: Scale<f32, Mm, Em> = Scale::new(10e20);

assert_eq!(to_mm.try_cast::<f32>(), Some(Scale::new(10.0)));
// Integer to small to store that number
assert_eq!(to_em.try_cast::<i32>(), None);
```

## Trait Implementations

### `impl<T, Src, Dst> Add for Scale<T, Src, Dst> where T: Add,`

#### `Output`

```rust
type Output = Scale<<T as Add>::Output, Src, Dst>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, other: Scale<T, Src, Dst>) -> <Scale<T, Src, Dst> as Add>::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T, Src, Dst> Clone for Scale<T, Src, Dst> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Scale<T, Src, Dst>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, Src, Dst> Debug for Scale<T, Src, Dst> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, Src, Dst> Default for Scale<T, Src, Dst> where T: Default,`

#### `default`

```rust
fn default() -> Scale<T, Src, Dst>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<Src, Dst, T> Div<Scale<T, Src, Dst>> for Length<T, Dst> where T: Div,`

#### `Output`

```rust
type Output = Length<<T as Div>::Output, Src>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, Src, Dst>, ) -> <Length<T, Dst> as Div<Scale<T, Src, Dst>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for Box2D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = Box2D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <Box2D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for Box3D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = Box3D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <Box3D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for Point2D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = Point2D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <Point2D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for Point3D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = Point3D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <Point3D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for Rect<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = Rect<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <Rect<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for SideOffsets2D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = SideOffsets2D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <SideOffsets2D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

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

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for Size3D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = Size3D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <Size3D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

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

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for Vector3D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = Vector3D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <Vector3D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Box2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Box3D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Point2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Point3D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Rect<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<Scale<T, U, U>> for SideOffsets2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Size2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Size3D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Vector2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Vector3D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, Src, Dst> From<Scale<T, Src, Dst>> for ScaleOffset2D<T, Src, Dst> where T: Copy + Zero,`

#### `from`

```rust
fn from(s: Scale<T, Src, Dst>) -> ScaleOffset2D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Scale<T, Src, Dst>> for Transform2D<T, Src, Dst> where T: Copy + Zero,`

#### `from`

```rust
fn from(s: Scale<T, Src, Dst>) -> Transform2D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Scale<T, Src, Dst>> for Transform3D<T, Src, Dst> where T: Copy + Zero + One,`

#### `from`

```rust
fn from(s: Scale<T, Src, Dst>) -> Transform3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> Hash for Scale<T, Src, Dst> where T: Hash,`

#### `hash`

```rust
fn hash<H>(&self, state: &mut H) where H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl<T, A, B, C> Mul<Scale<T, B, C>> for Scale<T, A, B> where T: Mul,`

#### `Output`

```rust
type Output = Scale<<T as Mul>::Output, A, C>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, other: Scale<T, B, C>, ) -> <Scale<T, A, B> as Mul<Scale<T, B, C>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<Src, Dst, T> Mul<Scale<T, Src, Dst>> for Length<T, Src> where T: Mul,`

#### `Output`

```rust
type Output = Length<<T as Mul>::Output, Dst>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, Src, Dst>, ) -> <Length<T, Src> as Mul<Scale<T, Src, Dst>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for Box2D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Box2D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <Box2D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for Box3D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Box3D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <Box3D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for Point2D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Point2D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <Point2D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for Point3D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Point3D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <Point3D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for Rect<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Rect<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <Rect<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for SideOffsets2D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = SideOffsets2D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <SideOffsets2D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

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

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for Size3D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Size3D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <Size3D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

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

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for Vector3D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Vector3D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <Vector3D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Box2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Box3D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Point2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Point3D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Rect<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<Scale<T, U, U>> for SideOffsets2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Size2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Size3D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Vector2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Vector3D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, Src, Dst> One for Scale<T, Src, Dst> where T: One,`

#### `one`

```rust
fn one() -> Scale<T, Src, Dst>
```

### `impl<T, Src, Dst> Ord for Scale<T, Src, Dst> where T: Ord,`

#### `cmp`

```rust
fn cmp(&self, other: &Scale<T, Src, Dst>) -> Ordering
```

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

#### `max`

```rust
fn max(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

#### `min`

```rust
fn min(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

#### `clamp`

```rust
fn clamp(self, min: Self, max: Self) -> Self where Self: Sized,
```

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

### `impl<T, Src, Dst> PartialEq for Scale<T, Src, Dst> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Scale<T, Src, Dst>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, Src, Dst> PartialOrd for Scale<T, Src, Dst> where T: PartialOrd,`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Scale<T, Src, Dst>) -> Option<Ordering>
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

### `impl<T, Src, Dst> Sub for Scale<T, Src, Dst> where T: Sub,`

#### `Output`

```rust
type Output = Scale<<T as Sub>::Output, Src, Dst>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, other: Scale<T, Src, Dst>) -> <Scale<T, Src, Dst> as Sub>::Output
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<S> Transformation<S> for Scale<S, UnknownUnit, UnknownUnit> where S: Scalar,`

#### `transform_point`

```rust
fn transform_point(&self, p: Point2D<S, UnknownUnit>) -> Point2D<S, UnknownUnit>
```

#### `transform_vector`

```rust
fn transform_vector( &self, v: Vector2D<S, UnknownUnit>, ) -> Vector2D<S, UnknownUnit>
```

### `impl<T, Src, Dst> Copy for Scale<T, Src, Dst> where T: Copy,`

### `impl<T, Src, Dst> Eq for Scale<T, Src, Dst> where T: Eq,`

## Auto Trait Implementations

- `impl<T, Src, Dst> Freeze for Scale<T, Src, Dst> where T: Freeze,`
- `impl<T, Src, Dst> RefUnwindSafe for Scale<T, Src, Dst> where T: RefUnwindSafe, Src: RefUnwindSafe, Dst: RefUnwindSafe,`
- `impl<T, Src, Dst> Send for Scale<T, Src, Dst> where T: Send, Src: Send, Dst: Send,`
- `impl<T, Src, Dst> Sync for Scale<T, Src, Dst> where T: Sync, Src: Sync, Dst: Sync,`
- `impl<T, Src, Dst> Unpin for Scale<T, Src, Dst> where T: Unpin, Src: Unpin, Dst: Unpin,`
- `impl<T, Src, Dst> UnsafeUnpin for Scale<T, Src, Dst> where T: UnsafeUnpin,`
- `impl<T, Src, Dst> UnwindSafe for Scale<T, Src, Dst> where T: UnwindSafe, Src: UnwindSafe, Dst: UnwindSafe,`
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

### `impl<Q, K> Comparable<K> for Q where Q: Ord + ?Sized, K: Borrow<Q> + ?Sized,`

#### `compare`

```rust
fn compare(&self, key: &K) -> Ordering
```

Compare self to `key` and return their ordering.

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
