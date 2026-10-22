---
title: "Compositor"
kind: enum
path: "iced_renderer::fallback::Compositor"
crate: "iced_renderer"
module: "iced_renderer::fallback"
file: "iced_renderer/fallback/enum.Compositor.md"
docs_url: "https://docs.iced.rs/iced_renderer/fallback/enum.Compositor.html"
source_file: "src/iced_renderer/fallback.rs"
source_lines: "220-229"
---

# `iced_renderer::fallback::Compositor`

```rust
pub enum Compositor<A, B> where
    A: Compositor,
    B: Compositor, {
    Primary(A),
    Secondary(B),
}
```

A compositor `A` with a fallback strategy `B`.

It works analogously to [`Renderer`](enum.Renderer.md).

**Source:** [fallback.rs](../../src/iced_renderer/fallback.rs.md) (lines 220-229)

## Variants

### `Primary(A)`

The primary compositing option.

### `Secondary(B)`

The secondary (or fallback) compositing option.

## Trait Implementations

### `impl<A, B> Compositor for Compositor<A, B> where A: Compositor, B: Compositor,`

#### `Renderer`

```rust
type Renderer = Renderer<<A as Compositor>::Renderer, <B as Compositor>::Renderer>
```

The iced renderer of the backend.

#### `Surface`

```rust
type Surface = Surface<<A as Compositor>::Surface, <B as Compositor>::Surface>
```

The surface of the backend.

#### `new`

```rust
async fn new( settings: Settings, display: impl Display + Clone, compatible_window: impl Window + Clone, shell: Shell, ) -> Result<Self, Error>
```

Creates a new [`Compositor`] with the given [`backend::Settings`].

#### `create_renderer`

```rust
fn create_renderer(&self, settings: Settings) -> Self::Renderer
```

Creates a [`Self::Renderer`] for the [`Compositor`].

#### `create_surface`

```rust
fn create_surface( &mut self, window: impl Window + Clone, width: u32, height: u32, ) -> Self::Surface
```

Crates a new `Surface` for the given window.

#### `configure_surface`

```rust
fn configure_surface( &mut self, surface: &mut Self::Surface, width: u32, height: u32, )
```

Configures a new `Surface` with the given dimensions.

#### `load_font`

```rust
fn load_font(&mut self, font: Cow<'static, [u8]>) -> Result<(), Error>
```

Loads a font from its bytes.

#### `list_fonts`

```rust
fn list_fonts(&mut self) -> Result<Vec<Family>, Error>
```

Lists all the available font families.

#### `information`

```rust
fn information(&self) -> Information
```

Returns [`Information`] used by this [`Compositor`].

#### `present`

```rust
fn present( &mut self, renderer: &mut Self::Renderer, surface: &mut Self::Surface, viewport: &Viewport, background_color: Color, on_pre_present: impl FnOnce(), ) -> Result<(), SurfaceError>
```

Presents the `Renderer` primitives to the next frame of the given `Surface`.

#### `screenshot`

```rust
fn screenshot( &mut self, renderer: &mut Self::Renderer, viewport: &Viewport, background_color: Color, ) -> Vec<u8> ⓘ
```

Screenshots the current `Renderer` primitives to an offscreen texture, and returns the bytes of the texture ordered as `RGBA` in the `sRGB` color space.

### `impl<A, B> Debug for Compositor<A, B> where A: Compositor + Debug, B: Compositor + Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl<A, B> Freeze for Compositor<A, B> where A: Freeze, B: Freeze,`
- `impl<A, B> RefUnwindSafe for Compositor<A, B> where A: RefUnwindSafe, B: RefUnwindSafe,`
- `impl<A, B> Send for Compositor<A, B> where A: Send, B: Send,`
- `impl<A, B> Sync for Compositor<A, B> where A: Sync, B: Sync,`
- `impl<A, B> Unpin for Compositor<A, B> where A: Unpin, B: Unpin,`
- `impl<A, B> UnsafeUnpin for Compositor<A, B> where A: UnsafeUnpin, B: UnsafeUnpin,`
- `impl<A, B> UnwindSafe for Compositor<A, B> where A: UnwindSafe, B: UnwindSafe,`
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
