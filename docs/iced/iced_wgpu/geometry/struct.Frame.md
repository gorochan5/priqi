---
title: "Frame"
kind: struct
path: "iced_wgpu::geometry::Frame"
crate: "iced_wgpu"
module: "iced_wgpu::geometry"
file: "iced_wgpu/geometry/struct.Frame.md"
docs_url: "https://docs.iced.rs/iced_wgpu/geometry/struct.Frame.html"
source_file: "src/iced_wgpu/geometry.rs"
source_lines: "96-105"
---

# `iced_wgpu::geometry::Frame`

```rust
pub struct Frame { /* private fields */ }
```

A frame for drawing some geometry.

**Source:** [geometry.rs](../../src/iced_wgpu/geometry.rs.md) (lines 96-105)

## Methods

### `impl Frame`

#### `new`

```rust
pub fn new(bounds: Rectangle) -> Frame
```

Creates a new [`Frame`](struct.Frame.md) with the given clip bounds.

## Trait Implementations

### `impl Backend for Frame`

#### `Geometry`

```rust
type Geometry = Geometry
```

#### `width`

```rust
fn width(&self) -> f32
```

#### `height`

```rust
fn height(&self) -> f32
```

#### `size`

```rust
fn size(&self) -> Size
```

#### `center`

```rust
fn center(&self) -> Point
```

#### `fill`

```rust
fn fill(&mut self, path: &Path, fill: impl Into<Fill>)
```

#### `fill_rectangle`

```rust
fn fill_rectangle(&mut self, top_left: Point, size: Size, fill: impl Into<Fill>)
```

#### `stroke`

```rust
fn stroke<'a>(&mut self, path: &Path, stroke: impl Into<Stroke<'a>>)
```

#### `stroke_rectangle`

```rust
fn stroke_rectangle<'a>( &mut self, top_left: Point, size: Size, stroke: impl Into<Stroke<'a>>, )
```

#### `stroke_text`

```rust
fn stroke_text<'a>( &mut self, text: impl Into<Text>, stroke: impl Into<Stroke<'a>>, )
```

#### `fill_text`

```rust
fn fill_text(&mut self, text: impl Into<Text>)
```

#### `translate`

```rust
fn translate(&mut self, translation: Vector)
```

#### `rotate`

```rust
fn rotate(&mut self, angle: impl Into<Radians>)
```

#### `scale`

```rust
fn scale(&mut self, scale: f32)
```

#### `scale_nonuniform`

```rust
fn scale_nonuniform(&mut self, scale: impl Into<Vector>)
```

#### `push_transform`

```rust
fn push_transform(&mut self)
```

#### `pop_transform`

```rust
fn pop_transform(&mut self)
```

#### `draft`

```rust
fn draft(&mut self, clip_bounds: Rectangle) -> Frame
```

#### `paste`

```rust
fn paste(&mut self, frame: Frame)
```

#### `into_geometry`

```rust
fn into_geometry(self) -> Self::Geometry
```

#### `draw_image`

```rust
fn draw_image(&mut self, bounds: Rectangle, image: impl Into<Image>)
```

#### `draw_svg`

```rust
fn draw_svg(&mut self, bounds: Rectangle, svg: impl Into<Svg>)
```

## Auto Trait Implementations

- `impl Freeze for Frame`
- `impl RefUnwindSafe for Frame`
- `impl Send for Frame`
- `impl Sync for Frame`
- `impl Unpin for Frame`
- `impl UnsafeUnpin for Frame`
- `impl UnwindSafe for Frame`
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

### `impl<T> Upcast<T> for T`

#### `upcast`

```rust
fn upcast(&self) -> Option<&T>
```

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
