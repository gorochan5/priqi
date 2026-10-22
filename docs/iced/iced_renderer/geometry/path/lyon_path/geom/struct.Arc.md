---
title: "Arc"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::Arc"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom"
file: "iced_renderer/geometry/path/lyon_path/geom/struct.Arc.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/struct.Arc.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::Arc`

```rust
pub struct Arc<S> {
    pub center: Point2D<S, UnknownUnit>,
    pub radii: Vector2D<S, UnknownUnit>,
    pub start_angle: Angle<S>,
    pub sweep_angle: Angle<S>,
    pub x_rotation: Angle<S>,
}
```

An elliptic arc curve segment.

## Fields

### `center: Point2D<S, UnknownUnit>`

### `radii: Vector2D<S, UnknownUnit>`

### `start_angle: Angle<S>`

### `sweep_angle: Angle<S>`

### `x_rotation: Angle<S>`

## Methods

### `impl<S> Arc<S> where S: Scalar,`

#### `cast`

```rust
pub fn cast<NewS>(self) -> Arc<NewS> where NewS: NumCast,
```

#### `circle`

```rust
pub fn circle(center: Point2D<S, UnknownUnit>, radius: S) -> Arc<S>
```

Create simple circle.

#### `from_svg_arc`

```rust
pub fn from_svg_arc(arc: &SvgArc<S>) -> Arc<S>
```

Convert from the SVG arc notation.

#### `to_svg_arc`

```rust
pub fn to_svg_arc(&self) -> SvgArc<S>
```

Convert to the SVG arc notation.

#### `for_each_quadratic_bezier`

```rust
pub fn for_each_quadratic_bezier<F>(&self, cb: &mut F) where F: FnMut(&QuadraticBezierSegment<S>),
```

Approximate the arc with a sequence of quadratic bézier curves.

#### `for_each_quadratic_bezier_with_t`

```rust
pub fn for_each_quadratic_bezier_with_t<F>(&self, cb: &mut F) where F: FnMut(&QuadraticBezierSegment<S>, Range<S>),
```

Approximate the arc with a sequence of quadratic bézier curves.

#### `for_each_cubic_bezier`

```rust
pub fn for_each_cubic_bezier<F>(&self, cb: &mut F) where F: FnMut(&CubicBezierSegment<S>),
```

Approximate the arc with a sequence of cubic bézier curves.

#### `sample`

```rust
pub fn sample(&self, t: S) -> Point2D<S, UnknownUnit>
```

Sample the curve at t (expecting t between 0 and 1).

#### `x`

```rust
pub fn x(&self, t: S) -> S
```

#### `y`

```rust
pub fn y(&self, t: S) -> S
```

#### `sample_tangent`

```rust
pub fn sample_tangent(&self, t: S) -> Vector2D<S, UnknownUnit>
```

Sample the curve’s tangent at t (expecting t between 0 and 1).

#### `get_angle`

```rust
pub fn get_angle(&self, t: S) -> Angle<S>
```

Sample the curve’s angle at t (expecting t between 0 and 1).

#### `end_angle`

```rust
pub fn end_angle(&self) -> Angle<S>
```

#### `from`

```rust
pub fn from(&self) -> Point2D<S, UnknownUnit>
```

#### `to`

```rust
pub fn to(&self) -> Point2D<S, UnknownUnit>
```

#### `split_range`

```rust
pub fn split_range(&self, t_range: Range<S>) -> Arc<S>
```

Return the sub-curve inside a given range of t.

This is equivalent splitting at the range’s end points.

#### `split`

```rust
pub fn split(&self, t: S) -> (Arc<S>, Arc<S>)
```

Split this curve into two sub-curves.

#### `before_split`

```rust
pub fn before_split(&self, t: S) -> Arc<S>
```

Return the curve before the split point.

#### `after_split`

```rust
pub fn after_split(&self, t: S) -> Arc<S>
```

Return the curve after the split point.

#### `flip`

```rust
pub fn flip(&self) -> Arc<S>
```

Swap the direction of the segment.

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

#### `flattened`

```rust
pub fn flattened(&self, tolerance: S) -> Flattened<S> ⓘ
```

Returns the flattened representation of the curve as an iterator, starting *after* the current point.

#### `fast_bounding_box`

```rust
pub fn fast_bounding_box(&self) -> Box2D<S, UnknownUnit>
```

Returns a conservative rectangle that contains the curve.

#### `bounding_box`

```rust
pub fn bounding_box(&self) -> Box2D<S, UnknownUnit>
```

Returns a conservative rectangle that contains the curve.

#### `for_each_local_x_extremum_t`

```rust
pub fn for_each_local_x_extremum_t<F>(&self, cb: &mut F) where F: FnMut(S),
```

#### `for_each_local_y_extremum_t`

```rust
pub fn for_each_local_y_extremum_t<F>(&self, cb: &mut F) where F: FnMut(S),
```

#### `bounding_range_x`

```rust
pub fn bounding_range_x(&self) -> (S, S)
```

#### `bounding_range_y`

```rust
pub fn bounding_range_y(&self) -> (S, S)
```

#### `fast_bounding_range_x`

```rust
pub fn fast_bounding_range_x(&self) -> (S, S)
```

#### `fast_bounding_range_y`

```rust
pub fn fast_bounding_range_y(&self) -> (S, S)
```

#### `approximate_length`

```rust
pub fn approximate_length(&self, tolerance: S) -> S
```

## Trait Implementations

### `impl<S> Clone for Arc<S> where S: Clone,`

#### `clone`

```rust
fn clone(&self) -> Arc<S>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<S> Debug for Arc<S> where S: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<S> From<SvgArc<S>> for Arc<S> where S: Scalar,`

#### `from`

```rust
fn from(svg: SvgArc<S>) -> Arc<S>
```

Converts to this type from the input type.

### `impl<S> PartialEq for Arc<S> where S: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Arc<S>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<S> Segment for Arc<S> where S: Scalar,`

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

#### `split`

```rust
fn split(&self, t: S) -> (Arc<S>, Arc<S>)
```

Split this curve into two sub-curves.

#### `before_split`

```rust
fn before_split(&self, t: S) -> Arc<S>
```

Return the curve before the split point.

#### `after_split`

```rust
fn after_split(&self, t: S) -> Arc<S>
```

Return the curve after the split point.

#### `split_range`

```rust
fn split_range(&self, t_range: Range<S>) -> Arc<S>
```

Return the curve inside a given range of t. [Read more](trait.Segment.md)

#### `flip`

```rust
fn flip(&self) -> Arc<S>
```

Swap the direction of the segment.

#### `approximate_length`

```rust
fn approximate_length(&self, tolerance: S) -> S
```

Compute the length of the segment using a flattened approximation.

#### `for_each_flattened_with_t`

```rust
fn for_each_flattened_with_t( &self, tolerance: <Arc<S> as Segment>::Scalar, callback: &mut dyn FnMut(&LineSegment<S>, Range<S>), )
```

Approximates the curve with sequence of line segments. [Read more](trait.Segment.md)

#### `dx`

```rust
fn dx(&self, t: Self::Scalar) -> Self::Scalar
```

Sample x derivative at t (expecting t between 0 and 1).

#### `dy`

```rust
fn dy(&self, t: Self::Scalar) -> Self::Scalar
```

Sample y derivative at t (expecting t between 0 and 1).

### `impl<S> Copy for Arc<S> where S: Copy,`

### `impl<S> StructuralPartialEq for Arc<S>`

## Auto Trait Implementations

- `impl<S> Freeze for Arc<S> where S: Freeze,`
- `impl<S> RefUnwindSafe for Arc<S> where S: RefUnwindSafe,`
- `impl<S> Send for Arc<S> where S: Send,`
- `impl<S> Sync for Arc<S> where S: Sync,`
- `impl<S> Unpin for Arc<S> where S: Unpin,`
- `impl<S> UnsafeUnpin for Arc<S> where S: UnsafeUnpin,`
- `impl<S> UnwindSafe for Arc<S> where S: UnwindSafe,`
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
