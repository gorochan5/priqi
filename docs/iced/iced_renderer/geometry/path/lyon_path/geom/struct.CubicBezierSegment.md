---
title: "CubicBezierSegment"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::CubicBezierSegment"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom"
file: "iced_renderer/geometry/path/lyon_path/geom/struct.CubicBezierSegment.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/struct.CubicBezierSegment.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::CubicBezierSegment`

```rust
pub struct CubicBezierSegment<S> {
    pub from: Point2D<S, UnknownUnit>,
    pub ctrl1: Point2D<S, UnknownUnit>,
    pub ctrl2: Point2D<S, UnknownUnit>,
    pub to: Point2D<S, UnknownUnit>,
}
```

A 2d curve segment defined by four points: the beginning of the segment, two control points and the end of the segment.

The curve is defined by equation:² `∀ t ∈ [0..1], P(t) = (1 - t)³ * from + 3 * (1 - t)² * t * ctrl1 + 3 * t² * (1 - t) * ctrl2 + t³ * to`

## Fields

### `from: Point2D<S, UnknownUnit>`

### `ctrl1: Point2D<S, UnknownUnit>`

### `ctrl2: Point2D<S, UnknownUnit>`

### `to: Point2D<S, UnknownUnit>`

## Methods

### `impl<S> CubicBezierSegment<S> where S: Scalar,`

#### `sample`

```rust
pub fn sample(&self, t: S) -> Point2D<S, UnknownUnit>
```

Sample the curve at t (expecting t between 0 and 1).

#### `x`

```rust
pub fn x(&self, t: S) -> S
```

Sample the x coordinate of the curve at t (expecting t between 0 and 1).

#### `y`

```rust
pub fn y(&self, t: S) -> S
```

Sample the y coordinate of the curve at t (expecting t between 0 and 1).

#### `solve_t_for_x`

```rust
pub fn solve_t_for_x(&self, x: S) -> ArrayVec<S, 3>
```

Return the parameter values corresponding to a given x coordinate.

#### `solve_t_for_y`

```rust
pub fn solve_t_for_y(&self, y: S) -> ArrayVec<S, 3>
```

Return the parameter values corresponding to a given y coordinate.

#### `derivative`

```rust
pub fn derivative(&self, t: S) -> Vector2D<S, UnknownUnit>
```

Sample the curve’s derivative at t (expecting t between 0 and 1).

#### `dx`

```rust
pub fn dx(&self, t: S) -> S
```

Sample the x coordinate of the curve’s derivative at t (expecting t between 0 and 1).

#### `dy`

```rust
pub fn dy(&self, t: S) -> S
```

Sample the y coordinate of the curve’s derivative at t (expecting t between 0 and 1).

#### `split_range`

```rust
pub fn split_range(&self, t_range: Range<S>) -> CubicBezierSegment<S>
```

Return the sub-curve inside a given range of t.

This is equivalent to splitting at the range’s end points.

#### `split`

```rust
pub fn split(&self, t: S) -> (CubicBezierSegment<S>, CubicBezierSegment<S>)
```

Split this curve into two sub-curves.

#### `before_split`

```rust
pub fn before_split(&self, t: S) -> CubicBezierSegment<S>
```

Return the curve before the split point.

#### `after_split`

```rust
pub fn after_split(&self, t: S) -> CubicBezierSegment<S>
```

Return the curve after the split point.

#### `baseline`

```rust
pub fn baseline(&self) -> LineSegment<S>
```

#### `is_linear`

```rust
pub fn is_linear(&self, tolerance: S) -> bool
```

Returns true if the curve can be approximated with a single line segment, given a tolerance threshold.

#### `fat_line`

```rust
pub fn fat_line(&self) -> (LineEquation<S>, LineEquation<S>)
```

Computes a “fat line” of this segment.

A fat line is two conservative lines between which the segment is fully contained.

#### `transformed`

```rust
pub fn transformed<T>(&self, transform: &T) -> CubicBezierSegment<S> where T: Transformation<S>,
```

Applies the transform to this curve and returns the results.

#### `flip`

```rust
pub fn flip(&self) -> CubicBezierSegment<S>
```

Swap the beginning and the end of the segment.

#### `to_quadratic`

```rust
pub fn to_quadratic(&self) -> QuadraticBezierSegment<S>
```

Approximate the curve with a single quadratic bézier segment.

This is terrible as a general approximation but works if the cubic curve does not have inflection points and is “flat” enough. Typically usable after subdividing the curve a few times.

#### `to_quadratic_error`

```rust
pub fn to_quadratic_error(&self) -> S
```

Evaluates an upper bound on the maximum distance between the curve and its quadratic approximation obtained using `to_quadratic`.

#### `is_quadratic`

```rust
pub fn is_quadratic(&self, tolerance: S) -> bool
```

Returns true if the curve can be safely approximated with a single quadratic bézier segment given the provided tolerance threshold.

Equivalent to comparing `to_quadratic_error` with the tolerance threshold, avoiding the cost of two square roots.

#### `num_quadratics`

```rust
pub fn num_quadratics(&self, tolerance: S) -> u32
```

Computes the number of quadratic bézier segments required to approximate this cubic curve given a tolerance threshold.

Derived by Raph Levien from section 10.6 of Sedeberg’s CAGD notes [https://scholarsarchive.byu.edu/cgi/viewcontent.cgi?article=1000&context=facpub#section.10.6](https://scholarsarchive.byu.edu/cgi/viewcontent.cgi?article=1000&context=facpub#section.10.6) and the error metric from the caffein owl blog post [http://caffeineowl.com/graphics/2d/vectorial/cubic2quad01.html](http://caffeineowl.com/graphics/2d/vectorial/cubic2quad01.html)

#### `flattened`

```rust
pub fn flattened(&self, tolerance: S) -> Flattened<S> ⓘ
```

Returns the flattened representation of the curve as an iterator, starting *after* the current point.

#### `for_each_monotonic_range`

```rust
pub fn for_each_monotonic_range<F>(&self, cb: &mut F) where F: FnMut(Range<S>),
```

Invokes a callback for each monotonic part of the segment.

#### `for_each_monotonic`

```rust
pub fn for_each_monotonic<F>(&self, cb: &mut F) where F: FnMut(&CubicBezierSegment<S>),
```

Invokes a callback for each monotonic part of the segment.

#### `for_each_y_monotonic_range`

```rust
pub fn for_each_y_monotonic_range<F>(&self, cb: &mut F) where F: FnMut(Range<S>),
```

Invokes a callback for each y-monotonic part of the segment.

#### `for_each_y_monotonic`

```rust
pub fn for_each_y_monotonic<F>(&self, cb: &mut F) where F: FnMut(&CubicBezierSegment<S>),
```

Invokes a callback for each y-monotonic part of the segment.

#### `for_each_x_monotonic_range`

```rust
pub fn for_each_x_monotonic_range<F>(&self, cb: &mut F) where F: FnMut(Range<S>),
```

Invokes a callback for each x-monotonic part of the segment.

#### `for_each_x_monotonic`

```rust
pub fn for_each_x_monotonic<F>(&self, cb: &mut F) where F: FnMut(&CubicBezierSegment<S>),
```

Invokes a callback for each x-monotonic part of the segment.

#### `for_each_quadratic_bezier`

```rust
pub fn for_each_quadratic_bezier<F>(&self, tolerance: S, cb: &mut F) where F: FnMut(&QuadraticBezierSegment<S>),
```

Approximates the cubic bézier curve with sequence of quadratic ones, invoking a callback at each step.

#### `for_each_quadratic_bezier_with_t`

```rust
pub fn for_each_quadratic_bezier_with_t<F>(&self, tolerance: S, cb: &mut F) where F: FnMut(&QuadraticBezierSegment<S>, Range<S>),
```

Approximates the cubic bézier curve with sequence of quadratic ones, invoking a callback at each step.

#### `for_each_flattened`

```rust
pub fn for_each_flattened<F>(&self, tolerance: S, callback: &mut F) where F: FnMut(&LineSegment<S>),
```

Approximates the curve with sequence of line segments.

The `tolerance` parameter defines the maximum distance between the curve and its approximation.

#### `for_each_flattened_with_t`

```rust
pub fn for_each_flattened_with_t<F>(&self, tolerance: S, callback: &mut F) where F: FnMut(&LineSegment<S>, Range<S>),
```

Approximates the curve with sequence of line segments.

The `tolerance` parameter defines the maximum distance between the curve and its approximation.

The end of the t parameter range at the final segment is guaranteed to be equal to `1.0`.

#### `approximate_length`

```rust
pub fn approximate_length(&self, tolerance: S) -> S
```

Compute the length of the segment using a flattened approximation.

#### `for_each_inflection_t`

```rust
pub fn for_each_inflection_t<F>(&self, cb: &mut F) where F: FnMut(S),
```

Invokes a callback at each inflection point if any.

#### `for_each_local_x_extremum_t`

```rust
pub fn for_each_local_x_extremum_t<F>(&self, cb: &mut F) where F: FnMut(S),
```

Return local x extrema or None if this curve is monotonic.

This returns the advancements along the curve, not the actual x position.

#### `for_each_local_y_extremum_t`

```rust
pub fn for_each_local_y_extremum_t<F>(&self, cb: &mut F) where F: FnMut(S),
```

Return local y extrema or None if this curve is monotonic.

This returns the advancements along the curve, not the actual y position.

#### `y_maximum_t`

```rust
pub fn y_maximum_t(&self) -> S
```

Find the advancement of the y-most position in the curve.

This returns the advancement along the curve, not the actual y position.

#### `y_minimum_t`

```rust
pub fn y_minimum_t(&self) -> S
```

Find the advancement of the y-least position in the curve.

This returns the advancement along the curve, not the actual y position.

#### `x_maximum_t`

```rust
pub fn x_maximum_t(&self) -> S
```

Find the advancement of the x-most position in the curve.

This returns the advancement along the curve, not the actual x position.

#### `x_minimum_t`

```rust
pub fn x_minimum_t(&self) -> S
```

Find the x-least position in the curve.

#### `fast_bounding_box`

```rust
pub fn fast_bounding_box(&self) -> Box2D<S, UnknownUnit>
```

Returns a conservative rectangle the curve is contained in.

This method is faster than `bounding_box` but more conservative.

#### `fast_bounding_range_x`

```rust
pub fn fast_bounding_range_x(&self) -> (S, S)
```

Returns a conservative range of x that contains this curve.

#### `fast_bounding_range_y`

```rust
pub fn fast_bounding_range_y(&self) -> (S, S)
```

Returns a conservative range of y that contains this curve.

#### `bounding_box`

```rust
pub fn bounding_box(&self) -> Box2D<S, UnknownUnit>
```

Returns a conservative rectangle that contains the curve.

#### `bounding_range_x`

```rust
pub fn bounding_range_x(&self) -> (S, S)
```

Returns the smallest range of x that contains this curve.

#### `bounding_range_y`

```rust
pub fn bounding_range_y(&self) -> (S, S)
```

Returns the smallest range of y that contains this curve.

#### `is_x_monotonic`

```rust
pub fn is_x_monotonic(&self) -> bool
```

Returns whether this segment is monotonic on the x axis.

#### `is_y_monotonic`

```rust
pub fn is_y_monotonic(&self) -> bool
```

Returns whether this segment is monotonic on the y axis.

#### `is_monotonic`

```rust
pub fn is_monotonic(&self) -> bool
```

Returns whether this segment is fully monotonic.

#### `cubic_intersections_t`

```rust
pub fn cubic_intersections_t( &self, curve: &CubicBezierSegment<S>, ) -> ArrayVec<(S, S), 9>
```

Computes the intersections (if any) between this segment and another one.

The result is provided in the form of the `t` parameters of each point along the curves. To get the intersection points, sample the curves at the corresponding values.

Returns endpoint intersections where an endpoint intersects the interior of the other curve, but not endpoint/endpoint intersections.

Returns no intersections if either curve is a point.

#### `cubic_intersections`

```rust
pub fn cubic_intersections( &self, curve: &CubicBezierSegment<S>, ) -> ArrayVec<Point2D<S, UnknownUnit>, 9>
```

Computes the intersection points (if any) between this segment and another one.

#### `quadratic_intersections_t`

```rust
pub fn quadratic_intersections_t( &self, curve: &QuadraticBezierSegment<S>, ) -> ArrayVec<(S, S), 9>
```

Computes the intersections (if any) between this segment a quadratic bézier segment.

The result is provided in the form of the `t` parameters of each point along the curves. To get the intersection points, sample the curves at the corresponding values.

Returns endpoint intersections where an endpoint intersects the interior of the other curve, but not endpoint/endpoint intersections.

Returns no intersections if either curve is a point.

#### `quadratic_intersections`

```rust
pub fn quadratic_intersections( &self, curve: &QuadraticBezierSegment<S>, ) -> ArrayVec<Point2D<S, UnknownUnit>, 9>
```

Computes the intersection points (if any) between this segment and a quadratic bézier segment.

#### `line_intersections_t`

```rust
pub fn line_intersections_t(&self, line: &Line<S>) -> ArrayVec<S, 3>
```

Computes the intersections (if any) between this segment and a line.

The result is provided in the form of the `t` parameters of each point along curve. To get the intersection points, sample the curve at the corresponding values.

#### `line_intersections`

```rust
pub fn line_intersections( &self, line: &Line<S>, ) -> ArrayVec<Point2D<S, UnknownUnit>, 3>
```

Computes the intersection points (if any) between this segment and a line.

#### `line_segment_intersections_t`

```rust
pub fn line_segment_intersections_t( &self, segment: &LineSegment<S>, ) -> ArrayVec<(S, S), 3>
```

Computes the intersections (if any) between this segment and a line segment.

The result is provided in the form of the `t` parameters of each point along curve and segment. To get the intersection points, sample the segments at the corresponding values.

#### `from`

```rust
pub fn from(&self) -> Point2D<S, UnknownUnit>
```

#### `to`

```rust
pub fn to(&self) -> Point2D<S, UnknownUnit>
```

#### `line_segment_intersections`

```rust
pub fn line_segment_intersections( &self, segment: &LineSegment<S>, ) -> ArrayVec<Point2D<S, UnknownUnit>, 3>
```

#### `drag`

```rust
pub fn drag( &self, t: S, new_position: Point2D<S, UnknownUnit>, ) -> CubicBezierSegment<S>
```

#### `drag_with_weight`

```rust
pub fn drag_with_weight( &self, t: S, new_position: Point2D<S, UnknownUnit>, weight: S, ) -> CubicBezierSegment<S>
```

#### `to_f32`

```rust
pub fn to_f32(&self) -> CubicBezierSegment<f32>
```

#### `to_f64`

```rust
pub fn to_f64(&self) -> CubicBezierSegment<f64>
```

#### `polynomial_form`

```rust
pub fn polynomial_form(&self) -> CubicBezierPolynomial<S>
```

## Trait Implementations

### `impl<S> Clone for CubicBezierSegment<S> where S: Clone,`

#### `clone`

```rust
fn clone(&self) -> CubicBezierSegment<S>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<S> Debug for CubicBezierSegment<S> where S: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<S> PartialEq for CubicBezierSegment<S> where S: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &CubicBezierSegment<S>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<S> Segment for CubicBezierSegment<S> where S: Scalar,`

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
fn derivative(&self, t: S) -> Vector2D<S, UnknownUnit>
```

Sample the derivative at t (expecting t between 0 and 1).

#### `dx`

```rust
fn dx(&self, t: S) -> S
```

Sample x derivative at t (expecting t between 0 and 1).

#### `dy`

```rust
fn dy(&self, t: S) -> S
```

Sample y derivative at t (expecting t between 0 and 1).

#### `split`

```rust
fn split(&self, t: S) -> (CubicBezierSegment<S>, CubicBezierSegment<S>)
```

Split this curve into two sub-curves.

#### `before_split`

```rust
fn before_split(&self, t: S) -> CubicBezierSegment<S>
```

Return the curve before the split point.

#### `after_split`

```rust
fn after_split(&self, t: S) -> CubicBezierSegment<S>
```

Return the curve after the split point.

#### `split_range`

```rust
fn split_range(&self, t_range: Range<S>) -> CubicBezierSegment<S>
```

Return the curve inside a given range of t. [Read more](trait.Segment.md)

#### `flip`

```rust
fn flip(&self) -> CubicBezierSegment<S>
```

Swap the direction of the segment.

#### `approximate_length`

```rust
fn approximate_length(&self, tolerance: S) -> S
```

Compute the length of the segment using a flattened approximation.

#### `for_each_flattened_with_t`

```rust
fn for_each_flattened_with_t( &self, tolerance: <CubicBezierSegment<S> as Segment>::Scalar, callback: &mut dyn FnMut(&LineSegment<S>, Range<S>), )
```

Approximates the curve with sequence of line segments. [Read more](trait.Segment.md)

### `impl<S> Copy for CubicBezierSegment<S> where S: Copy,`

### `impl<S> StructuralPartialEq for CubicBezierSegment<S>`

## Auto Trait Implementations

- `impl<S> Freeze for CubicBezierSegment<S> where S: Freeze,`
- `impl<S> RefUnwindSafe for CubicBezierSegment<S> where S: RefUnwindSafe,`
- `impl<S> Send for CubicBezierSegment<S> where S: Send,`
- `impl<S> Sync for CubicBezierSegment<S> where S: Sync,`
- `impl<S> Unpin for CubicBezierSegment<S> where S: Unpin,`
- `impl<S> UnsafeUnpin for CubicBezierSegment<S> where S: UnsafeUnpin,`
- `impl<S> UnwindSafe for CubicBezierSegment<S> where S: UnwindSafe,`
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
