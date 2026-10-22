---
title: "Box3D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Box3D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Box3D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Box3D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Box3D`

```rust
#[repr(C)]pub struct Box3D<T, U> {
    pub min: Point3D<T, U>,
    pub max: Point3D<T, U>,
}
```

An axis aligned 3D box represented by its minimum and maximum coordinates.

## Fields

### `min: Point3D<T, U>`

### `max: Point3D<T, U>`

## Methods

### `impl<T, U> Box3D<T, U>`

#### `new`

```rust
pub const fn new(min: Point3D<T, U>, max: Point3D<T, U>) -> Box3D<T, U>
```

Constructor.

#### `from_origin_and_size`

```rust
pub fn from_origin_and_size( origin: Point3D<T, U>, size: Size3D<T, U>, ) -> Box3D<T, U> where T: Copy + Add<Output = T>,
```

Constructor.

#### `from_size`

```rust
pub fn from_size(size: Size3D<T, U>) -> Box3D<T, U> where T: Zero,
```

Creates a `Box3D` of the given size, at offset zero.

### `impl<T, U> Box3D<T, U> where T: PartialOrd,`

#### `is_negative`

```rust
pub fn is_negative(&self) -> bool
```

Returns `true` if the box has a negative volume.

The common interpretation for a negative box is to consider it empty. It can be obtained by calculating the intersection of two boxes that do not intersect.

#### `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

Returns `true` if the size is zero, negative or NaN.

#### `intersects`

```rust
pub fn intersects(&self, other: &Box3D<T, U>) -> bool
```

#### `contains`

```rust
pub fn contains(&self, other: Point3D<T, U>) -> bool
```

Returns `true` if this [`Box3D`](struct.Box3D.md) contains the point `p`.

Points on the front, left, and top faces are inside the box, whereas points on the back, right, and bottom faces are outside the box. See [`Box3D::contains_inclusive`](struct.Box3D.md) for a variant that also includes those latter points.

###### Examples

```rust
use euclid::default::{Box3D, Point3D};

let cube = Box3D::new(Point3D::origin(), Point3D::new(2, 2, 2));

assert!(cube.contains(Point3D::new(1, 1, 1)));

assert!(cube.contains(Point3D::new(0, 1, 1))); // front face
assert!(cube.contains(Point3D::new(1, 0, 1))); // left face
assert!(cube.contains(Point3D::new(1, 1, 0))); // top face
assert!(cube.contains(Point3D::new(0, 0, 0)));

assert!(!cube.contains(Point3D::new(2, 1, 1))); // back face
assert!(!cube.contains(Point3D::new(1, 2, 1))); // right face
assert!(!cube.contains(Point3D::new(1, 1, 2))); // bottom face
assert!(!cube.contains(Point3D::new(2, 2, 2)));
```

#### `contains_inclusive`

```rust
pub fn contains_inclusive(&self, other: Point3D<T, U>) -> bool
```

Returns `true` if this [`Box3D`](struct.Box3D.md) contains the point `p`.

This is like [`Box3D::contains`](struct.Box3D.md), but points on the back, right, and bottom faces are also inside the box.

###### Examples

```rust
use euclid::default::{Box3D, Point3D};

let cube = Box3D::new(Point3D::origin(), Point3D::new(2, 2, 2));

assert!(cube.contains_inclusive(Point3D::new(1, 1, 1)));

assert!(cube.contains_inclusive(Point3D::new(0, 1, 1))); // front face
assert!(cube.contains_inclusive(Point3D::new(1, 0, 1))); // left face
assert!(cube.contains_inclusive(Point3D::new(1, 1, 0))); // top face
assert!(cube.contains_inclusive(Point3D::new(0, 0, 0))); // front-left-top corner

assert!(cube.contains_inclusive(Point3D::new(2, 1, 1))); // back face
assert!(cube.contains_inclusive(Point3D::new(1, 2, 1))); // right face
assert!(cube.contains_inclusive(Point3D::new(1, 1, 2))); // bottom face
assert!(cube.contains_inclusive(Point3D::new(2, 2, 2))); // back-right-bottom corner
```

#### `contains_box`

```rust
pub fn contains_box(&self, other: &Box3D<T, U>) -> bool
```

Returns `true` if this box3d contains the interior of the other box3d. Always returns `true` if other is empty, and always returns `false` if other is nonempty but this box3d is empty.

### `impl<T, U> Box3D<T, U> where T: Copy + PartialOrd,`

#### `to_non_empty`

```rust
pub fn to_non_empty(&self) -> Option<Box3D<T, U>>
```

#### `intersection`

```rust
pub fn intersection(&self, other: &Box3D<T, U>) -> Option<Box3D<T, U>>
```

#### `intersection_unchecked`

```rust
pub fn intersection_unchecked(&self, other: &Box3D<T, U>) -> Box3D<T, U>
```

#### `union`

```rust
pub fn union(&self, other: &Box3D<T, U>) -> Box3D<T, U>
```

Computes the union of two boxes.

If either of the boxes is empty, the other one is returned.

### `impl<T, U> Box3D<T, U> where T: Copy + Add<Output = T>,`

#### `translate`

```rust
pub fn translate(&self, by: Vector3D<T, U>) -> Box3D<T, U>
```

Returns the same box3d, translated by a vector.

### `impl<T, U> Box3D<T, U> where T: Copy + Sub<Output = T>,`

#### `size`

```rust
pub fn size(&self) -> Size3D<T, U>
```

#### `width`

```rust
pub fn width(&self) -> T
```

#### `height`

```rust
pub fn height(&self) -> T
```

#### `depth`

```rust
pub fn depth(&self) -> T
```

### `impl<T, U> Box3D<T, U> where T: Copy + Add<Output = T> + Sub<Output = T>,`

#### `inflate`

```rust
pub fn inflate(&self, width: T, height: T, depth: T) -> Box3D<T, U>
```

Inflates the box by the specified sizes on each side respectively.

### `impl<T, U> Box3D<T, U> where T: Copy + Zero + PartialOrd,`

#### `from_points`

```rust
pub fn from_points<I>(points: I) -> Box3D<T, U> where I: IntoIterator, <I as IntoIterator>::Item: Borrow<Point3D<T, U>>,
```

Returns the smallest box enclosing all of the provided points.

The top/bottom/left/right/front/back-most points are exactly on the box’s edges. Since [`Box3D::contains`](struct.Box3D.md) excludes points that are on the right/bottom/back-most faces, not all points passed to [`Box3D::from_points`](struct.Box3D.md) are contained in the returned [`Box3D`](struct.Box3D.md) when probed with [`Box3D::contains`](struct.Box3D.md), but are when probed with [`Box3D::contains_inclusive`](struct.Box3D.md).

For example:

```rust
use euclid::default::{Point3D, Box3D};

let a = Point3D::origin();
let b = Point3D::new(1, 2, 3);
let box3 = Box3D::from_points([a, b]);

assert_eq!(box3.width(), 1);
assert_eq!(box3.height(), 2);
assert_eq!(box3.depth(), 3);

assert!(box3.contains(a));
assert!(!box3.contains(b));
assert!(box3.contains_inclusive(b));
```

In particular, calling [`Box3D::from_points`](struct.Box3D.md) with a single point results in an empty [`Box3D`](struct.Box3D.md):

```rust
use euclid::default::{Point3D, Box3D};

let a = Point3D::new(1, 0, 1);
let box3 = Box3D::from_points([a]);

assert!(box3.is_empty());
assert!(!box3.contains(a));
assert!(box3.contains_inclusive(a));
```

The [`Box3D`](struct.Box3D.md) enclosing no points is also empty:

```rust
use euclid::default::{Box3D, Point3D};

let box3 = Box3D::from_points(std::iter::empty::<Point3D<i32>>());
assert!(box3.is_empty());
```

### `impl<T, U> Box3D<T, U> where T: One + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,`

#### `lerp`

```rust
pub fn lerp(&self, other: Box3D<T, U>, t: T) -> Box3D<T, U>
```

Linearly interpolate between this box3d and another box3d.

### `impl<T, U> Box3D<T, U> where T: Copy + One + Add<Output = T> + Div<Output = T>,`

#### `center`

```rust
pub fn center(&self) -> Point3D<T, U>
```

### `impl<T, U> Box3D<T, U> where T: Copy + Mul<Output = T> + Sub<Output = T>,`

#### `volume`

```rust
pub fn volume(&self) -> T
```

#### `xy_area`

```rust
pub fn xy_area(&self) -> T
```

#### `yz_area`

```rust
pub fn yz_area(&self) -> T
```

#### `xz_area`

```rust
pub fn xz_area(&self) -> T
```

### `impl<T, U> Box3D<T, U> where T: Zero,`

#### `zero`

```rust
pub fn zero() -> Box3D<T, U>
```

Constructor, setting all sides to zero.

### `impl<T, U> Box3D<T, U> where T: Copy,`

#### `x_range`

```rust
pub fn x_range(&self) -> Range<T>
```

#### `y_range`

```rust
pub fn y_range(&self) -> Range<T>
```

#### `z_range`

```rust
pub fn z_range(&self) -> Range<T>
```

#### `to_untyped`

```rust
pub fn to_untyped(&self) -> Box3D<T, UnknownUnit>
```

Drop the units, preserving only the numeric value.

#### `from_untyped`

```rust
pub fn from_untyped(c: &Box3D<T, UnknownUnit>) -> Box3D<T, U>
```

Tag a unitless value with units.

#### `cast_unit`

```rust
pub fn cast_unit<V>(&self) -> Box3D<T, V>
```

Cast the unit

#### `scale`

```rust
pub fn scale<S>(&self, x: S, y: S, z: S) -> Box3D<T, U> where S: Copy, T: Mul<S, Output = T>,
```

### `impl<T, U> Box3D<T, U> where T: NumCast + Copy,`

#### `cast`

```rust
pub fn cast<NewT>(&self) -> Box3D<NewT, U> where NewT: NumCast,
```

Cast from one numeric representation to another, preserving the units.

When casting from floating point to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using [`round`](struct.Box3D.md), [`round_in`](struct.Box3D.md) or [`round_out`](struct.Box3D.md) before casting.

#### `try_cast`

```rust
pub fn try_cast<NewT>(&self) -> Option<Box3D<NewT, U>> where NewT: NumCast,
```

Fallible cast from one numeric representation to another, preserving the units.

When casting from floating point to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using [`round`](struct.Box3D.md), [`round_in`](struct.Box3D.md) or [`round_out`](struct.Box3D.md) before casting.

#### `to_f32`

```rust
pub fn to_f32(&self) -> Box3D<f32, U>
```

Cast into an `f32` box3d.

#### `to_f64`

```rust
pub fn to_f64(&self) -> Box3D<f64, U>
```

Cast into an `f64` box3d.

#### `to_usize`

```rust
pub fn to_usize(&self) -> Box3D<usize, U>
```

Cast into an `usize` box3d, truncating decimals if any.

When casting from floating point cuboids, it is worth considering whether to `round()`, `round_in()` or `round_out()` before the cast in order to obtain the desired conversion behavior.

#### `to_u32`

```rust
pub fn to_u32(&self) -> Box3D<u32, U>
```

Cast into an `u32` box3d, truncating decimals if any.

When casting from floating point cuboids, it is worth considering whether to `round()`, `round_in()` or `round_out()` before the cast in order to obtain the desired conversion behavior.

#### `to_i32`

```rust
pub fn to_i32(&self) -> Box3D<i32, U>
```

Cast into an `i32` box3d, truncating decimals if any.

When casting from floating point cuboids, it is worth considering whether to `round()`, `round_in()` or `round_out()` before the cast in order to obtain the desired conversion behavior.

#### `to_i64`

```rust
pub fn to_i64(&self) -> Box3D<i64, U>
```

Cast into an `i64` box3d, truncating decimals if any.

When casting from floating point cuboids, it is worth considering whether to `round()`, `round_in()` or `round_out()` before the cast in order to obtain the desired conversion behavior.

### `impl<T, U> Box3D<T, U> where T: Float,`

#### `is_finite`

```rust
pub fn is_finite(self) -> bool
```

*Available on crate features std or libm only.*

Returns `true` if all members are finite.

### `impl<T, U> Box3D<T, U> where T: Round,`

#### `round`

```rust
pub fn round(&self) -> Box3D<T, U>
```

Return a box3d with edges rounded to integer coordinates, such that the returned box3d has the same set of pixel centers as the original one. Values equal to 0.5 round up. Suitable for most places where integral device coordinates are needed, but note that any translation should be applied first to avoid pixel rounding errors. Note that this is *not* rounding to nearest integer if the values are negative. They are always rounding as floor(n + 0.5).

### `impl<T, U> Box3D<T, U> where T: Floor + Ceil,`

#### `round_in`

```rust
pub fn round_in(&self) -> Box3D<T, U>
```

Return a box3d with faces/edges rounded to integer coordinates, such that the original box3d contains the resulting box3d.

#### `round_out`

```rust
pub fn round_out(&self) -> Box3D<T, U>
```

Return a box3d with faces/edges rounded to integer coordinates, such that the original box3d is contained in the resulting box3d.

## Trait Implementations

### `impl<T, U> Clone for Box3D<T, U> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Box3D<T, U>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, U> Debug for Box3D<T, U> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, U> Default for Box3D<T, U> where T: Default,`

#### `default`

```rust
fn default() -> Box3D<T, U>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

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

### `impl<T, U> Div<T> for Box3D<T, U> where T: Copy + Div,`

#### `Output`

```rust
type Output = Box3D<<T as Div>::Output, U>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, scale: T) -> <Box3D<T, U> as Div<T>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Box3D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<T> for Box3D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: T)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> From<Size3D<T, U>> for Box3D<T, U> where T: Copy + Zero + PartialOrd,`

#### `from`

```rust
fn from(b: Size3D<T, U>) -> Box3D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> Hash for Box3D<T, U> where T: Hash,`

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

### `impl<T, U> Mul<T> for Box3D<T, U> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Box3D<<T as Mul>::Output, U>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: T) -> <Box3D<T, U> as Mul<T>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Box3D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<T> for Box3D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: T)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> PartialEq for Box3D<T, U> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Box3D<T, U>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, U> Copy for Box3D<T, U> where T: Copy,`

### `impl<T, U> Eq for Box3D<T, U> where T: Eq,`

## Auto Trait Implementations

- `impl<T, U> Freeze for Box3D<T, U> where T: Freeze,`
- `impl<T, U> RefUnwindSafe for Box3D<T, U> where T: RefUnwindSafe, U: RefUnwindSafe,`
- `impl<T, U> Send for Box3D<T, U> where T: Send, U: Send,`
- `impl<T, U> Sync for Box3D<T, U> where T: Sync, U: Sync,`
- `impl<T, U> Unpin for Box3D<T, U> where T: Unpin, U: Unpin,`
- `impl<T, U> UnsafeUnpin for Box3D<T, U> where T: UnsafeUnpin,`
- `impl<T, U> UnwindSafe for Box3D<T, U> where T: UnwindSafe, U: UnwindSafe,`
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
