---
title: "Renderer"
kind: enum
path: "iced_renderer::fallback::Renderer"
crate: "iced_renderer"
module: "iced_renderer::fallback"
file: "iced_renderer/fallback/enum.Renderer.md"
docs_url: "https://docs.iced.rs/iced_renderer/fallback/enum.Renderer.html"
source_file: "src/iced_renderer/fallback.rs"
source_lines: "20-25"
---

# `iced_renderer::fallback::Renderer`

```rust
pub enum Renderer<A, B> {
    Primary(A),
    Secondary(B),
}
```

A renderer `A` with a fallback strategy `B`.

This type can be used to easily compose existing renderers and create custom, type-safe fallback strategies.

**Source:** [fallback.rs](../../src/iced_renderer/fallback.rs.md) (lines 20-25)

## Variants

### `Primary(A)`

The primary rendering option.

### `Secondary(B)`

The secondary (or fallback) rendering option.

## Trait Implementations

### `impl<A: Debug, B: Debug> Debug for Renderer<A, B>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<A, B> Default for Renderer<A, B> where A: Default, B: Default,`

#### `Compositor`

```rust
type Compositor = Compositor<<A as Default>::Compositor, <B as Default>::Compositor>
```

The compositor of the renderer.

### `impl<A, B> Headless for Renderer<A, B> where A: Headless, B: Headless,`

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

### `impl<A, B> Renderer for Renderer<A, B> where A: Renderer, B: Renderer,`

#### `fill_quad`

```rust
fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>)
```

Fills a [`Quad`] with the provided [`Background`].

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

#### `allocate_image`

```rust
fn allocate_image( &self, handle: &Handle, callback: impl FnOnce(Result<Allocation, Error>) + Send + 'static, )
```

Creates an [`image::Allocation`] for the given [`image::Handle`] and calls the given callback with it.

#### `hint`

```rust
fn hint(&mut self, scale: Scale)
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

### `impl<A, B> Renderer for Renderer<A, B> where A: Renderer, B: Renderer<Paragraph = A::Paragraph, Editor = A::Editor>,`

#### `ICON_FONT`

```rust
const ICON_FONT: Font = A::ICON_FONT
```

The icon font of the backend.

#### `CHECKMARK_ICON`

```rust
const CHECKMARK_ICON: char = A::CHECKMARK_ICON
```

The `char` representing a ✔ icon in the `ICON_FONT`.

#### `ARROW_DOWN_ICON`

```rust
const ARROW_DOWN_ICON: char = A::ARROW_DOWN_ICON
```

The `char` representing a ▼ icon in the built-in `ICON_FONT`.

#### `SCROLL_UP_ICON`

```rust
const SCROLL_UP_ICON: char = A::SCROLL_UP_ICON
```

The `char` representing a ^ icon in the built-in `ICON_FONT`.

#### `SCROLL_DOWN_ICON`

```rust
const SCROLL_DOWN_ICON: char = A::SCROLL_DOWN_ICON
```

The `char` representing a v icon in the built-in `ICON_FONT`.

#### `SCROLL_LEFT_ICON`

```rust
const SCROLL_LEFT_ICON: char = A::SCROLL_LEFT_ICON
```

The `char` representing a < icon in the built-in `ICON_FONT`.

#### `SCROLL_RIGHT_ICON`

```rust
const SCROLL_RIGHT_ICON: char = A::SCROLL_RIGHT_ICON
```

The `char` representing a > icon in the built-in `ICON_FONT`.

#### `ICED_LOGO`

```rust
const ICED_LOGO: char = A::ICED_LOGO
```

The ‘char’ representing the iced logo in the built-in ‘ICON_FONT’.

#### `Paragraph`

```rust
type Paragraph = <A as Renderer>::Paragraph
```

The [`Paragraph`] of this [`Renderer`].

#### `Editor`

```rust
type Editor = <A as Renderer>::Editor
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
fn fill_text( &mut self, text: Text<String>, position: Point, color: Color, clip_bounds: Rectangle, )
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

### `impl<A, B> Renderer for Renderer<A, B> where A: Renderer, B: Renderer,`

#### `fill_raw`

```rust
fn fill_raw(&mut self, raw: Raw)
```

Draws the given [`Raw`] text.

### `impl<A, B> Renderer for Renderer<A, B> where A: Renderer, B: Renderer<Handle = A::Handle>,`

#### `Handle`

```rust
type Handle = <A as Renderer>::Handle
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
fn draw_image( &mut self, image: Image<A::Handle>, bounds: Rectangle, clip_bounds: Rectangle, )
```

Draws an [`Image`](../geometry/struct.Image.md) inside the provided `bounds`. Read more

### `impl<A, B> Renderer for Renderer<A, B> where A: Renderer, B: Renderer,`

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

### `impl<A, B> Renderer for Renderer<A, B> where A: Renderer, B: Renderer,`

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

### `impl<A, B> Renderer for Renderer<A, B> where A: Renderer, B: Renderer,`

#### `draw_primitive`

```rust
fn draw_primitive(&mut self, bounds: Rectangle, primitive: impl Primitive)
```

Draws a custom primitive.

### `impl<A, B> Renderer for Renderer<A, B> where A: Renderer, B: Renderer,`

#### `Geometry`

```rust
type Geometry = Geometry<<A as Renderer>::Geometry, <B as Renderer>::Geometry>
```

The kind of geometry this renderer can draw.

#### `Frame`

```rust
type Frame = Frame<<A as Renderer>::Frame, <B as Renderer>::Frame>
```

The kind of [`Frame`](../geometry/struct.Frame.md) this renderer supports.

#### `new_frame`

```rust
fn new_frame(&self, bounds: Rectangle) -> Self::Frame
```

Creates a new [`Self::Frame`](../geometry/trait.Renderer.md).

#### `draw_geometry`

```rust
fn draw_geometry(&mut self, geometry: Self::Geometry)
```

Draws the given [`Self::Geometry`](../geometry/trait.Renderer.md).

## Auto Trait Implementations

- `impl<A, B> Freeze for Renderer<A, B> where A: Freeze, B: Freeze,`
- `impl<A, B> RefUnwindSafe for Renderer<A, B> where A: RefUnwindSafe, B: RefUnwindSafe,`
- `impl<A, B> Send for Renderer<A, B> where A: Send, B: Send,`
- `impl<A, B> Sync for Renderer<A, B> where A: Sync, B: Sync,`
- `impl<A, B> Unpin for Renderer<A, B> where A: Unpin, B: Unpin,`
- `impl<A, B> UnsafeUnpin for Renderer<A, B> where A: UnsafeUnpin, B: UnsafeUnpin,`
- `impl<A, B> UnwindSafe for Renderer<A, B> where A: UnwindSafe, B: UnwindSafe,`
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
