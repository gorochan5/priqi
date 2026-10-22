---
title: "Box2D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Box2D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Box2D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Box2D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Box2D`

```rust
#[repr(C)]pub struct Box2D<T, U> {
    pub min: Point2D<T, U>,
    pub max: Point2D<T, U>,
}
```

A 2d axis aligned rectangle represented by its minimum and maximum coordinates.

## Representation

This struct is similar to [`Rect`](struct.Rect.md), but stores rectangle as two endpoints instead of origin point and size. Such representation has several advantages over [`Rect`](struct.Rect.md) representation:

- Several operations are more efficient with `Box2D`, including [`intersection`](struct.Box2D.md), [`union`](struct.Box2D.md), and point-in-rect.
- The representation is less susceptible to overflow. With [`Rect`](struct.Rect.md), computation of second point can overflow for a large range of values of origin and size. However, with `Box2D`, computation of [`size`](struct.Box2D.md) cannot overflow if the coordinates are signed and the resulting size is unsigned.

A known disadvantage of `Box2D` is that translating the rectangle requires translating both points, whereas translating [`Rect`](struct.Rect.md) only requires translating one point.

## Empty box

A box is considered empty (see [`is_empty`](struct.Box2D.md)) if any of the following is true:

- it’s area is empty,
- it’s area is negative (`min.x > max.x` or `min.y > max.y`),
- it contains NaNs.

## Fields

### `min: Point2D<T, U>`

### `max: Point2D<T, U>`

## Methods

### `impl<T, U> Box2D<T, U>`

#### `new`

```rust
pub const fn new(min: Point2D<T, U>, max: Point2D<T, U>) -> Box2D<T, U>
```

Constructor.

#### `from_origin_and_size`

```rust
pub fn from_origin_and_size( origin: Point2D<T, U>, size: Size2D<T, U>, ) -> Box2D<T, U> where T: Copy + Add<Output = T>,
```

Constructor.

#### `from_size`

```rust
pub fn from_size(size: Size2D<T, U>) -> Box2D<T, U> where T: Zero,
```

Creates a `Box2D` of the given size, at offset zero.

### `impl<T, U> Box2D<T, U> where T: PartialOrd,`

#### `is_negative`

```rust
pub fn is_negative(&self) -> bool
```

Returns `true` if the box has a negative area.

The common interpretation for a negative box is to consider it empty. It can be obtained by calculating the intersection of two boxes that do not intersect.

#### `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

Returns `true` if the size is zero, negative or NaN.

#### `intersects`

```rust
pub fn intersects(&self, other: &Box2D<T, U>) -> bool
```

Returns `true` if the two boxes intersect.

#### `contains`

```rust
pub fn contains(&self, p: Point2D<T, U>) -> bool
```

Returns `true` if this [`Box2D`](struct.Box2D.md) contains the point `p`.

Points on the top and left edges are inside the box, whereas points on the bottom and right edges are outside the box. See [`Box2D::contains_inclusive`](struct.Box2D.md) for a variant that also includes those latter points.

###### Examples

```rust
use euclid::default::{Box2D, Point2D};

let rect = Box2D::new(Point2D::origin(), Point2D::new(2, 2));

assert!(rect.contains(Point2D::new(1, 1)));

assert!(rect.contains(Point2D::new(0, 1))); // left edge
assert!(rect.contains(Point2D::new(1, 0))); // top edge
assert!(rect.contains(Point2D::origin()));

assert!(!rect.contains(Point2D::new(2, 1))); // right edge
assert!(!rect.contains(Point2D::new(1, 2))); // bottom edge
assert!(!rect.contains(Point2D::new(2, 2)));
```

#### `contains_inclusive`

```rust
pub fn contains_inclusive(&self, p: Point2D<T, U>) -> bool
```

Returns `true` if this box contains the point `p`.

This is like [`Box2D::contains`](struct.Box2D.md), but points on the bottom and right edges are also inside the box.

###### Examples

```rust
use euclid::default::{Box2D, Point2D};

let rect = Box2D::new(Point2D::origin(), Point2D::new(2, 2));

assert!(rect.contains_inclusive(Point2D::new(1, 1)));

assert!(rect.contains_inclusive(Point2D::new(0, 1))); // left edge
assert!(rect.contains_inclusive(Point2D::new(1, 0))); // top edge
assert!(rect.contains_inclusive(Point2D::origin()));

assert!(rect.contains_inclusive(Point2D::new(2, 1))); // right edge
assert!(rect.contains_inclusive(Point2D::new(1, 2))); // bottom edge
assert!(rect.contains_inclusive(Point2D::new(2, 2)));
```

#### `contains_box`

```rust
pub fn contains_box(&self, other: &Box2D<T, U>) -> bool
```

Returns `true` if this box contains the interior of the other box. Always returns `true` if other is empty, and always returns `false` if other is nonempty but this box is empty.

### `impl<T, U> Box2D<T, U> where T: Copy + PartialOrd,`

#### `to_non_empty`

```rust
pub fn to_non_empty(&self) -> Option<Box2D<T, U>>
```

#### `intersection`

```rust
pub fn intersection(&self, other: &Box2D<T, U>) -> Option<Box2D<T, U>>
```

Computes the intersection of two boxes, returning `None` if the boxes do not intersect.

#### `intersection_unchecked`

```rust
pub fn intersection_unchecked(&self, other: &Box2D<T, U>) -> Box2D<T, U>
```

Computes the intersection of two boxes without check whether they do intersect.

The result is a negative box if the boxes do not intersect. This can be useful for computing the intersection of more than two boxes, as it is possible to chain multiple `intersection_unchecked` calls and check for empty/negative result at the end.

#### `union`

```rust
pub fn union(&self, other: &Box2D<T, U>) -> Box2D<T, U>
```

Computes the union of two boxes.

If either of the boxes is empty, the other one is returned.

### `impl<T, U> Box2D<T, U> where T: Copy + Add<Output = T>,`

#### `translate`

```rust
pub fn translate(&self, by: Vector2D<T, U>) -> Box2D<T, U>
```

Returns the same box, translated by a vector.

### `impl<T, U> Box2D<T, U> where T: Copy + Sub<Output = T>,`

#### `size`

```rust
pub fn size(&self) -> Size2D<T, U>
```

#### `set_size`

```rust
pub fn set_size(&mut self, size: Size2D<T, U>)
```

Change the size of the box by adjusting the max endpoint without modifying the min endpoint.

#### `width`

```rust
pub fn width(&self) -> T
```

#### `height`

```rust
pub fn height(&self) -> T
```

#### `to_rect`

```rust
pub fn to_rect(&self) -> Rect<T, U>
```

### `impl<T, U> Box2D<T, U> where T: Copy + Add<Output = T> + Sub<Output = T>,`

#### `inflate`

```rust
pub fn inflate(&self, width: T, height: T) -> Box2D<T, U>
```

Inflates the box by the specified sizes on each side respectively.

#### `inner_box`

```rust
pub fn inner_box(&self, offsets: SideOffsets2D<T, U>) -> Box2D<T, U>
```

Calculate the size and position of an inner box.

Subtracts the side offsets from all sides. The horizontal, vertical and applicate offsets must not be larger than the original side length.

#### `outer_box`

```rust
pub fn outer_box(&self, offsets: SideOffsets2D<T, U>) -> Box2D<T, U>
```

Calculate the b and position of an outer box.

Add the offsets to all sides. The expanded box is returned.

### `impl<T, U> Box2D<T, U> where T: Copy + Zero + PartialOrd,`

#### `from_points`

```rust
pub fn from_points<I>(points: I) -> Box2D<T, U> where I: IntoIterator, <I as IntoIterator>::Item: Borrow<Point2D<T, U>>,
```

Returns the smallest box enclosing all of the provided points.

The top/bottom/left/right-most points are exactly on the box’s edges. Since [`Box2D::contains`](struct.Box2D.md) excludes points that are on the right-most and bottom-most edges, not all points passed to [`Box2D::from_points`](struct.Box2D.md) are contained in the returned [`Box2D`](struct.Box2D.md) when probed with [`Box2D::contains`](struct.Box2D.md), but are when probed with [`Box2D::contains_inclusive`](struct.Box2D.md).

For example:

```rust
use euclid::default::{Point2D, Box2D};

let a = Point2D::origin();
let b = Point2D::new(1, 2);
let rect = Box2D::from_points([a, b]);

assert_eq!(rect.width(), 1);
assert_eq!(rect.height(), 2);

assert!(rect.contains(a));
assert!(!rect.contains(b));
assert!(rect.contains_inclusive(b));
```

In particular, calling [`Box2D::from_points`](struct.Box2D.md) with a single point results in an empty [`Box2D`](struct.Box2D.md):

```rust
use euclid::default::{Point2D, Box2D};

let a = Point2D::new(1, 0);
let rect = Box2D::from_points([a]);

assert!(rect.is_empty());
assert!(!rect.contains(a));
assert!(rect.contains_inclusive(a));
```

The [`Box2D`](struct.Box2D.md) enclosing no points is also empty:

```rust
use euclid::default::{Box2D, Point2D};

let rect = Box2D::from_points(std::iter::empty::<Point2D<i32>>());
assert!(rect.is_empty());
```

### `impl<T, U> Box2D<T, U> where T: One + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,`

#### `lerp`

```rust
pub fn lerp(&self, other: Box2D<T, U>, t: T) -> Box2D<T, U>
```

Linearly interpolate between this box and another box.

### `impl<T, U> Box2D<T, U> where T: Copy + One + Add<Output = T> + Div<Output = T>,`

#### `center`

```rust
pub fn center(&self) -> Point2D<T, U>
```

### `impl<T, U> Box2D<T, U> where T: Copy + Mul<Output = T> + Sub<Output = T>,`

#### `area`

```rust
pub fn area(&self) -> T
```

### `impl<T, U> Box2D<T, U> where T: Zero,`

#### `zero`

```rust
pub fn zero() -> Box2D<T, U>
```

Constructor, setting all sides to zero.

### `impl<T, U> Box2D<T, U> where T: Copy,`

#### `x_range`

```rust
pub fn x_range(&self) -> Range<T>
```

#### `y_range`

```rust
pub fn y_range(&self) -> Range<T>
```

#### `to_untyped`

```rust
pub fn to_untyped(&self) -> Box2D<T, UnknownUnit>
```

Drop the units, preserving only the numeric value.

#### `from_untyped`

```rust
pub fn from_untyped(c: &Box2D<T, UnknownUnit>) -> Box2D<T, U>
```

Tag a unitless value with units.

#### `cast_unit`

```rust
pub fn cast_unit<V>(&self) -> Box2D<T, V>
```

Cast the unit

#### `scale`

```rust
pub fn scale<S>(&self, x: S, y: S) -> Box2D<T, U> where S: Copy, T: Mul<S, Output = T>,
```

### `impl<T, U> Box2D<T, U> where T: NumCast + Copy,`

#### `cast`

```rust
pub fn cast<NewT>(&self) -> Box2D<NewT, U> where NewT: NumCast,
```

Cast from one numeric representation to another, preserving the units.

When casting from floating point to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using [`round`](struct.Box2D.md), [`round_in`](struct.Box2D.md) or [`round_out`](struct.Box2D.md) before casting.

#### `try_cast`

```rust
pub fn try_cast<NewT>(&self) -> Option<Box2D<NewT, U>> where NewT: NumCast,
```

Fallible cast from one numeric representation to another, preserving the units.

When casting from floating point to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using [`round`](struct.Box2D.md), [`round_in`](struct.Box2D.md) or [`round_out`](struct.Box2D.md) before casting.

#### `to_f32`

```rust
pub fn to_f32(&self) -> Box2D<f32, U>
```

Cast into an `f32` box.

#### `to_f64`

```rust
pub fn to_f64(&self) -> Box2D<f64, U>
```

Cast into an `f64` box.

#### `to_usize`

```rust
pub fn to_usize(&self) -> Box2D<usize, U>
```

Cast into an `usize` box, truncating decimals if any.

When casting from floating point boxes, it is worth considering whether to `round()`, `round_in()` or `round_out()` before the cast in order to obtain the desired conversion behavior.

#### `to_u32`

```rust
pub fn to_u32(&self) -> Box2D<u32, U>
```

Cast into an `u32` box, truncating decimals if any.

When casting from floating point boxes, it is worth considering whether to `round()`, `round_in()` or `round_out()` before the cast in order to obtain the desired conversion behavior.

#### `to_i32`

```rust
pub fn to_i32(&self) -> Box2D<i32, U>
```

Cast into an `i32` box, truncating decimals if any.

When casting from floating point boxes, it is worth considering whether to `round()`, `round_in()` or `round_out()` before the cast in order to obtain the desired conversion behavior.

#### `to_i64`

```rust
pub fn to_i64(&self) -> Box2D<i64, U>
```

Cast into an `i64` box, truncating decimals if any.

When casting from floating point boxes, it is worth considering whether to `round()`, `round_in()` or `round_out()` before the cast in order to obtain the desired conversion behavior.

### `impl<T, U> Box2D<T, U> where T: Round,`

#### `round`

```rust
pub fn round(&self) -> Box2D<T, U>
```

Return a box with edges rounded to integer coordinates, such that the returned box has the same set of pixel centers as the original one. Values equal to 0.5 round up. Suitable for most places where integral device coordinates are needed, but note that any translation should be applied first to avoid pixel rounding errors. Note that this is *not* rounding to nearest integer if the values are negative. They are always rounding as floor(n + 0.5).

### `impl<T, U> Box2D<T, U> where T: Floor + Ceil,`

#### `round_in`

```rust
pub fn round_in(&self) -> Box2D<T, U>
```

Return a box with faces/edges rounded to integer coordinates, such that the original box contains the resulting box.

#### `round_out`

```rust
pub fn round_out(&self) -> Box2D<T, U>
```

Return a box with faces/edges rounded to integer coordinates, such that the original box is contained in the resulting box.

### `impl<T, U> Box2D<T, U> where T: Float,`

#### `is_finite`

```rust
pub fn is_finite(self) -> bool
```

Returns `true` if all members are finite.

## Trait Implementations

### `impl<T, U> Clone for Box2D<T, U> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Box2D<T, U>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, U> Debug for Box2D<T, U> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, U> Default for Box2D<T, U> where T: Default,`

#### `default`

```rust
fn default() -> Box2D<T, U>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

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

### `impl<T, U> Div<T> for Box2D<T, U> where T: Copy + Div,`

#### `Output`

```rust
type Output = Box2D<<T as Div>::Output, U>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, scale: T) -> <Box2D<T, U> as Div<T>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U> DivAssign<Scale<T, U, U>> for Box2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<T> for Box2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, scale: T)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> From<Box2D<T, U>> for Rect<T, U> where T: Copy + Sub<Output = T>,`

#### `from`

```rust
fn from(b: Box2D<T, U>) -> Rect<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<Rect<T, U>> for Box2D<T, U> where T: Copy + Add<Output = T>,`

#### `from`

```rust
fn from(r: Rect<T, U>) -> Box2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> From<Size2D<T, U>> for Box2D<T, U> where T: Copy + Zero + PartialOrd,`

#### `from`

```rust
fn from(b: Size2D<T, U>) -> Box2D<T, U>
```

Converts to this type from the input type.

### `impl<T, U> Hash for Box2D<T, U> where T: Hash,`

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

### `impl<T, U> Mul<T> for Box2D<T, U> where T: Copy + Mul,`

#### `Output`

```rust
type Output = Box2D<<T as Mul>::Output, U>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: T) -> <Box2D<T, U> as Mul<T>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U> MulAssign<Scale<T, U, U>> for Box2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<T> for Box2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, scale: T)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> PartialEq for Box2D<T, U> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Box2D<T, U>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, U> Copy for Box2D<T, U> where T: Copy,`

### `impl<T, U> Eq for Box2D<T, U> where T: Eq,`

## Auto Trait Implementations

- `impl<T, U> Freeze for Box2D<T, U> where T: Freeze,`
- `impl<T, U> RefUnwindSafe for Box2D<T, U> where T: RefUnwindSafe, U: RefUnwindSafe,`
- `impl<T, U> Send for Box2D<T, U> where T: Send, U: Send,`
- `impl<T, U> Sync for Box2D<T, U> where T: Sync, U: Sync,`
- `impl<T, U> Unpin for Box2D<T, U> where T: Unpin, U: Unpin,`
- `impl<T, U> UnsafeUnpin for Box2D<T, U> where T: UnsafeUnpin,`
- `impl<T, U> UnwindSafe for Box2D<T, U> where T: UnwindSafe, U: UnwindSafe,`
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
