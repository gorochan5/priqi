---
title: "Flattened"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::builder::Flattened"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::builder"
file: "iced_renderer/geometry/path/lyon_path/builder/struct.Flattened.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/builder/struct.Flattened.html"
---

# `iced_renderer::geometry::path::lyon_path::builder::Flattened`

```rust
pub struct Flattened<Builder> { /* private fields */ }
```

A Builder that approximates curves with successions of line segments.

## Methods

### `impl<Builder> Flattened<Builder> where Builder: PathBuilder,`

#### `new`

```rust
pub fn new(builder: Builder, tolerance: f32) -> Flattened<Builder>
```

#### `build`

```rust
pub fn build(self) -> <Builder as Build>::PathType where Builder: Build,
```

#### `set_tolerance`

```rust
pub fn set_tolerance(&mut self, tolerance: f32)
```

## Trait Implementations

### `impl<Builder> Build for Flattened<Builder> where Builder: Build,`

#### `PathType`

```rust
type PathType = <Builder as Build>::PathType
```

The type of object that is created by this builder.

#### `build`

```rust
fn build(self) -> <Builder as Build>::PathType
```

Builds a path object, consuming the builder.

### `impl<Builder> PathBuilder for Flattened<Builder> where Builder: PathBuilder,`

#### `num_attributes`

```rust
fn num_attributes(&self) -> usize
```

#### `begin`

```rust
fn begin( &mut self, at: Point2D<f32, UnknownUnit>, attributes: &[f32], ) -> EndpointId
```

Starts a new sub-path at a given position. [Read more](trait.PathBuilder.md)

#### `end`

```rust
fn end(&mut self, close: bool)
```

Ends the current sub path. [Read more](trait.PathBuilder.md)

#### `line_to`

```rust
fn line_to( &mut self, to: Point2D<f32, UnknownUnit>, attributes: &[f32], ) -> EndpointId
```

Adds a line segment to the current sub-path. [Read more](trait.PathBuilder.md)

#### `quadratic_bezier_to`

```rust
fn quadratic_bezier_to( &mut self, ctrl: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, attributes: &[f32], ) -> EndpointId
```

Adds a quadratic bézier curve to the current sub-path. [Read more](trait.PathBuilder.md)

#### `cubic_bezier_to`

```rust
fn cubic_bezier_to( &mut self, ctrl1: Point2D<f32, UnknownUnit>, ctrl2: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, attributes: &[f32], ) -> EndpointId
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

## Auto Trait Implementations

- `impl<Builder> Freeze for Flattened<Builder> where Builder: Freeze,`
- `impl<Builder> RefUnwindSafe for Flattened<Builder> where Builder: RefUnwindSafe,`
- `impl<Builder> Send for Flattened<Builder> where Builder: Send,`
- `impl<Builder> Sync for Flattened<Builder> where Builder: Sync,`
- `impl<Builder> Unpin for Flattened<Builder> where Builder: Unpin,`
- `impl<Builder> UnsafeUnpin for Flattened<Builder> where Builder: UnsafeUnpin,`
- `impl<Builder> UnwindSafe for Flattened<Builder> where Builder: UnwindSafe,`
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
