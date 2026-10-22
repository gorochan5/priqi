---
title: "Renderer"
kind: struct
path: "iced_wgpu::Renderer"
crate: "iced_wgpu"
module: "iced_wgpu"
file: "iced_wgpu/struct.Renderer.md"
docs_url: "https://docs.iced.rs/iced_wgpu/struct.Renderer.html"
source_file: "src/iced_wgpu/lib.rs"
source_lines: "73-93"
---

# `iced_wgpu::Renderer`

```rust
pub struct Renderer { /* private fields */ }
```

A [`wgpu`](https://github.com/gfx-rs/wgpu-rs) graphics renderer for [`iced`](https://github.com/iced-rs/iced).

**Source:** [lib.rs](../src/iced_wgpu/lib.rs.md) (lines 73-93)

## Methods

### `impl Renderer`

#### `new`

```rust
pub fn new(engine: Engine, settings: Settings) -> Self
```

#### `draw`

```rust
pub fn draw( &mut self, clear_color: Option<Color>, target: &TextureView, viewport: &Viewport, ) -> CommandEncoder
```

Record commands that draw the current primitives to the target texture view.

You must call [`finish`](struct.Renderer.md) and [`recall`](struct.Renderer.md) when submitting the resulting [`wgpu::CommandEncoder`].

#### `present`

```rust
pub fn present( &mut self, clear_color: Option<Color>, _format: TextureFormat, frame: &TextureView, viewport: &Viewport, ) -> SubmissionIndex
```

#### `screenshot`

```rust
pub fn screenshot( &mut self, viewport: &Viewport, background_color: Color, ) -> Vec<u8> ⓘ
```

Renders the current surface to an offscreen buffer.

Returns RGBA bytes of the texture data.

#### `finish`

```rust
pub fn finish(&mut self)
```

Prepares currently mapped buffers for use in a submission.

Usually, this method is only needed if you are calling [`Renderer::draw`](struct.Renderer.md) directly, instead of relying on [`Renderer::present`](struct.Renderer.md).

You must call this method *before* submitting the resulting [`wgpu::CommandEncoder`] of [`Renderer::draw`](struct.Renderer.md) to a [`wgpu::Queue`].

#### `recall`

```rust
pub fn recall(&mut self)
```

Recalls all of the closed buffers back to be reused.

Usually, this method is only needed if you are calling [`Renderer::draw`](struct.Renderer.md) directly, instead of relying on [`Renderer::present`](struct.Renderer.md) to a [`wgpu::Queue`].

You must call this method *after* submitting the resulting [`wgpu::CommandEncoder`] of [`Renderer::draw`](struct.Renderer.md) to a [`wgpu::Queue`].

## Trait Implementations

### `impl Default for Renderer`

#### `Compositor`

```rust
type Compositor = Compositor
```

The compositor of the renderer.

### `impl Headless for Renderer`

#### `new`

```rust
async fn new(settings: Settings, backend: Option<&str>) -> Option<Self>
```

Creates a new [`Headless`] renderer;

#### `name`

```rust
fn name(&self) -> String
```

Returns the unique name of the renderer. Read more

#### `screenshot`

```rust
fn screenshot( &mut self, size: Size<u32>, scale_factor: f32, background_color: Color, ) -> Vec<u8> ⓘ
```

Draws offscreen into a screenshot, returning a collection of bytes representing the rendered pixels in RGBA order.

### `impl Renderer for Renderer`

#### `start_layer`

```rust
fn start_layer(&mut self, bounds: Rectangle)
```

Starts recording a new layer.

#### `end_layer`

```rust
fn end_layer(&mut self)
```

Ends recording a new layer. Read more

#### `start_transformation`

```rust
fn start_transformation(&mut self, transformation: Transformation)
```

Starts recording with a new [`Transformation`].

#### `end_transformation`

```rust
fn end_transformation(&mut self)
```

Ends recording a new layer. Read more

#### `fill_quad`

```rust
fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>)
```

Fills a [`Quad`] with the provided [`Background`].

#### `allocate_image`

```rust
fn allocate_image( &self, _handle: &Handle, _callback: impl FnOnce(Result<Allocation, Error>) + Send + 'static, )
```

Creates an [`image::Allocation`] for the given [`image::Handle`] and calls the given callback with it.

#### `hint`

```rust
fn hint(&mut self, scale_factor: Scale)
```

Provides hints to the [`Renderer`] about the rendering target. Read more

#### `scale`

```rust
fn scale(&self) -> Option<Scale>
```

Returns the last [`Scale`] provided as a `hint`.

#### `tick`

```rust
fn tick(&mut self)
```

Polls any concurrent computations that may be pending in the [`Renderer`]. Read more

#### `reset`

```rust
fn reset(&mut self, new_bounds: Rectangle)
```

Resets the [`Renderer`] to start drawing in the `new_bounds` from scratch.

#### `settings`

```rust
fn settings(&self) -> Settings
```

Returns the [`Settings`] of this [`Renderer`].

#### `with_layer`

```rust
fn with_layer(&mut self, bounds: Rectangle, f: impl FnOnce(&mut Self))
```

Draws the primitives recorded in the given closure in a new layer. Read more

#### `with_transformation`

```rust
fn with_transformation( &mut self, transformation: Transformation, f: impl FnOnce(&mut Self), )
```

Applies a [`Transformation`] to the primitives recorded in the given closure.

#### `with_translation`

```rust
fn with_translation(&mut self, translation: Vector, f: impl FnOnce(&mut Self))
```

Applies a translation to the primitives recorded in the given closure.

#### `hint_factor`

```rust
fn hint_factor(&self) -> Option<f32>
```

Returns the last hint factor provided as a `hint`, only if [`Settings::metrics_hinting`] is enabled.

### `impl Renderer for Renderer`

#### `ICON_FONT`

```rust
const ICON_FONT: Font
```

The icon font of the backend.

#### `CHECKMARK_ICON`

```rust
const CHECKMARK_ICON: char = '\u{f00c}'
```

The `char` representing a ✔ icon in the `ICON_FONT`.

#### `ARROW_DOWN_ICON`

```rust
const ARROW_DOWN_ICON: char = '\u{e800}'
```

The `char` representing a ▼ icon in the built-in `ICON_FONT`.

#### `ICED_LOGO`

```rust
const ICED_LOGO: char = '\u{e801}'
```

The ‘char’ representing the iced logo in the built-in ‘ICON_FONT’.

#### `SCROLL_UP_ICON`

```rust
const SCROLL_UP_ICON: char = '\u{e802}'
```

The `char` representing a ^ icon in the built-in `ICON_FONT`.

#### `SCROLL_DOWN_ICON`

```rust
const SCROLL_DOWN_ICON: char = '\u{e803}'
```

The `char` representing a v icon in the built-in `ICON_FONT`.

#### `SCROLL_LEFT_ICON`

```rust
const SCROLL_LEFT_ICON: char = '\u{e804}'
```

The `char` representing a < icon in the built-in `ICON_FONT`.

#### `SCROLL_RIGHT_ICON`

```rust
const SCROLL_RIGHT_ICON: char = '\u{e805}'
```

The `char` representing a > icon in the built-in `ICON_FONT`.

#### `Paragraph`

```rust
type Paragraph = Paragraph
```

The [`Paragraph`] of this [`Renderer`].

#### `Editor`

```rust
type Editor = Editor
```

The [`Editor`] of this [`Renderer`].

#### `fill_paragraph`

```rust
fn fill_paragraph( &mut self, text: &Self::Paragraph, position: Point, color: Color, clip_bounds: Rectangle, )
```

Draws the given [`Paragraph`] at the given position and with the given [`Color`].

#### `fill_editor`

```rust
fn fill_editor( &mut self, editor: &Self::Editor, position: Point, color: Color, clip_bounds: Rectangle, )
```

Draws the given [`Editor`] at the given position and with the given [`Color`].

#### `fill_text`

```rust
fn fill_text( &mut self, text: Text, position: Point, color: Color, clip_bounds: Rectangle, )
```

Draws the given [`Text`] at the given position and with the given [`Color`].

#### `font`

```rust
fn font(&self) -> Font
```

Returns the default [`Font`].

#### `text_size`

```rust
fn text_size(&self) -> Pixels
```

Returns the default size of [`Text`].

#### `line_height`

```rust
fn line_height(&self) -> LineHeight
```

Returns the default line height of [`Text`].

### `impl Renderer for Renderer`

#### `fill_raw`

```rust
fn fill_raw(&mut self, raw: Raw)
```

Draws the given [`Raw`] text.

### `impl Renderer for Renderer`

#### `Handle`

```rust
type Handle = Handle
```

The image Handle to be displayed. Iced exposes its own default implementation of a `Handle`

#### `load_image`

```rust
fn load_image(&self, handle: &Self::Handle) -> Result<Allocation, Error>
```

Loads an image and returns an explicit [`Allocation`] to it. Read more

#### `measure_image`

```rust
fn measure_image(&self, handle: &Self::Handle) -> Option<Size<u32>>
```

Returns the dimensions of an image for the given [`Handle`]. Read more

#### `draw_image`

```rust
fn draw_image( &mut self, image: Image, bounds: Rectangle, clip_bounds: Rectangle, )
```

Draws an [`Image`] inside the provided `bounds`. Read more

### `impl Renderer for Renderer`

#### `measure_svg`

```rust
fn measure_svg(&self, handle: &Handle) -> Size<u32>
```

Returns the default dimensions of an SVG for the given [`Handle`].

#### `draw_svg`

```rust
fn draw_svg(&mut self, svg: Svg, bounds: Rectangle, clip_bounds: Rectangle)
```

Draws an SVG with the given [`Handle`], an optional [`Color`] filter, and inside the provided `bounds`.

### `impl Renderer for Renderer`

#### `draw_mesh`

```rust
fn draw_mesh(&mut self, mesh: Mesh)
```

Draws the given [`Mesh`].

#### `draw_mesh_cache`

```rust
fn draw_mesh_cache(&mut self, cache: Cache)
```

Draws the given [`Cache`].

### `impl Renderer for Renderer`

#### `Geometry`

```rust
type Geometry = Geometry
```

The kind of geometry this renderer can draw.

#### `Frame`

```rust
type Frame = Frame
```

The kind of [`Frame`] this renderer supports.

#### `new_frame`

```rust
fn new_frame(&self, bounds: Rectangle) -> Self::Frame
```

Creates a new [`Self::Frame`].

#### `draw_geometry`

```rust
fn draw_geometry(&mut self, geometry: Self::Geometry)
```

Draws the given [`Self::Geometry`].

### `impl Renderer for Renderer`

#### `draw_primitive`

```rust
fn draw_primitive(&mut self, bounds: Rectangle, primitive: impl Primitive)
```

Draws a custom primitive.

## Auto Trait Implementations

- `impl !Freeze for Renderer`
- `impl !RefUnwindSafe for Renderer`
- `impl Send for Renderer`
- `impl !Sync for Renderer`
- `impl Unpin for Renderer`
- `impl UnsafeUnpin for Renderer`
- `impl !UnwindSafe for Renderer`
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

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`
