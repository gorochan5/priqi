---
title: "Frame"
kind: struct
path: "iced_graphics::geometry::frame::Frame"
crate: "iced_graphics"
module: "iced_graphics::geometry::frame"
file: "iced_graphics/geometry/frame/struct.Frame.md"
docs_url: "https://docs.iced.rs/iced_graphics/geometry/frame/struct.Frame.html"
source_file: "src/iced_graphics/geometry/frame.rs"
source_lines: "6-11"
---

# `iced_graphics::geometry::frame::Frame`

```rust
pub struct Frame<Renderer> where
    Renderer: Renderer, { /* private fields */ }
```

The region of a surface that can be used to draw geometry.

**Source:** [frame.rs](../../../src/iced_graphics/geometry/frame.rs.md) (lines 6-11)

## Methods

### `impl<Renderer> Frame<Renderer> where Renderer: Renderer,`

#### `new`

```rust
pub fn new(renderer: &Renderer, size: Size) -> Self
```

Creates a new [`Frame`](struct.Frame.md) with the given dimensions.

Any geometry drawn outside of the dimensions will be clipped. If you need further control, use [`with_bounds`](struct.Frame.md).

#### `with_bounds`

```rust
pub fn with_bounds(renderer: &Renderer, bounds: Rectangle) -> Self
```

Creates a new [`Frame`](struct.Frame.md) with the given clip bounds.

#### `width`

```rust
pub fn width(&self) -> f32
```

Returns the width of the [`Frame`](struct.Frame.md).

#### `height`

```rust
pub fn height(&self) -> f32
```

Returns the height of the [`Frame`](struct.Frame.md).

#### `size`

```rust
pub fn size(&self) -> Size
```

Returns the dimensions of the [`Frame`](struct.Frame.md).

#### `center`

```rust
pub fn center(&self) -> Point
```

Returns the coordinate of the center of the [`Frame`](struct.Frame.md).

#### `fill`

```rust
pub fn fill(&mut self, path: &Path, fill: impl Into<Fill>)
```

Draws the given [`Path`](../path/struct.Path.md) on the [`Frame`](struct.Frame.md) by filling it with the provided style.

#### `fill_rectangle`

```rust
pub fn fill_rectangle( &mut self, top_left: Point, size: Size, fill: impl Into<Fill>, )
```

Draws an axis-aligned rectangle given its top-left corner coordinate and its `Size` on the [`Frame`](struct.Frame.md) by filling it with the provided style.

#### `stroke`

```rust
pub fn stroke<'a>(&mut self, path: &Path, stroke: impl Into<Stroke<'a>>)
```

Draws the stroke of the given [`Path`](../path/struct.Path.md) on the [`Frame`](struct.Frame.md) with the provided style.

#### `stroke_rectangle`

```rust
pub fn stroke_rectangle<'a>( &mut self, top_left: Point, size: Size, stroke: impl Into<Stroke<'a>>, )
```

Draws the stroke of an axis-aligned rectangle with the provided style given its top-left corner coordinate and its `Size` on the [`Frame`](struct.Frame.md) .

#### `fill_text`

```rust
pub fn fill_text(&mut self, text: impl Into<Text>)
```

Draws the characters of the given [`Text`](../struct.Text.md) on the [`Frame`](struct.Frame.md), filling them with the given color.

**Warning:** All text will be rendered on top of all the layers of a `Canvas`. Therefore, it is currently only meant to be used for overlays, which is the most common use case.

#### `draw_image`

```rust
pub fn draw_image(&mut self, bounds: Rectangle, image: impl Into<Image>)
```

*Available on crate feature image only.*

Draws the given [`Image`](../struct.Image.md) on the [`Frame`](struct.Frame.md) inside the given bounds.

#### `draw_svg`

```rust
pub fn draw_svg(&mut self, bounds: Rectangle, svg: impl Into<Svg>)
```

*Available on crate feature svg only.*

Draws the given [`Svg`](../struct.Svg.md) on the [`Frame`](struct.Frame.md) inside the given bounds.

#### `with_save`

```rust
pub fn with_save<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R
```

Stores the current transform of the [`Frame`](struct.Frame.md) and executes the given drawing operations, restoring the transform afterwards.

This method is useful to compose transforms and perform drawing operations in different coordinate systems.

#### `push_transform`

```rust
pub fn push_transform(&mut self)
```

Pushes the current transform in the transform stack.

#### `pop_transform`

```rust
pub fn pop_transform(&mut self)
```

Pops a transform from the transform stack and sets it as the current transform.

#### `with_clip`

```rust
pub fn with_clip<R>( &mut self, region: Rectangle, f: impl FnOnce(&mut Self) -> R, ) -> R
```

Executes the given drawing operations within a [`Rectangle`] region, clipping any geometry that overflows its bounds. Any transformations performed are local to the provided closure.

This method is useful to perform drawing operations that need to be clipped.

#### `translate`

```rust
pub fn translate(&mut self, translation: Vector)
```

Applies a translation to the current transform of the [`Frame`](struct.Frame.md).

#### `rotate`

```rust
pub fn rotate(&mut self, angle: impl Into<Radians>)
```

Applies a rotation in radians to the current transform of the [`Frame`](struct.Frame.md).

#### `scale`

```rust
pub fn scale(&mut self, scale: f32)
```

Applies a uniform scaling to the current transform of the [`Frame`](struct.Frame.md).

#### `scale_nonuniform`

```rust
pub fn scale_nonuniform(&mut self, scale: impl Into<Vector>)
```

Applies a non-uniform scaling to the current transform of the [`Frame`](struct.Frame.md).

#### `into_geometry`

```rust
pub fn into_geometry(self) -> Renderer::Geometry
```

Turns the [`Frame`](struct.Frame.md) into its underlying geometry.

## Auto Trait Implementations

- `impl<Renderer> Freeze for Frame<Renderer> where <Renderer as Renderer>::Frame: Freeze,`
- `impl<Renderer> RefUnwindSafe for Frame<Renderer> where <Renderer as Renderer>::Frame: RefUnwindSafe,`
- `impl<Renderer> Send for Frame<Renderer> where <Renderer as Renderer>::Frame: Send,`
- `impl<Renderer> Sync for Frame<Renderer> where <Renderer as Renderer>::Frame: Sync,`
- `impl<Renderer> Unpin for Frame<Renderer> where <Renderer as Renderer>::Frame: Unpin,`
- `impl<Renderer> UnsafeUnpin for Frame<Renderer> where <Renderer as Renderer>::Frame: UnsafeUnpin,`
- `impl<Renderer> UnwindSafe for Frame<Renderer> where <Renderer as Renderer>::Frame: UnwindSafe,`
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
