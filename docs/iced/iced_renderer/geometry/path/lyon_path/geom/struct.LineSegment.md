---
title: "LineSegment"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::LineSegment"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom"
file: "iced_renderer/geometry/path/lyon_path/geom/struct.LineSegment.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/struct.LineSegment.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::LineSegment`

```rust
pub struct LineSegment<S> {
    pub from: Point2D<S, UnknownUnit>,
    pub to: Point2D<S, UnknownUnit>,
}
```

A linear segment.

## Fields

### `from: Point2D<S, UnknownUnit>`

### `to: Point2D<S, UnknownUnit>`

## Methods

### `impl<S> LineSegment<S> where S: Scalar,`

#### `sample`

```rust
pub fn sample(&self, t: S) -> Point2D<S, UnknownUnit>
```

Sample the segment at t (expecting t between 0 and 1).

#### `x`

```rust
pub fn x(&self, t: S) -> S
```

Sample the x coordinate of the segment at t (expecting t between 0 and 1).

#### `y`

```rust
pub fn y(&self, t: S) -> S
```

Sample the y coordinate of the segment at t (expecting t between 0 and 1).

#### `from`

```rust
pub fn from(&self) -> Point2D<S, UnknownUnit>
```

#### `to`

```rust
pub fn to(&self) -> Point2D<S, UnknownUnit>
```

#### `solve_t_for_x`

```rust
pub fn solve_t_for_x(&self, x: S) -> S
```

#### `solve_t_for_y`

```rust
pub fn solve_t_for_y(&self, y: S) -> S
```

#### `solve_y_for_x`

```rust
pub fn solve_y_for_x(&self, x: S) -> S
```

#### `solve_x_for_y`

```rust
pub fn solve_x_for_y(&self, y: S) -> S
```

#### `flip`

```rust
pub fn flip(&self) -> LineSegment<S>
```

Returns an inverted version of this segment where the beginning and the end points are swapped.

#### `split_range`

```rust
pub fn split_range(&self, t_range: Range<S>) -> LineSegment<S>
```

Return the sub-segment inside a given range of t.

This is equivalent splitting at the range’s end points.

#### `split`

```rust
pub fn split(&self, t: S) -> (LineSegment<S>, LineSegment<S>)
```

Split this curve into two sub-segments.

#### `before_split`

```rust
pub fn before_split(&self, t: S) -> LineSegment<S>
```

Return the segment before the split point.

#### `after_split`

```rust
pub fn after_split(&self, t: S) -> LineSegment<S>
```

Return the segment after the split point.

#### `split_at_x`

```rust
pub fn split_at_x(&self, x: S) -> (LineSegment<S>, LineSegment<S>)
```

#### `bounding_box`

```rust
pub fn bounding_box(&self) -> Box2D<S, UnknownUnit>
```

Return the smallest rectangle containing this segment.

#### `to_vector`

```rust
pub fn to_vector(&self) -> Vector2D<S, UnknownUnit>
```

Returns the vector between this segment’s `from` and `to` points.

#### `to_line`

```rust
pub fn to_line(&self) -> Line<S>
```

Returns the line containing this segment.

#### `length`

```rust
pub fn length(&self) -> S
```

Computes the length of this segment.

#### `square_length`

```rust
pub fn square_length(&self) -> S
```

Computes the squared length of this segment.

#### `set_length`

```rust
pub fn set_length(&mut self, new_length: S)
```

Changes the segment’s length, moving destination point.

#### `mid_point`

```rust
pub fn mid_point(&self) -> Point2D<S, UnknownUnit>
```

Computes third mid-point of this segment.

#### `translate`

```rust
pub fn translate(&self, by: Vector2D<S, UnknownUnit>) -> LineSegment<S>
```

#### `transformed`

```rust
pub fn transformed<T>(&self, transform: &T) -> LineSegment<S> where T: Transformation<S>,
```

Applies the transform to this segment and returns the results.

#### `intersection_t`

```rust
pub fn intersection_t(&self, other: &LineSegment<S>) -> Option<(S, S)>
```

Computes the intersection (if any) between this segment and another one.

The result is provided in the form of the `t` parameter of each segment. To get the intersection point, sample one of the segments at the corresponding value.

#### `intersection`

```rust
pub fn intersection( &self, other: &LineSegment<S>, ) -> Option<Point2D<S, UnknownUnit>>
```

#### `line_intersection_t`

```rust
pub fn line_intersection_t(&self, line: &Line<S>) -> Option<S>
```

#### `line_intersection`

```rust
pub fn line_intersection( &self, line: &Line<S>, ) -> Option<Point2D<S, UnknownUnit>>
```

#### `horizontal_line_intersection_t`

```rust
pub fn horizontal_line_intersection_t(&self, y: S) -> Option<S>
```

#### `vertical_line_intersection_t`

```rust
pub fn vertical_line_intersection_t(&self, x: S) -> Option<S>
```

#### `horizontal_line_intersection`

```rust
pub fn horizontal_line_intersection( &self, y: S, ) -> Option<Point2D<S, UnknownUnit>>
```

#### `vertical_line_intersection`

```rust
pub fn vertical_line_intersection( &self, x: S, ) -> Option<Point2D<S, UnknownUnit>>
```

#### `intersects`

```rust
pub fn intersects(&self, other: &LineSegment<S>) -> bool
```

#### `intersects_line`

```rust
pub fn intersects_line(&self, line: &Line<S>) -> bool
```

#### `overlaps_line`

```rust
pub fn overlaps_line(&self, line: &Line<S>) -> bool
```

#### `overlaps_segment`

```rust
pub fn overlaps_segment(&self, other: &LineSegment<S>) -> bool
```

#### `contains_segment`

```rust
pub fn contains_segment(&self, other: &LineSegment<S>) -> bool
```

#### `clipped_x`

```rust
pub fn clipped_x(&self, clip: Range<S>) -> Option<LineSegment<S>>
```

Horizontally clip this segment against a range of the x axis.

#### `clipped_y`

```rust
pub fn clipped_y(&self, clip: Range<S>) -> Option<LineSegment<S>>
```

Vertically clip this segment against a range of the y axis.

#### `clipped`

```rust
pub fn clipped(&self, clip: &Box2D<S, UnknownUnit>) -> Option<LineSegment<S>>
```

Clip this segment against a rectangle.

#### `distance_to_point`

```rust
pub fn distance_to_point(&self, p: Point2D<S, UnknownUnit>) -> S
```

Computes the distance between this segment and a point.

#### `square_distance_to_point`

```rust
pub fn square_distance_to_point(&self, p: Point2D<S, UnknownUnit>) -> S
```

Computes the squared distance between this segment and a point.

Can be useful to save a square root and a division when comparing against a distance that can be squared.

#### `closest_point`

```rust
pub fn closest_point( &self, p: Point2D<S, UnknownUnit>, ) -> Point2D<S, UnknownUnit>
```

Computes the closest point on this segment to `p`.

#### `to_f32`

```rust
pub fn to_f32(&self) -> LineSegment<f32>
```

#### `to_f64`

```rust
pub fn to_f64(&self) -> LineSegment<f64>
```

## Trait Implementations

### `impl<S> Clone for LineSegment<S> where S: Clone,`

#### `clone`

```rust
fn clone(&self) -> LineSegment<S>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<S> Debug for LineSegment<S> where S: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<S> PartialEq for LineSegment<S> where S: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &LineSegment<S>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<S> Segment for LineSegment<S> where S: Scalar,`

#### `Scalar`

```rust
type Scalar = S
```

#### `from`

```rust
fn from(&self) -> Point2D<S, UnknownUnit>
```

Start of the curve.

#### `to`

```rust
fn to(&self) -> Point2D<S, UnknownUnit>
```

End of the curve.

#### `sample`

```rust
fn sample(&self, t: S) -> Point2D<S, UnknownUnit>
```

Sample the curve at t (expecting t between 0 and 1).

#### `x`

```rust
fn x(&self, t: S) -> S
```

Sample x at t (expecting t between 0 and 1).

#### `y`

```rust
fn y(&self, t: S) -> S
```

Sample y at t (expecting t between 0 and 1).

#### `derivative`

```rust
fn derivative(&self, _t: S) -> Vector2D<S, UnknownUnit>
```

Sample the derivative at t (expecting t between 0 and 1).

#### `dx`

```rust
fn dx(&self, _t: S) -> S
```

Sample x derivative at t (expecting t between 0 and 1).

#### `dy`

```rust
fn dy(&self, _t: S) -> S
```

Sample y derivative at t (expecting t between 0 and 1).

#### `split`

```rust
fn split(&self, t: S) -> (LineSegment<S>, LineSegment<S>)
```

Split this curve into two sub-curves.

#### `before_split`

```rust
fn before_split(&self, t: S) -> LineSegment<S>
```

Return the curve before the split point.

#### `after_split`

```rust
fn after_split(&self, t: S) -> LineSegment<S>
```

Return the curve after the split point.

#### `split_range`

```rust
fn split_range(&self, t_range: Range<S>) -> LineSegment<S>
```

Return the curve inside a given range of t. [Read more](trait.Segment.md)

#### `flip`

```rust
fn flip(&self) -> LineSegment<S>
```

Swap the direction of the segment.

#### `approximate_length`

```rust
fn approximate_length(&self, _tolerance: S) -> S
```

Compute the length of the segment using a flattened approximation.

#### `for_each_flattened_with_t`

```rust
fn for_each_flattened_with_t( &self, _tolerance: <LineSegment<S> as Segment>::Scalar, callback: &mut dyn FnMut(&LineSegment<S>, Range<S>), )
```

Approximates the curve with sequence of line segments. [Read more](trait.Segment.md)

### `impl<S> Copy for LineSegment<S> where S: Copy,`

### `impl<S> StructuralPartialEq for LineSegment<S>`

## Auto Trait Implementations

- `impl<S> Freeze for LineSegment<S> where S: Freeze,`
- `impl<S> RefUnwindSafe for LineSegment<S> where S: RefUnwindSafe,`
- `impl<S> Send for LineSegment<S> where S: Send,`
- `impl<S> Sync for LineSegment<S> where S: Sync,`
- `impl<S> Unpin for LineSegment<S> where S: Unpin,`
- `impl<S> UnsafeUnpin for LineSegment<S> where S: UnsafeUnpin,`
- `impl<S> UnwindSafe for LineSegment<S> where S: UnwindSafe,`
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
