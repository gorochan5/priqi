---
title: "Layer"
kind: struct
path: "iced_wgpu::layer::Layer"
crate: "iced_wgpu"
module: "iced_wgpu::layer"
file: "iced_wgpu/layer/struct.Layer.md"
docs_url: "https://docs.iced.rs/iced_wgpu/layer/struct.Layer.html"
source_file: "src/iced_wgpu/layer.rs"
source_lines: "17-26"
---

# `iced_wgpu::layer::Layer`

```rust
pub struct Layer {
    pub bounds: Rectangle,
    pub quads: Batch,
    pub triangles: Vec<Item>,
    pub primitives: Batch,
    pub images: Vec<Image>,
    pub text: Vec<Item>,
    /* private fields */
}
```

**Source:** [layer.rs](../../src/iced_wgpu/layer.rs.md) (lines 17-26)

## Fields

### `bounds: Rectangle`

### `quads: Batch`

### `triangles: Vec<Item>`

### `primitives: Batch`

### `images: Vec<Image>`

### `text: Vec<Item>`

## Methods

### `impl Layer`

#### `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

#### `draw_quad`

```rust
pub fn draw_quad( &mut self, quad: Quad, background: Background, transformation: Transformation, )
```

#### `draw_paragraph`

```rust
pub fn draw_paragraph( &mut self, paragraph: &Paragraph, position: Point, color: Color, clip_bounds: Rectangle, transformation: Transformation, )
```

#### `draw_editor`

```rust
pub fn draw_editor( &mut self, editor: &Editor, position: Point, color: Color, clip_bounds: Rectangle, transformation: Transformation, )
```

#### `draw_text`

```rust
pub fn draw_text( &mut self, text: Text, position: Point, color: Color, clip_bounds: Rectangle, transformation: Transformation, )
```

#### `draw_text_raw`

```rust
pub fn draw_text_raw(&mut self, raw: Raw, transformation: Transformation)
```

#### `draw_image`

```rust
pub fn draw_image(&mut self, image: Image, transformation: Transformation)
```

#### `draw_raster`

```rust
pub fn draw_raster( &mut self, image: Image, bounds: Rectangle, clip_bounds: Rectangle, transformation: Transformation, )
```

#### `draw_svg`

```rust
pub fn draw_svg( &mut self, svg: Svg, bounds: Rectangle, clip_bounds: Rectangle, transformation: Transformation, )
```

#### `draw_mesh`

```rust
pub fn draw_mesh(&mut self, mesh: Mesh, transformation: Transformation)
```

#### `draw_mesh_group`

```rust
pub fn draw_mesh_group( &mut self, meshes: Vec<Mesh>, transformation: Transformation, )
```

#### `draw_mesh_cache`

```rust
pub fn draw_mesh_cache(&mut self, cache: Cache, transformation: Transformation)
```

#### `draw_text_group`

```rust
pub fn draw_text_group( &mut self, text: Vec<Text>, transformation: Transformation, )
```

#### `draw_text_cache`

```rust
pub fn draw_text_cache(&mut self, cache: Cache, transformation: Transformation)
```

#### `draw_primitive`

```rust
pub fn draw_primitive( &mut self, bounds: Rectangle, primitive: impl Primitive, transformation: Transformation, )
```

## Trait Implementations

### `impl Debug for Layer`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Layer`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Layer for Layer`

#### `with_bounds`

```rust
fn with_bounds(bounds: Rectangle) -> Self
```

Creates a new [`Layer`] with the given bounds.

#### `bounds`

```rust
fn bounds(&self) -> Rectangle
```

Returns the current bounds of the [`Layer`].

#### `flush`

```rust
fn flush(&mut self)
```

Flushes and settles any pending group of primitives in the [`Layer`]. Read more

#### `resize`

```rust
fn resize(&mut self, bounds: Rectangle)
```

Resizes the [`Layer`] to the given bounds.

#### `reset`

```rust
fn reset(&mut self)
```

Clears all the layers contents and resets its bounds.

#### `start`

```rust
fn start(&self) -> usize
```

Returns the start level of the [`Layer`]. Read more

#### `end`

```rust
fn end(&self) -> usize
```

Returns the end level of the [`Layer`].

#### `merge`

```rust
fn merge(&mut self, layer: &mut Self)
```

Merges a [`Layer`] with the current one.

## Auto Trait Implementations

- `impl Freeze for Layer`
- `impl !RefUnwindSafe for Layer`
- `impl Send for Layer`
- `impl Sync for Layer`
- `impl Unpin for Layer`
- `impl UnsafeUnpin for Layer`
- `impl !UnwindSafe for Layer`
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
