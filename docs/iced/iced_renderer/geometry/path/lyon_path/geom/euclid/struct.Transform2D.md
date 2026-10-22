---
title: "Transform2D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Transform2D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Transform2D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Transform2D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Transform2D`

```rust
pub struct Transform2D<T, Src, Dst> {
    pub m11: T,
    pub m12: T,
    pub m21: T,
    pub m22: T,
    pub m31: T,
    pub m32: T,
    /* private fields */
}
```

A 2d transform represented by a column-major 3 by 3 matrix, compressed down to 3 by 2.

Transforms can be parametrized over the source and destination units, to describe a transformation from a space to another. For example, `Transform2D<f32, WorldSpace, ScreenSpace>::transform_point4d` takes a `Point2D<f32, WorldSpace>` and returns a `Point2D<f32, ScreenSpace>`.

Transforms expose a set of convenience methods for pre- and post-transformations. Pre-transformations (`pre_*` methods) correspond to adding an operation that is applied before the rest of the transformation, while post-transformations (`then_*` methods) add an operation that is applied after.

The matrix representation is conceptually equivalent to a 3 by 3 matrix transformation compressed to 3 by 2 with the components that aren’t needed to describe the set of 2d transformations we are interested in implicitly defined:

```
 | m11 m21 m31 |   |x|   |x'|
 | m12 m22 m32 | x |y| = |y'|
 |   0   0   1 |   |1|   |1 |
```

When translating `Transform2D` into general matrix representations, consider that the representation follows the column-major notation with column vectors.

The translation terms are `m31` and `m32`.

## Fields

### `m11: T`

### `m12: T`

### `m21: T`

### `m22: T`

### `m31: T`

### `m32: T`

## Methods

### `impl<T, Src, Dst> Transform2D<T, Src, Dst>`

#### `new`

```rust
pub const fn new( m11: T, m12: T, m21: T, m22: T, m31: T, m32: T, ) -> Transform2D<T, Src, Dst>
```

Create a transform specifying its components in using the column-major-column-vector matrix notation.

For example, the translation terms m31 and m32 are the last two parameters parameters.

```rust
use euclid::default::Transform2D;
let tx = 1.0;
let ty = 2.0;
let translation = Transform2D::new(
  1.0, 0.0,
  0.0, 1.0,
  tx,  ty,
);
```

### `impl<T, Src, Dst> Transform2D<T, Src, Dst> where T: Copy,`

#### `to_array`

```rust
pub fn to_array(&self) -> [T; 6]
```

Returns an array containing this transform’s terms.

The terms are laid out in the same order as they are specified in [`Transform2D::new`](struct.Transform2D.md), that is following the column-major-column-vector matrix notation.

For example the translation terms are found in the last two slots of the array.

#### `to_array_transposed`

```rust
pub fn to_array_transposed(&self) -> [T; 6]
```

Returns an array containing this transform’s terms transposed.

The terms are laid out in transposed order from the same order of `Transform3D::new` and `Transform3D::to_array`, that is following the row-major-column-vector matrix notation.

For example the translation terms are found at indices 2 and 5 in the array.

#### `to_arrays`

```rust
pub fn to_arrays(&self) -> [[T; 2]; 3]
```

Equivalent to `to_array` with elements packed two at a time in an array of arrays.

#### `from_array`

```rust
pub fn from_array(array: [T; 6]) -> Transform2D<T, Src, Dst>
```

Create a transform providing its components via an array of 6 elements instead of as individual parameters.

The order of the components corresponds to the column-major-column-vector matrix notation (the same order as `Transform2D::new`).

#### `from_arrays`

```rust
pub fn from_arrays(array: [[T; 2]; 3]) -> Transform2D<T, Src, Dst>
```

Equivalent to `from_array` with elements packed two at a time in an array of arrays.

The order of the components corresponds to the column-major-column-vector matrix notation (the same order as `Transform3D::new`).

#### `to_untyped`

```rust
pub fn to_untyped(&self) -> Transform2D<T, UnknownUnit, UnknownUnit>
```

Drop the units, preserving only the numeric value.

#### `from_untyped`

```rust
pub fn from_untyped( p: &Transform2D<T, UnknownUnit, UnknownUnit>, ) -> Transform2D<T, Src, Dst>
```

Tag a unitless value with units.

#### `with_source`

```rust
pub fn with_source<NewSrc>(&self) -> Transform2D<T, NewSrc, Dst>
```

Returns the same transform with a different source unit.

#### `with_destination`

```rust
pub fn with_destination<NewDst>(&self) -> Transform2D<T, Src, NewDst>
```

Returns the same transform with a different destination unit.

#### `to_3d`

```rust
pub fn to_3d(&self) -> Transform3D<T, Src, Dst> where T: Zero + One,
```

Create a 3D transform from the current transform

#### `is_scale_offset_eps`

```rust
pub fn is_scale_offset_eps(&self, epsilon: T) -> bool where T: Signed + One + PartialOrd,
```

Returns true if self can be represented as a 2d scale+offset transform.

#### `to_scale_offset`

```rust
pub fn to_scale_offset(&self) -> ScaleOffset2D<T, Src, Dst>
```

Creates a 2D scale+offset transform from the current transform.

This method assumes that self can be represented as a 2d scale+offset transformation, callers should check that [`is_scale_offset_2d`] or [`is_scale_offset_2d_eps`] returns `true` beforehand.

### `impl<T, Src, Dst> Transform2D<T, Src, Dst> where T: NumCast + Copy,`

#### `cast`

```rust
pub fn cast<NewT>(&self) -> Transform2D<NewT, Src, Dst> where NewT: NumCast,
```

Cast from one numeric representation to another, preserving the units.

#### `try_cast`

```rust
pub fn try_cast<NewT>(&self) -> Option<Transform2D<NewT, Src, Dst>> where NewT: NumCast,
```

Fallible cast from one numeric representation to another, preserving the units.

### `impl<T, Src, Dst> Transform2D<T, Src, Dst> where T: Zero + One,`

#### `identity`

```rust
pub fn identity() -> Transform2D<T, Src, Dst>
```

Create an identity matrix:

```
1 0
0 1
0 0
```

### `impl<T, Src, Dst> Transform2D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,`

#### `then`

```rust
pub fn then<NewDst>( &self, mat: &Transform2D<T, Dst, NewDst>, ) -> Transform2D<T, Src, NewDst>
```

Returns the multiplication of the two matrices such that mat’s transformation applies after self’s transformation.

### `impl<T, Src, Dst> Transform2D<T, Src, Dst> where T: Zero + One,`

#### `translation`

```rust
pub fn translation(x: T, y: T) -> Transform2D<T, Src, Dst>
```

Create a 2d translation transform:

```
1 0
0 1
x y
```

#### `then_translate`

```rust
pub fn then_translate(&self, v: Vector2D<T, Dst>) -> Transform2D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,
```

Applies a translation after self’s transformation and returns the resulting transform.

#### `pre_translate`

```rust
pub fn pre_translate(&self, v: Vector2D<T, Src>) -> Transform2D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,
```

Applies a translation before self’s transformation and returns the resulting transform.

### `impl<T, Src, Dst> Transform2D<T, Src, Dst>`

#### `scale`

```rust
pub fn scale(x: T, y: T) -> Transform2D<T, Src, Dst> where T: Zero,
```

Create a 2d scale transform:

```
x 0
0 y
0 0
```

#### `then_scale`

```rust
pub fn then_scale(&self, x: T, y: T) -> Transform2D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T> + Zero,
```

Applies a scale after self’s transformation and returns the resulting transform.

#### `pre_scale`

```rust
pub fn pre_scale(&self, x: T, y: T) -> Transform2D<T, Src, Dst> where T: Copy + Mul<Output = T>,
```

Applies a scale before self’s transformation and returns the resulting transform.

### `impl<T, Src, Dst> Transform2D<T, Src, Dst> where T: Copy + Add<Output = T> + Mul<Output = T>,`

#### `transform_point`

```rust
pub fn transform_point(&self, point: Point2D<T, Src>) -> Point2D<T, Dst>
```

Returns the given point transformed by this transform.

#### `transform_vector`

```rust
pub fn transform_vector(&self, vec: Vector2D<T, Src>) -> Vector2D<T, Dst>
```

Returns the given vector transformed by this matrix.

#### `outer_transformed_rect`

```rust
pub fn outer_transformed_rect(&self, rect: &Rect<T, Src>) -> Rect<T, Dst> where T: Sub<Output = T> + Zero + PartialOrd,
```

Returns a rectangle that encompasses the result of transforming the given rectangle by this transform.

#### `outer_transformed_box`

```rust
pub fn outer_transformed_box(&self, b: &Box2D<T, Src>) -> Box2D<T, Dst> where T: Sub<Output = T> + Zero + PartialOrd,
```

Returns a box that encompasses the result of transforming the given box by this transform.

### `impl<T, Src, Dst> Transform2D<T, Src, Dst> where T: Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialEq + Zero + One,`

#### `determinant`

```rust
pub fn determinant(&self) -> T
```

Computes and returns the determinant of this transform.

#### `is_invertible`

```rust
pub fn is_invertible(&self) -> bool
```

Returns whether it is possible to compute the inverse transform.

#### `inverse`

```rust
pub fn inverse(&self) -> Option<Transform2D<T, Dst, Src>>
```

Returns the inverse transform if possible.

### `impl<T, Src, Dst> Transform2D<T, Src, Dst>`

#### `approx_eq`

```rust
pub fn approx_eq(&self, other: &Transform2D<T, Src, Dst>) -> bool where T: ApproxEq<T>,
```

Returns `true` if this transform is approximately equal to the other one, using `T`’s default epsilon value.

The same as [`ApproxEq::approx_eq`](approxeq/trait.ApproxEq.md) but available without importing trait.

#### `approx_eq_eps`

```rust
pub fn approx_eq_eps(&self, other: &Transform2D<T, Src, Dst>, eps: &T) -> bool where T: ApproxEq<T>,
```

Returns `true` if this transform is approximately equal to the other one, using a provided epsilon value.

The same as [`ApproxEq::approx_eq_eps`](approxeq/trait.ApproxEq.md) but available without importing trait.

#### `is_scale_offset`

```rust
pub fn is_scale_offset(&self) -> bool where T: Copy + Signed + One + PartialOrd + ApproxEq<T>,
```

Returns true if self can be represented as a 2d scale+offset transform, using `T`’s default epsilon value.

### `impl<T, Src, Dst> Transform2D<T, Src, Dst> where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Zero + Trig,`

#### `rotation`

```rust
pub fn rotation(theta: Angle<T>) -> Transform2D<T, Src, Dst>
```

Returns a rotation transform.fn pre_translate

#### `then_rotate`

```rust
pub fn then_rotate(&self, theta: Angle<T>) -> Transform2D<T, Src, Dst>
```

Applies a rotation after self’s transformation and returns the resulting transform.

#### `pre_rotate`

```rust
pub fn pre_rotate(&self, theta: Angle<T>) -> Transform2D<T, Src, Dst>
```

Applies a rotation before self’s transformation and returns the resulting transform.

## Trait Implementations

### `impl<T, Src, Dst> ApproxEq<T> for Transform2D<T, Src, Dst> where T: ApproxEq<T>,`

#### `approx_eq_eps`

```rust
fn approx_eq_eps(&self, other: &Transform2D<T, Src, Dst>, eps: &T) -> bool
```

Returns `true` if this transform is approximately equal to the other one, using a provided epsilon value.

#### `approx_epsilon`

```rust
fn approx_epsilon() -> T
```

Default epsilon value

#### `approx_eq`

```rust
fn approx_eq(&self, other: &Self) -> bool
```

Returns `true` if this object is approximately equal to the other one, using the [`approx_epsilon`](approxeq/trait.ApproxEq.md) epsilon value.

### `impl<T, Src, Dst> Clone for Transform2D<T, Src, Dst> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Transform2D<T, Src, Dst>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, Src, Dst> Debug for Transform2D<T, Src, Dst> where T: Copy + Debug + PartialEq + One + Zero,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, Src, Dst> Default for Transform2D<T, Src, Dst> where T: Zero + One,`

#### `default`

```rust
fn default() -> Transform2D<T, Src, Dst>
```

Returns the [identity transform](struct.Transform2D.md).

### `impl<T, Src, Dst> From<Rotation2D<T, Src, Dst>> for Transform2D<T, Src, Dst> where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Zero + Trig,`

#### `from`

```rust
fn from(r: Rotation2D<T, Src, Dst>) -> Transform2D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Scale<T, Src, Dst>> for Transform2D<T, Src, Dst> where T: Copy + Zero,`

#### `from`

```rust
fn from(s: Scale<T, Src, Dst>) -> Transform2D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<ScaleOffset2D<T, Src, Dst>> for Transform2D<T, Src, Dst> where T: Copy + Zero,`

#### `from`

```rust
fn from(t: ScaleOffset2D<T, Src, Dst>) -> Transform2D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Transform2D<T, Src, Dst>> for Transform3D<T, Src, Dst> where T: Copy + Zero + One,`

#### `from`

```rust
fn from(t: Transform2D<T, Src, Dst>) -> Transform3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Translation2D<T, Src, Dst>> for Transform2D<T, Src, Dst> where T: Zero + One,`

#### `from`

```rust
fn from(t: Translation2D<T, Src, Dst>) -> Transform2D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> Hash for Transform2D<T, Src, Dst> where T: Hash,`

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

### `impl<T, Src, Dst> PartialEq for Transform2D<T, Src, Dst> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Transform2D<T, Src, Dst>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<S> Transformation<S> for Transform2D<S, UnknownUnit, UnknownUnit> where S: Scalar,`

#### `transform_point`

```rust
fn transform_point(&self, p: Point2D<S, UnknownUnit>) -> Point2D<S, UnknownUnit>
```

#### `transform_vector`

```rust
fn transform_vector( &self, v: Vector2D<S, UnknownUnit>, ) -> Vector2D<S, UnknownUnit>
```

### `impl<T, Src, Dst> Copy for Transform2D<T, Src, Dst> where T: Copy,`

### `impl<T, Src, Dst> Eq for Transform2D<T, Src, Dst> where T: Eq,`

## Auto Trait Implementations

- `impl<T, Src, Dst> Freeze for Transform2D<T, Src, Dst> where T: Freeze,`
- `impl<T, Src, Dst> RefUnwindSafe for Transform2D<T, Src, Dst> where T: RefUnwindSafe, Src: RefUnwindSafe, Dst: RefUnwindSafe,`
- `impl<T, Src, Dst> Send for Transform2D<T, Src, Dst> where T: Send, Src: Send, Dst: Send,`
- `impl<T, Src, Dst> Sync for Transform2D<T, Src, Dst> where T: Sync, Src: Sync, Dst: Sync,`
- `impl<T, Src, Dst> Unpin for Transform2D<T, Src, Dst> where T: Unpin, Src: Unpin, Dst: Unpin,`
- `impl<T, Src, Dst> UnsafeUnpin for Transform2D<T, Src, Dst> where T: UnsafeUnpin,`
- `impl<T, Src, Dst> UnwindSafe for Transform2D<T, Src, Dst> where T: UnwindSafe, Src: UnwindSafe, Dst: UnwindSafe,`
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
