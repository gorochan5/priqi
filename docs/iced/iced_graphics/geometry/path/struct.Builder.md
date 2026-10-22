---
title: "Builder"
kind: struct
path: "iced_graphics::geometry::path::Builder"
crate: "iced_graphics"
module: "iced_graphics::geometry::path"
file: "iced_graphics/geometry/path/struct.Builder.md"
docs_url: "https://docs.iced.rs/iced_graphics/geometry/path/struct.Builder.html"
source_file: "src/iced_graphics/geometry/path/builder.rs"
source_lines: "13-15"
---

# `iced_graphics::geometry::path::Builder`

```rust
pub struct Builder { /* private fields */ }
```

A [`Path`](struct.Path.md) builder.

Once a [`Path`](struct.Path.md) is built, it can no longer be mutated.

**Source:** [builder.rs](../../../src/iced_graphics/geometry/path/builder.rs.md) (lines 13-15)

## Methods

### `impl Builder`

#### `new`

```rust
pub fn new() -> Builder
```

Creates a new [`Builder`](struct.Builder.md).

#### `move_to`

```rust
pub fn move_to(&mut self, point: Point)
```

Moves the starting point of a new sub-path to the given `Point`.

#### `line_to`

```rust
pub fn line_to(&mut self, point: Point)
```

Connects the last point in the [`Path`](struct.Path.md) to the given `Point` with a straight line.

#### `arc`

```rust
pub fn arc(&mut self, arc: Arc)
```

Adds an [`Arc`](arc/struct.Arc.md) to the [`Path`](struct.Path.md) from `start_angle` to `end_angle` in a clockwise direction.

#### `arc_to`

```rust
pub fn arc_to(&mut self, a: Point, b: Point, radius: f32)
```

Adds a circular arc to the [`Path`](struct.Path.md) with the given control points and radius.

This essentially draws a straight line segment from the current position to `a`, but fits a circular arc of `radius` tangent to that segment and tangent to the line between `a` and `b`.

With another `.line_to(b)`, the result will be a path connecting the starting point and `b` with straight line segments towards `a` and a circular arc smoothing out the corner at `a`.

See [the HTML5 specification of `arcTo`](https://html.spec.whatwg.org/multipage/canvas.html#building-paths:dom-context-2d-arcto) for more details and examples.

#### `ellipse`

```rust
pub fn ellipse(&mut self, arc: Elliptical)
```

Adds an ellipse to the [`Path`](struct.Path.md) using a clockwise direction.

#### `bezier_curve_to`

```rust
pub fn bezier_curve_to(&mut self, control_a: Point, control_b: Point, to: Point)
```

Adds a cubic Bézier curve to the [`Path`](struct.Path.md) given its two control points and its end point.

#### `quadratic_curve_to`

```rust
pub fn quadratic_curve_to(&mut self, control: Point, to: Point)
```

Adds a quadratic Bézier curve to the [`Path`](struct.Path.md) given its control point and its end point.

#### `rectangle`

```rust
pub fn rectangle(&mut self, top_left: Point, size: Size)
```

Adds a rectangle to the [`Path`](struct.Path.md) given its top-left corner coordinate and its `Size`.

#### `rounded_rectangle`

```rust
pub fn rounded_rectangle(&mut self, top_left: Point, size: Size, radius: Radius)
```

Adds a rounded rectangle to the [`Path`](struct.Path.md) given its top-left corner coordinate its [`Size`] and [`border::Radius`].

#### `circle`

```rust
pub fn circle(&mut self, center: Point, radius: f32)
```

Adds a circle to the [`Path`](struct.Path.md) given its center coordinate and its radius.

#### `close`

```rust
pub fn close(&mut self)
```

Closes the current sub-path in the [`Path`](struct.Path.md) with a straight line to the starting point.

#### `build`

```rust
pub fn build(self) -> Path
```

Builds the [`Path`](struct.Path.md) of this [`Builder`](struct.Builder.md).

## Trait Implementations

### `impl Default for Builder`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations

- `impl Freeze for Builder`
- `impl RefUnwindSafe for Builder`
- `impl Send for Builder`
- `impl Sync for Builder`
- `impl Unpin for Builder`
- `impl UnsafeUnpin for Builder`
- `impl UnwindSafe for Builder`
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

### `impl<T> From<T> for T`

#### `from`

```rust
fn from(t: T) -> T
```

Returns the argument unchanged.

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

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
