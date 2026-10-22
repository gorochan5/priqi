---
title: "WithSvg"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::builder::WithSvg"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::builder"
file: "iced_renderer/geometry/path/lyon_path/builder/struct.WithSvg.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/builder/struct.WithSvg.html"
---

# `iced_renderer::geometry::path::lyon_path::builder::WithSvg`

```rust
pub struct WithSvg<Builder> where
    Builder: PathBuilder, { /* private fields */ }
```

Implements an SVG-like building interface on top of a PathBuilder.

## Methods

### `impl<Builder> WithSvg<Builder> where Builder: PathBuilder,`

#### `new`

```rust
pub fn new(builder: Builder) -> WithSvg<Builder>
```

#### `build`

```rust
pub fn build(self) -> <Builder as Build>::PathType where Builder: Build,
```

#### `flattened`

```rust
pub fn flattened(self, tolerance: f32) -> WithSvg<Flattened<Builder>>
```

#### `transformed`

```rust
pub fn transformed<Transform>( self, transform: Transform, ) -> WithSvg<Transformed<Builder, Transform>> where Transform: Transformation<f32>,
```

#### `move_to`

```rust
pub fn move_to(&mut self, to: Point2D<f32, UnknownUnit>) -> EndpointId
```

#### `line_to`

```rust
pub fn line_to(&mut self, to: Point2D<f32, UnknownUnit>) -> EndpointId
```

#### `close`

```rust
pub fn close(&mut self)
```

#### `quadratic_bezier_to`

```rust
pub fn quadratic_bezier_to( &mut self, ctrl: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, ) -> EndpointId
```

#### `cubic_bezier_to`

```rust
pub fn cubic_bezier_to( &mut self, ctrl1: Point2D<f32, UnknownUnit>, ctrl2: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, ) -> EndpointId
```

#### `arc`

```rust
pub fn arc( &mut self, center: Point2D<f32, UnknownUnit>, radii: Vector2D<f32, UnknownUnit>, sweep_angle: Angle<f32>, x_rotation: Angle<f32>, )
```

#### `current_position`

```rust
pub fn current_position(&self) -> Point2D<f32, UnknownUnit>
```

#### `reserve`

```rust
pub fn reserve(&mut self, endpoints: usize, ctrl_points: usize)
```

### `impl<Builder, Transform> WithSvg<Transformed<Builder, Transform>> where Builder: PathBuilder, Transform: Transformation<f32>,`

#### `set_transform`

```rust
pub fn set_transform(&mut self, transform: Transform)
```

## Trait Implementations

### `impl<Builder> Build for WithSvg<Builder> where Builder: PathBuilder + Build,`

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

### `impl<Builder> SvgPathBuilder for WithSvg<Builder> where Builder: PathBuilder,`

#### `move_to`

```rust
fn move_to(&mut self, to: Point2D<f32, UnknownUnit>)
```

Start a new sub-path at the given position. [Read more](trait.SvgPathBuilder.md)

#### `close`

```rust
fn close(&mut self)
```

Ends the current sub-path by connecting it back to its initial point. [Read more](trait.SvgPathBuilder.md)

#### `line_to`

```rust
fn line_to(&mut self, to: Point2D<f32, UnknownUnit>)
```

Adds a line segment to the current sub-path. [Read more](trait.SvgPathBuilder.md)

#### `quadratic_bezier_to`

```rust
fn quadratic_bezier_to( &mut self, ctrl: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, )
```

Adds a quadratic bézier segment to the current sub-path. [Read more](trait.SvgPathBuilder.md)

#### `cubic_bezier_to`

```rust
fn cubic_bezier_to( &mut self, ctrl1: Point2D<f32, UnknownUnit>, ctrl2: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, )
```

Adds a cubic bézier segment to the current sub-path. [Read more](trait.SvgPathBuilder.md)

#### `relative_move_to`

```rust
fn relative_move_to(&mut self, to: Vector2D<f32, UnknownUnit>)
```

Equivalent to `move_to` in relative coordinates. [Read more](trait.SvgPathBuilder.md)

#### `relative_line_to`

```rust
fn relative_line_to(&mut self, to: Vector2D<f32, UnknownUnit>)
```

Equivalent to `line_to` in relative coordinates. [Read more](trait.SvgPathBuilder.md)

#### `relative_quadratic_bezier_to`

```rust
fn relative_quadratic_bezier_to( &mut self, ctrl: Vector2D<f32, UnknownUnit>, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `quadratic_bezier_to` in relative coordinates. [Read more](trait.SvgPathBuilder.md)

#### `relative_cubic_bezier_to`

```rust
fn relative_cubic_bezier_to( &mut self, ctrl1: Vector2D<f32, UnknownUnit>, ctrl2: Vector2D<f32, UnknownUnit>, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `cubic_bezier_to` in relative coordinates. [Read more](trait.SvgPathBuilder.md)

#### `smooth_cubic_bezier_to`

```rust
fn smooth_cubic_bezier_to( &mut self, ctrl2: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, )
```

Equivalent to `cubic_bezier_to` with implicit first control point. [Read more](trait.SvgPathBuilder.md)

#### `smooth_relative_cubic_bezier_to`

```rust
fn smooth_relative_cubic_bezier_to( &mut self, ctrl2: Vector2D<f32, UnknownUnit>, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `smooth_cubic_bezier_to` in relative coordinates. [Read more](trait.SvgPathBuilder.md)

#### `smooth_quadratic_bezier_to`

```rust
fn smooth_quadratic_bezier_to(&mut self, to: Point2D<f32, UnknownUnit>)
```

Equivalent to `quadratic_bezier_to` with implicit control point. [Read more](trait.SvgPathBuilder.md)

#### `smooth_relative_quadratic_bezier_to`

```rust
fn smooth_relative_quadratic_bezier_to( &mut self, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `smooth_quadratic_bezier_to` in relative coordinates. [Read more](trait.SvgPathBuilder.md)

#### `horizontal_line_to`

```rust
fn horizontal_line_to(&mut self, x: f32)
```

Adds an horizontal line segment. [Read more](trait.SvgPathBuilder.md)

#### `relative_horizontal_line_to`

```rust
fn relative_horizontal_line_to(&mut self, dx: f32)
```

Adds an horizontal line segment in relative coordinates. [Read more](trait.SvgPathBuilder.md)

#### `vertical_line_to`

```rust
fn vertical_line_to(&mut self, y: f32)
```

Adds a vertical line segment. [Read more](trait.SvgPathBuilder.md)

#### `relative_vertical_line_to`

```rust
fn relative_vertical_line_to(&mut self, dy: f32)
```

Adds a vertical line segment in relative coordinates. [Read more](trait.SvgPathBuilder.md)

#### `arc_to`

```rust
fn arc_to( &mut self, radii: Vector2D<f32, UnknownUnit>, x_rotation: Angle<f32>, flags: ArcFlags, to: Point2D<f32, UnknownUnit>, )
```

Adds an elliptical arc. [Read more](trait.SvgPathBuilder.md)

#### `relative_arc_to`

```rust
fn relative_arc_to( &mut self, radii: Vector2D<f32, UnknownUnit>, x_rotation: Angle<f32>, flags: ArcFlags, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `arc_to` in relative coordinates. [Read more](trait.SvgPathBuilder.md)

#### `reserve`

```rust
fn reserve(&mut self, endpoints: usize, ctrl_points: usize)
```

Hints at the builder that a certain number of endpoints and control points will be added. [Read more](trait.SvgPathBuilder.md)

#### `add_polygon`

```rust
fn add_polygon(&mut self, polygon: Polygon<'_, Point2D<f32, UnknownUnit>>)
```

Adds a sub-path from a polygon. [Read more](trait.SvgPathBuilder.md)

## Auto Trait Implementations

- `impl<Builder> Freeze for WithSvg<Builder> where Builder: Freeze,`
- `impl<Builder> RefUnwindSafe for WithSvg<Builder> where Builder: RefUnwindSafe,`
- `impl<Builder> Send for WithSvg<Builder> where Builder: Send,`
- `impl<Builder> Sync for WithSvg<Builder> where Builder: Sync,`
- `impl<Builder> Unpin for WithSvg<Builder> where Builder: Unpin,`
- `impl<Builder> UnsafeUnpin for WithSvg<Builder> where Builder: UnsafeUnpin,`
- `impl<Builder> UnwindSafe for WithSvg<Builder> where Builder: UnwindSafe,`
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
