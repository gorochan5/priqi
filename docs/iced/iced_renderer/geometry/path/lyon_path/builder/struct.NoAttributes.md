---
title: "NoAttributes"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::builder::NoAttributes"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::builder"
file: "iced_renderer/geometry/path/lyon_path/builder/struct.NoAttributes.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/builder/struct.NoAttributes.html"
---

# `iced_renderer::geometry::path::lyon_path::builder::NoAttributes`

```rust
pub struct NoAttributes<B> where
    B: PathBuilder, { /* private fields */ }
```

A convenience wrapper for `PathBuilder` without custom attributes.

See the [PathBuilder](trait.PathBuilder.md) trait.

This simply forwards to an underlying `PathBuilder` implementation, using no attributes.

## Methods

### `impl<B> NoAttributes<B> where B: PathBuilder,`

#### `wrap`

```rust
pub fn wrap(inner: B) -> NoAttributes<B>
```

#### `new`

```rust
pub fn new() -> NoAttributes<B> where B: Default,
```

#### `with_capacity`

```rust
pub fn with_capacity(endpoints: usize, ctrl_points: usize) -> NoAttributes<B> where B: Default,
```

#### `begin`

```rust
pub fn begin(&mut self, at: Point2D<f32, UnknownUnit>) -> EndpointId
```

Starts a new sub-path at a given position.

There must be no sub-path in progress when this method is called. `at` becomes the current position of the sub-path.

#### `end`

```rust
pub fn end(&mut self, close: bool)
```

Ends the current sub path.

A sub-path must be in progress when this method is called. After this method is called, there is no sub-path in progress until `begin` is called again.

#### `close`

```rust
pub fn close(&mut self)
```

Closes the current sub path.

Shorthand for `builder.end(true)`.

#### `line_to`

```rust
pub fn line_to(&mut self, to: Point2D<f32, UnknownUnit>) -> EndpointId
```

Adds a line segment to the current sub-path.

A sub-path must be in progress when this method is called.

#### `quadratic_bezier_to`

```rust
pub fn quadratic_bezier_to( &mut self, ctrl: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, ) -> EndpointId
```

Adds a quadratic bézier curve to the current sub-path.

A sub-path must be in progress when this method is called.

#### `cubic_bezier_to`

```rust
pub fn cubic_bezier_to( &mut self, ctrl1: Point2D<f32, UnknownUnit>, ctrl2: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, ) -> EndpointId
```

Adds a cubic bézier curve to the current sub-path.

A sub-path must be in progress when this method is called.

#### `reserve`

```rust
pub fn reserve(&mut self, endpoints: usize, ctrl_points: usize)
```

Hints at the builder that a certain number of endpoints and control points will be added.

The Builder implementation may use this information to pre-allocate memory as an optimization.

#### `path_event`

```rust
pub fn path_event( &mut self, event: Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>, )
```

Applies the provided path event.

By default this calls one of `begin`, `end`, `line`, `quadratic_bezier_segment`, or `cubic_bezier_segment` according to the path event.

The requirements for each method apply to the corresponding event.

#### `add_polygon`

```rust
pub fn add_polygon(&mut self, polygon: Polygon<'_, Point2D<f32, UnknownUnit>>)
```

Adds a sub-path from a polygon.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

#### `add_point`

```rust
pub fn add_point(&mut self, at: Point2D<f32, UnknownUnit>) -> EndpointId
```

Adds a sub-path containing a single point.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

#### `add_line_segment`

```rust
pub fn add_line_segment( &mut self, line: &LineSegment<f32>, ) -> (EndpointId, EndpointId)
```

Adds a sub-path containing a single line segment.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

#### `add_ellipse`

```rust
pub fn add_ellipse( &mut self, center: Point2D<f32, UnknownUnit>, radii: Vector2D<f32, UnknownUnit>, x_rotation: Angle<f32>, winding: Winding, )
```

Adds a sub-path containing an ellipse.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

#### `add_circle`

```rust
pub fn add_circle( &mut self, center: Point2D<f32, UnknownUnit>, radius: f32, winding: Winding, )
```

Adds a sub-path containing a circle.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

#### `add_rectangle`

```rust
pub fn add_rectangle( &mut self, rect: &Box2D<f32, UnknownUnit>, winding: Winding, )
```

Adds a sub-path containing a rectangle.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

#### `add_rounded_rectangle`

```rust
pub fn add_rounded_rectangle( &mut self, rect: &Box2D<f32, UnknownUnit>, radii: &BorderRadii, winding: Winding, )
```

Adds a sub-path containing a rectangle.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

#### `flattened`

```rust
pub fn flattened(self, tolerance: f32) -> NoAttributes<Flattened<B>>
```

Returns a builder that approximates all curves with sequences of line segments.

#### `transformed`

```rust
pub fn transformed<Transform>( self, transform: Transform, ) -> NoAttributes<Transformed<B, Transform>> where Transform: Transformation<f32>,
```

Returns a builder that applies the given transformation to all positions.

#### `with_svg`

```rust
pub fn with_svg(self) -> WithSvg<B>
```

Returns a builder that support SVG commands.

This must be called before starting to add any sub-path.

#### `build`

```rust
pub fn build<P>(self) -> P where B: Build<PathType = P>,
```

Builds a path object, consuming the builder.

#### `inner`

```rust
pub fn inner(&self) -> &B
```

#### `inner_mut`

```rust
pub fn inner_mut(&mut self) -> &mut B
```

#### `into_inner`

```rust
pub fn into_inner(self) -> B
```

### `impl NoAttributes<BuilderImpl>`

#### `extend_from_paths`

```rust
pub fn extend_from_paths(&mut self, paths: &[PathSlice<'_>])
```

## Trait Implementations

### `impl<B> Build for NoAttributes<B> where B: PathBuilder + Build,`

#### `PathType`

```rust
type PathType = <B as Build>::PathType
```

The type of object that is created by this builder.

#### `build`

```rust
fn build(self) -> <B as Build>::PathType
```

Builds a path object, consuming the builder.

### `impl<B> Clone for NoAttributes<B> where B: Clone + PathBuilder,`

#### `clone`

```rust
fn clone(&self) -> NoAttributes<B>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<B> Debug for NoAttributes<B> where B: Debug + PathBuilder,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<B> Default for NoAttributes<B> where B: PathBuilder + Default,`

#### `default`

```rust
fn default() -> NoAttributes<B>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<B> Hash for NoAttributes<B> where B: Hash + PathBuilder,`

#### `hash`

```rust
fn hash<__H>(&self, state: &mut __H) where __H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl<B> PartialEq for NoAttributes<B> where B: PartialEq + PathBuilder,`

#### `eq`

```rust
fn eq(&self, other: &NoAttributes<B>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<B> PathBuilder for NoAttributes<B> where B: PathBuilder,`

#### `num_attributes`

```rust
fn num_attributes(&self) -> usize
```

#### `begin`

```rust
fn begin( &mut self, at: Point2D<f32, UnknownUnit>, _attributes: &[f32], ) -> EndpointId
```

Starts a new sub-path at a given position. [Read more](trait.PathBuilder.md)

#### `end`

```rust
fn end(&mut self, close: bool)
```

Ends the current sub path. [Read more](trait.PathBuilder.md)

#### `line_to`

```rust
fn line_to( &mut self, to: Point2D<f32, UnknownUnit>, _attributes: &[f32], ) -> EndpointId
```

Adds a line segment to the current sub-path. [Read more](trait.PathBuilder.md)

#### `quadratic_bezier_to`

```rust
fn quadratic_bezier_to( &mut self, ctrl: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, _attributes: &[f32], ) -> EndpointId
```

Adds a quadratic bézier curve to the current sub-path. [Read more](trait.PathBuilder.md)

#### `cubic_bezier_to`

```rust
fn cubic_bezier_to( &mut self, ctrl1: Point2D<f32, UnknownUnit>, ctrl2: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, _attributes: &[f32], ) -> EndpointId
```

Adds a cubic bézier curve to the current sub-path. [Read more](trait.PathBuilder.md)

#### `reserve`

```rust
fn reserve(&mut self, endpoints: usize, ctrl_points: usize)
```

Hints at the builder that a certain number of endpoints and control points will be added. [Read more](trait.PathBuilder.md)

#### `close`

```rust
fn close(&mut self)
```

Closes the current sub path. [Read more](trait.PathBuilder.md)

#### `path_event`

```rust
fn path_event( &mut self, event: Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>, attributes: &[f32], )
```

Applies the provided path event. [Read more](trait.PathBuilder.md)

#### `event`

```rust
fn event( &mut self, event: Event<(Point2D<f32, UnknownUnit>, &[f32]), Point2D<f32, UnknownUnit>>, )
```

#### `add_polygon`

```rust
fn add_polygon( &mut self, polygon: Polygon<'_, Point2D<f32, UnknownUnit>>, attributes: &[f32], )
```

Adds a sub-path from a polygon. [Read more](trait.PathBuilder.md)

#### `add_point`

```rust
fn add_point( &mut self, at: Point2D<f32, UnknownUnit>, attributes: &[f32], ) -> EndpointId
```

Adds a sub-path containing a single point. [Read more](trait.PathBuilder.md)

#### `add_line_segment`

```rust
fn add_line_segment( &mut self, line: &LineSegment<f32>, attributes: &[f32], ) -> (EndpointId, EndpointId)
```

Adds a sub-path containing a single line segment. [Read more](trait.PathBuilder.md)

#### `add_ellipse`

```rust
fn add_ellipse( &mut self, center: Point2D<f32, UnknownUnit>, radii: Vector2D<f32, UnknownUnit>, x_rotation: Angle<f32>, winding: Winding, attributes: &[f32], )
```

Adds a sub-path containing an ellipse. [Read more](trait.PathBuilder.md)

#### `add_circle`

```rust
fn add_circle( &mut self, center: Point2D<f32, UnknownUnit>, radius: f32, winding: Winding, attributes: &[f32], ) where Self: Sized,
```

Adds a sub-path containing a circle. [Read more](trait.PathBuilder.md)

#### `add_rectangle`

```rust
fn add_rectangle( &mut self, rect: &Box2D<f32, UnknownUnit>, winding: Winding, attributes: &[f32], )
```

Adds a sub-path containing a rectangle. [Read more](trait.PathBuilder.md)

#### `add_rounded_rectangle`

```rust
fn add_rounded_rectangle( &mut self, rect: &Box2D<f32, UnknownUnit>, radii: &BorderRadii, winding: Winding, custom_attributes: &[f32], ) where Self: Sized,
```

Adds a sub-path containing a rectangle. [Read more](trait.PathBuilder.md)

#### `flattened`

```rust
fn flattened(self, tolerance: f32) -> Flattened<Self> where Self: Sized,
```

Returns a builder that approximates all curves with sequences of line segments.

#### `transformed`

```rust
fn transformed<Transform>( self, transform: Transform, ) -> Transformed<Self, Transform> where Self: Sized, Transform: Transformation<f32>,
```

Returns a builder that applies the given transformation to all positions.

#### `with_svg`

```rust
fn with_svg(self) -> WithSvg<Self> where Self: Sized,
```

Returns a builder that support SVG commands. [Read more](trait.PathBuilder.md)

### `impl<B> StructuralPartialEq for NoAttributes<B> where B: PathBuilder,`

## Auto Trait Implementations

- `impl<B> Freeze for NoAttributes<B> where B: Freeze,`
- `impl<B> RefUnwindSafe for NoAttributes<B> where B: RefUnwindSafe,`
- `impl<B> Send for NoAttributes<B> where B: Send,`
- `impl<B> Sync for NoAttributes<B> where B: Sync,`
- `impl<B> Unpin for NoAttributes<B> where B: Unpin,`
- `impl<B> UnsafeUnpin for NoAttributes<B> where B: UnsafeUnpin,`
- `impl<B> UnwindSafe for NoAttributes<B> where B: UnwindSafe,`
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
