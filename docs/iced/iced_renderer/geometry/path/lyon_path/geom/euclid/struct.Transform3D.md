---
title: "Transform3D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Transform3D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Transform3D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Transform3D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Transform3D`

```rust
pub struct Transform3D<T, Src, Dst> {Show 16 fields
    pub m11: T,
    pub m12: T,
    pub m13: T,
    pub m14: T,
    pub m21: T,
    pub m22: T,
    pub m23: T,
    pub m24: T,
    pub m31: T,
    pub m32: T,
    pub m33: T,
    pub m34: T,
    pub m41: T,
    pub m42: T,
    pub m43: T,
    pub m44: T,
    /* private fields */
}
```

A 3d transform stored as a column-major 4 by 4 matrix.

Transforms can be parametrized over the source and destination units, to describe a transformation from a space to another. For example, `Transform3D<f32, WorldSpace, ScreenSpace>::transform_point3d` takes a `Point3D<f32, WorldSpace>` and returns a `Point3D<f32, ScreenSpace>`.

Transforms expose a set of convenience methods for pre- and post-transformations. Pre-transformations (`pre_*` methods) correspond to adding an operation that is applied before the rest of the transformation, while post-transformations (`then_*` methods) add an operation that is applied after.

When translating `Transform3D` into general matrix representations, consider that the representation follows the column major notation with column vectors.

```
 |x'|   | m11 m12 m13 m14 |   |x|
 |y'|   | m21 m22 m23 m24 |   |y|
 |z'| = | m31 m32 m33 m34 | x |z|
 |w |   | m41 m42 m43 m44 |   |1|
```

The translation terms are `m41`, `m42` and `m43`.

## Fields

### `m11: T`

### `m12: T`

### `m13: T`

### `m14: T`

### `m21: T`

### `m22: T`

### `m23: T`

### `m24: T`

### `m31: T`

### `m32: T`

### `m33: T`

### `m34: T`

### `m41: T`

### `m42: T`

### `m43: T`

### `m44: T`

## Methods

### `impl<T, Src, Dst> Transform3D<T, Src, Dst>`

#### `new`

```rust
pub const fn new( m11: T, m12: T, m13: T, m14: T, m21: T, m22: T, m23: T, m24: T, m31: T, m32: T, m33: T, m34: T, m41: T, m42: T, m43: T, m44: T, ) -> Transform3D<T, Src, Dst>
```

Create a transform specifying all of it’s component as a 4 by 4 matrix.

Components are specified following column-major-column-vector matrix notation. For example, the translation terms m41, m42, m43 are the 13rd, 14th and 15th parameters.

```rust
use euclid::default::Transform3D;
let tx = 1.0;
let ty = 2.0;
let tz = 3.0;
let translation = Transform3D::new(
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 1.0, 0.0,
  tx,  ty,  tz,  1.0,
);
```

#### `new_2d`

```rust
pub fn new_2d( m11: T, m12: T, m21: T, m22: T, m41: T, m42: T, ) -> Transform3D<T, Src, Dst> where T: Zero + One,
```

Create a transform representing a 2d transformation from the components of a 2 by 3 matrix transformation.

Components follow the column-major-column-vector notation (m41 and m42 representing the translation terms).

```
m11  m12   0   0
m21  m22   0   0
  0    0   1   0
m41  m42   0   1
```

#### `is_2d`

```rust
pub fn is_2d(&self) -> bool where T: Zero + One + PartialEq,
```

Returns `true` if this transform can be represented with a `Transform2D`.

See [https://drafts.csswg.org/css-transforms/#2d-transform](https://drafts.csswg.org/css-transforms/#2d-transform)

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: Copy,`

#### `to_array`

```rust
pub fn to_array(&self) -> [T; 16]
```

Returns an array containing this transform’s terms.

The terms are laid out in the same order as they are specified in `Transform3D::new`, that is following the column-major-column-vector matrix notation.

For example the translation terms are found on the 13th, 14th and 15th slots of the array.

#### `to_array_transposed`

```rust
pub fn to_array_transposed(&self) -> [T; 16]
```

Returns an array containing this transform’s terms transposed.

The terms are laid out in transposed order from the same order of `Transform3D::new` and `Transform3D::to_array`, that is following the row-major-column-vector matrix notation.

For example the translation terms are found at indices 3, 7 and 11 of the array.

#### `to_arrays`

```rust
pub fn to_arrays(&self) -> [[T; 4]; 4]
```

Equivalent to `to_array` with elements packed four at a time in an array of arrays.

#### `to_arrays_transposed`

```rust
pub fn to_arrays_transposed(&self) -> [[T; 4]; 4]
```

Equivalent to `to_array_transposed` with elements packed four at a time in an array of arrays.

#### `from_array`

```rust
pub fn from_array(array: [T; 16]) -> Transform3D<T, Src, Dst>
```

Create a transform providing its components via an array of 16 elements instead of as individual parameters.

The order of the components corresponds to the column-major-column-vector matrix notation (the same order as `Transform3D::new`).

#### `from_arrays`

```rust
pub fn from_arrays(array: [[T; 4]; 4]) -> Transform3D<T, Src, Dst>
```

Equivalent to `from_array` with elements packed four at a time in an array of arrays.

The order of the components corresponds to the column-major-column-vector matrix notation (the same order as `Transform3D::new`).

#### `from_untyped`

```rust
pub fn from_untyped( m: &Transform3D<T, UnknownUnit, UnknownUnit>, ) -> Transform3D<T, Src, Dst>
```

Tag a unitless value with units.

#### `to_untyped`

```rust
pub fn to_untyped(&self) -> Transform3D<T, UnknownUnit, UnknownUnit>
```

Drop the units, preserving only the numeric value.

#### `with_source`

```rust
pub fn with_source<NewSrc>(&self) -> Transform3D<T, NewSrc, Dst>
```

Returns the same transform with a different source unit.

#### `with_destination`

```rust
pub fn with_destination<NewDst>(&self) -> Transform3D<T, Src, NewDst>
```

Returns the same transform with a different destination unit.

#### `to_2d`

```rust
pub fn to_2d(&self) -> Transform2D<T, Src, Dst>
```

Create a 2D transform picking the relevant terms from this transform.

This method assumes that self represents a 2d transformation, callers should check that [`is_2d`](struct.Transform3D.md) returns `true` beforehand.

#### `is_scale_offset_2d_eps`

```rust
pub fn is_scale_offset_2d_eps(&self, epsilon: T) -> bool where T: Signed + PartialOrd,
```

Returns true if self can be represented as a 2d scale+offset transform.

#### `to_scale_offset2d`

```rust
pub fn to_scale_offset2d(&self) -> Option<ScaleOffset2D<T, Src, Dst>> where T: Signed + One + PartialOrd,
```

Creates a 2D scale+offset transform from the current transform.

This method assumes that self can be represented as a 2d scale+offset transformation, callers should check that [`is_scale_offset_2d`] or [`is_scale_offset_2d_eps`] returns `true` beforehand.

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: Zero + One,`

#### `identity`

```rust
pub fn identity() -> Transform3D<T, Src, Dst>
```

Creates an identity matrix:

```
1 0 0 0
0 1 0 0
0 0 1 0
0 0 0 1
```

#### `perspective`

```rust
pub fn perspective(d: T) -> Transform3D<T, Src, Dst> where T: Neg<Output = T> + Div<Output = T>,
```

Create a simple perspective transform, projecting to the plane `z = -d`.

```
1   0   0   0
0   1   0   0
0   0   1 -1/d
0   0   0   1
```

See [https://drafts.csswg.org/css-transforms-2/#PerspectiveDefined](https://drafts.csswg.org/css-transforms-2/#PerspectiveDefined).

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,`

#### `then`

```rust
pub fn then<NewDst>( &self, other: &Transform3D<T, Dst, NewDst>, ) -> Transform3D<T, Src, NewDst>
```

Returns the multiplication of the two matrices such that mat’s transformation applies after self’s transformation.

Assuming row vectors, this is equivalent to self * mat

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: Zero + One,`

#### `translation`

```rust
pub fn translation(x: T, y: T, z: T) -> Transform3D<T, Src, Dst>
```

Create a 3d translation transform:

```
1 0 0 0
0 1 0 0
0 0 1 0
x y z 1
```

#### `pre_translate`

```rust
pub fn pre_translate(&self, v: Vector3D<T, Src>) -> Transform3D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,
```

Returns a transform with a translation applied before self’s transformation.

#### `then_translate`

```rust
pub fn then_translate(&self, v: Vector3D<T, Dst>) -> Transform3D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,
```

Returns a transform with a translation applied after self’s transformation.

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: Zero + One,`

#### `scale`

```rust
pub fn scale(x: T, y: T, z: T) -> Transform3D<T, Src, Dst>
```

Create a 3d scale transform:

```
x 0 0 0
0 y 0 0
0 0 z 0
0 0 0 1
```

#### `pre_scale`

```rust
pub fn pre_scale(&self, x: T, y: T, z: T) -> Transform3D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,
```

Returns a transform with a scale applied before self’s transformation.

#### `then_scale`

```rust
pub fn then_scale(&self, x: T, y: T, z: T) -> Transform3D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,
```

Returns a transform with a scale applied after self’s transformation.

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,`

#### `transform_point2d_homogeneous`

```rust
pub fn transform_point2d_homogeneous( &self, p: Point2D<T, Src>, ) -> HomogeneousVector<T, Dst>
```

Returns the homogeneous vector corresponding to the transformed 2d point.

The input point must be use the unit Src, and the returned point has the unit Dst.

#### `transform_point2d`

```rust
pub fn transform_point2d(&self, p: Point2D<T, Src>) -> Option<Point2D<T, Dst>> where T: Div<Output = T> + Zero + PartialOrd,
```

Returns the given 2d point transformed by this transform, if the transform makes sense, or `None` otherwise.

The input point must be use the unit Src, and the returned point has the unit Dst.

#### `transform_vector2d`

```rust
pub fn transform_vector2d(&self, v: Vector2D<T, Src>) -> Vector2D<T, Dst>
```

Returns the given 2d vector transformed by this matrix.

The input point must be use the unit Src, and the returned point has the unit Dst.

#### `transform_point3d_homogeneous`

```rust
pub fn transform_point3d_homogeneous( &self, p: Point3D<T, Src>, ) -> HomogeneousVector<T, Dst>
```

Returns the homogeneous vector corresponding to the transformed 3d point.

The input point must be use the unit Src, and the returned point has the unit Dst.

#### `transform_point3d`

```rust
pub fn transform_point3d(&self, p: Point3D<T, Src>) -> Option<Point3D<T, Dst>> where T: Div<Output = T> + Zero + PartialOrd,
```

Returns the given 3d point transformed by this transform, if the transform makes sense, or `None` otherwise.

The input point must be use the unit Src, and the returned point has the unit Dst.

#### `transform_vector3d`

```rust
pub fn transform_vector3d(&self, v: Vector3D<T, Src>) -> Vector3D<T, Dst>
```

Returns the given 3d vector transformed by this matrix.

The input point must be use the unit Src, and the returned point has the unit Dst.

#### `outer_transformed_rect`

```rust
pub fn outer_transformed_rect( &self, rect: &Rect<T, Src>, ) -> Option<Rect<T, Dst>> where T: Sub<Output = T> + Div<Output = T> + Zero + PartialOrd,
```

Returns a rectangle that encompasses the result of transforming the given rectangle by this transform, if the transform makes sense for it, or `None` otherwise.

#### `outer_transformed_box2d`

```rust
pub fn outer_transformed_box2d( &self, b: &Box2D<T, Src>, ) -> Option<Box2D<T, Dst>> where T: Sub<Output = T> + Div<Output = T> + Zero + PartialOrd,
```

Returns a 2d box that encompasses the result of transforming the given box by this transform, if the transform makes sense for it, or `None` otherwise.

#### `outer_transformed_box3d`

```rust
pub fn outer_transformed_box3d( &self, b: &Box3D<T, Src>, ) -> Option<Box3D<T, Dst>> where T: Sub<Output = T> + Div<Output = T> + Zero + PartialOrd,
```

Returns a 3d box that encompasses the result of transforming the given box by this transform, if the transform makes sense for it, or `None` otherwise.

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + Neg<Output = T> + Add<Output = T> + PartialOrd + One + Zero,`

#### `ortho`

```rust
pub fn ortho( left: T, right: T, bottom: T, top: T, near: T, far: T, ) -> Transform3D<T, Src, Dst>
```

Create an orthogonal projection transform.

#### `is_backface_visible`

```rust
pub fn is_backface_visible(&self) -> bool
```

Check whether shapes on the XY plane with Z pointing towards the screen transformed by this matrix would be facing back.

#### `is_invertible`

```rust
pub fn is_invertible(&self) -> bool
```

Returns whether it is possible to compute the inverse transform.

#### `inverse`

```rust
pub fn inverse(&self) -> Option<Transform3D<T, Dst, Src>>
```

Returns the inverse transform if possible.

#### `determinant`

```rust
pub fn determinant(&self) -> T
```

Compute the determinant of the transform.

#### `mul_s`

```rust
pub fn mul_s(&self, x: T) -> Transform3D<T, Src, Dst>
```

Multiplies all of the transform’s component by a scalar and returns the result.

#### `from_scale`

```rust
pub fn from_scale(scale: Scale<T, Src, Dst>) -> Transform3D<T, Src, Dst>
```

Convenience function to create a scale transform from a `Scale`.

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: Copy + Mul<Output = T> + Div<Output = T> + Zero + One + PartialEq,`

#### `project_to_2d`

```rust
pub fn project_to_2d(&self) -> Transform3D<T, Src, Dst>
```

Returns a projection of this transform in 2d space.

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: NumCast + Copy,`

#### `cast`

```rust
pub fn cast<NewT>(&self) -> Transform3D<NewT, Src, Dst> where NewT: NumCast,
```

Cast from one numeric representation to another, preserving the units.

#### `try_cast`

```rust
pub fn try_cast<NewT>(&self) -> Option<Transform3D<NewT, Src, Dst>> where NewT: NumCast,
```

Fallible cast from one numeric representation to another, preserving the units.

### `impl<T, Src, Dst> Transform3D<T, Src, Dst>`

#### `is_scale_offset_2d`

```rust
pub fn is_scale_offset_2d(&self) -> bool where T: Copy + Signed + PartialOrd + ApproxEq<T>,
```

Returns true if self can be represented as a 2d scale+offset transform, using `T`’s default epsilon value.

#### `skew`

```rust
pub fn skew(alpha: Angle<T>, beta: Angle<T>) -> Transform3D<T, Src, Dst> where T: Trig + Zero + One,
```

Create a 2d skew transform.

See [https://drafts.csswg.org/css-transforms/#funcdef-skew](https://drafts.csswg.org/css-transforms/#funcdef-skew)

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + Zero + One + Trig,`

#### `rotation`

```rust
pub fn rotation(x: T, y: T, z: T, theta: Angle<T>) -> Transform3D<T, Src, Dst>
```

Create a 3d rotation transform from an angle / axis. The supplied axis must be normalized.

#### `then_rotate`

```rust
pub fn then_rotate( &self, x: T, y: T, z: T, theta: Angle<T>, ) -> Transform3D<T, Src, Dst>
```

Returns a transform with a rotation applied after self’s transformation.

#### `pre_rotate`

```rust
pub fn pre_rotate( &self, x: T, y: T, z: T, theta: Angle<T>, ) -> Transform3D<T, Src, Dst>
```

Returns a transform with a rotation applied before self’s transformation.

### `impl<T, Src, Dst> Transform3D<T, Src, Dst> where T: ApproxEq<T>,`

#### `approx_eq`

```rust
pub fn approx_eq(&self, other: &Transform3D<T, Src, Dst>) -> bool
```

Returns `true` if this transform is approximately equal to the other one, using `T`’s default epsilon value.

The same as [`ApproxEq::approx_eq`](approxeq/trait.ApproxEq.md) but available without importing trait.

#### `approx_eq_eps`

```rust
pub fn approx_eq_eps(&self, other: &Transform3D<T, Src, Dst>, eps: &T) -> bool
```

Returns `true` if this transform is approximately equal to the other one, using a provided epsilon value.

The same as [`ApproxEq::approx_eq_eps`](approxeq/trait.ApproxEq.md) but available without importing trait.

## Trait Implementations

### `impl<T, Src, Dst> ApproxEq<T> for Transform3D<T, Src, Dst> where T: ApproxEq<T>,`

#### `approx_epsilon`

```rust
fn approx_epsilon() -> T
```

Default epsilon value

#### `approx_eq_eps`

```rust
fn approx_eq_eps(&self, other: &Transform3D<T, Src, Dst>, eps: &T) -> bool
```

Returns `true` if this object is approximately equal to the other one, using a provided epsilon value.

#### `approx_eq`

```rust
fn approx_eq(&self, other: &Self) -> bool
```

Returns `true` if this object is approximately equal to the other one, using the [`approx_epsilon`](approxeq/trait.ApproxEq.md) epsilon value.

### `impl<T, Src, Dst> Clone for Transform3D<T, Src, Dst> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Transform3D<T, Src, Dst>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, Src, Dst> Debug for Transform3D<T, Src, Dst> where T: Copy + Debug + PartialEq + One + Zero,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, Src, Dst> Default for Transform3D<T, Src, Dst> where T: Zero + One,`

#### `default`

```rust
fn default() -> Transform3D<T, Src, Dst>
```

Returns the [identity transform](struct.Transform3D.md).

### `impl<T, Src, Dst> From<Rotation3D<T, Src, Dst>> for Transform3D<T, Src, Dst> where T: Real + ApproxEq<T>,`

#### `from`

```rust
fn from(r: Rotation3D<T, Src, Dst>) -> Transform3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Scale<T, Src, Dst>> for Transform3D<T, Src, Dst> where T: Copy + Zero + One,`

#### `from`

```rust
fn from(s: Scale<T, Src, Dst>) -> Transform3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Transform2D<T, Src, Dst>> for Transform3D<T, Src, Dst> where T: Copy + Zero + One,`

#### `from`

```rust
fn from(t: Transform2D<T, Src, Dst>) -> Transform3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Translation3D<T, Src, Dst>> for Transform3D<T, Src, Dst> where T: Zero + One,`

#### `from`

```rust
fn from(t: Translation3D<T, Src, Dst>) -> Transform3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> Hash for Transform3D<T, Src, Dst> where T: Hash,`

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

### `impl<T, Src, Dst> PartialEq for Transform3D<T, Src, Dst> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Transform3D<T, Src, Dst>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, Src, Dst> Copy for Transform3D<T, Src, Dst> where T: Copy,`

### `impl<T, Src, Dst> Eq for Transform3D<T, Src, Dst> where T: Eq,`

## Auto Trait Implementations

- `impl<T, Src, Dst> Freeze for Transform3D<T, Src, Dst> where T: Freeze,`
- `impl<T, Src, Dst> RefUnwindSafe for Transform3D<T, Src, Dst> where T: RefUnwindSafe, Src: RefUnwindSafe, Dst: RefUnwindSafe,`
- `impl<T, Src, Dst> Send for Transform3D<T, Src, Dst> where T: Send, Src: Send, Dst: Send,`
- `impl<T, Src, Dst> Sync for Transform3D<T, Src, Dst> where T: Sync, Src: Sync, Dst: Sync,`
- `impl<T, Src, Dst> Unpin for Transform3D<T, Src, Dst> where T: Unpin, Src: Unpin, Dst: Unpin,`
- `impl<T, Src, Dst> UnsafeUnpin for Transform3D<T, Src, Dst> where T: UnsafeUnpin,`
- `impl<T, Src, Dst> UnwindSafe for Transform3D<T, Src, Dst> where T: UnwindSafe, Src: UnwindSafe, Dst: UnwindSafe,`
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
