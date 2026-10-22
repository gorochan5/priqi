---
title: "Surface"
kind: struct
path: "iced_wgpu::window::Surface"
crate: "iced_wgpu"
module: "iced_wgpu::window"
file: "iced_wgpu/window/struct.Surface.md"
docs_url: "https://docs.iced.rs/iced_wgpu/window/struct.Surface.html"
---

# `iced_wgpu::window::Surface`

```rust
pub struct Surface<'window> { /* private fields */ }
```

Handle to a presentable surface.

A `Surface` represents a platform-specific surface (e.g. a window) onto which rendered images may be presented. A `Surface` may be created with the function [`Instance::create_surface`].

This type is unique to the Rust API of `wgpu`. In the WebGPU specification, [`GPUCanvasContext`](https://gpuweb.github.io/gpuweb/#canvas-context) serves a similar role.

## Methods

### `impl Surface<'_>`

#### `get_capabilities`

```rust
pub fn get_capabilities(&self, adapter: &Adapter) -> SurfaceCapabilities
```

Returns the capabilities of the surface when used with the given adapter.

Returns specified values (see [`SurfaceCapabilities`]) if surface is incompatible with the adapter.

#### `display_hdr_info`

```rust
pub fn display_hdr_info(&self, adapter: &Adapter) -> DisplayHdrInfo
```

Returns the HDR and luminance characteristics of the display backing this surface, or [`DisplayHdrInfo::default`] (all fields `None`) when nothing is known - which means unknown, not an SDR display. Never panics, including on wasm. See [`DisplayHdrInfo`] for the fields and how to use them.

###### Threading

Each call re-queries the OS; nothing is cached. On the Metal backend the display’s HDR state lives on main-thread-only AppKit objects (`NSScreen` / `NSWindow`), so call this from the main thread. Off the main thread it logs once and returns [`DisplayHdrInfo::default`]; a later main-thread call still returns real data. No other backend has this requirement.

#### `get_default_config`

```rust
pub fn get_default_config( &self, adapter: &Adapter, width: u32, height: u32, ) -> Option<SurfaceConfiguration<Vec<TextureFormat>>>
```

Return a default `SurfaceConfiguration` from width and height to use for the [`Surface`](struct.Surface.md) with this adapter.

The returned configuration requests the surface’s preferred format and [`SurfaceColorSpace::Auto`], reproducing wgpu’s historical SDR / standard behavior. Set the `color_space` field to opt into wide-gamut or HDR output; see [`SurfaceColorSpace`] for what each color space means.

Returns None if the surface isn’t supported by this adapter

#### `configure`

```rust
pub fn configure( &self, device: &Device, config: &SurfaceConfiguration<Vec<TextureFormat>>, )
```

Initializes [`Surface`](struct.Surface.md) for presentation.

If the surface is already configured, this will wait for the GPU to come idle before recreating the swapchain to prevent race conditions.

###### Validation Errors

- Submissions that happen *during* the configure may cause the internal wait-for-idle to fail, raising a validation error.

###### Panics

- An old [`SurfaceTexture`] is still alive referencing an old surface.
- Texture format requested is unsupported on the surface.
- The requested color space is unsupported for the requested format (see [`SurfaceCapabilities::format_capabilities`]).
- `config.width` or `config.height` is zero.

#### `get_configuration`

```rust
pub fn get_configuration( &self, ) -> Option<SurfaceConfiguration<Vec<TextureFormat>>>
```

Returns the current configuration of [`Surface`](struct.Surface.md), if configured.

This is similar to [WebGPU `GPUcCanvasContext::getConfiguration`](https://gpuweb.github.io/gpuweb/#dom-gpucanvascontext-getconfiguration).

Note that this returns the configuration as passed to [`Surface::configure`](struct.Surface.md): automatic values such as [`SurfaceColorSpace::Auto`] are returned as-is, not as the concrete values they resolved to.

#### `get_current_texture`

```rust
pub fn get_current_texture(&self) -> CurrentSurfaceTexture
```

Returns the next texture to be presented by the surface for drawing.

After rendering to the returned [`SurfaceTexture`], submit work via [`Queue::submit`] and then call [`Queue::present`] to display it.

If a [`SurfaceTexture`] referencing this surface is alive when [`Surface::configure()`](struct.Surface.md) is called, the configure call will panic.

See the documentation of [`CurrentSurfaceTexture`] for how each possible result should be handled.

#### `as_hal`

```rust
pub unsafe fn as_hal<A>( &self, ) -> Option<impl Deref<Target = <A as Api>::Surface> + WasmNotSendSync> where A: Api,
```

*Available on wgpu_core only.*

Get the [`wgpu_hal`] surface from this `Surface`.

Find the Api struct corresponding to the active backend in [`wgpu_hal::api`], and pass that struct to the to the `A` type parameter.

Returns a guard that dereferences to the type of the hal backend which implements `A::Surface`.

###### Types

The returned type depends on the backend:

- [`hal::api::Vulkan`] uses [`hal::vulkan::Surface`]
- `hal::api::Metal` uses `hal::metal::Surface`
- `hal::api::Dx12` uses `hal::dx12::Surface`
- [`hal::api::Gles`] uses [`hal::gles::Surface`]

###### Errors

This method will return None if:

- The surface is not from the backend specified by `A`.
- The surface is from the `webgpu` or `custom` backend.

###### Safety

- The returned resource must not be destroyed unless the guard is the last reference to it and it is not in use by the GPU. The guard and handle may be dropped at any time however.
- All the safety requirements of wgpu-hal must be upheld.

## Trait Implementations

### `impl Debug for Surface<'_>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Hash for Surface<'_>`

#### `hash`

```rust
fn hash<H>(&self, state: &mut H) where H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl Ord for Surface<'_>`

#### `cmp`

```rust
fn cmp(&self, other: &Surface<'_>) -> Ordering
```

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

#### `max`

```rust
fn max(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

#### `min`

```rust
fn min(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

#### `clamp`

```rust
fn clamp(self, min: Self, max: Self) -> Self where Self: Sized,
```

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

### `impl PartialEq for Surface<'_>`

#### `eq`

```rust
fn eq(&self, other: &Surface<'_>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd for Surface<'_>`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Surface<'_>) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, other: &Rhs) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, other: &Rhs) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `gt`

```rust
fn gt(&self, other: &Rhs) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

#### `ge`

```rust
fn ge(&self, other: &Rhs) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

### `impl Eq for Surface<'_>`

## Auto Trait Implementations

- `impl<'window> !Freeze for Surface<'window>`
- `impl<'window> !RefUnwindSafe for Surface<'window>`
- `impl<'window> Send for Surface<'window>`
- `impl<'window> Sync for Surface<'window>`
- `impl<'window> Unpin for Surface<'window>`
- `impl<'window> UnsafeUnpin for Surface<'window>`
- `impl<'window> !UnwindSafe for Surface<'window>`
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

### `impl<Q, K> Comparable<K> for Q where Q: Ord + ?Sized, K: Borrow<Q> + ?Sized,`

#### `compare`

```rust
fn compare(&self, key: &K) -> Ordering
```

Compare self to `key` and return their ordering.

### `impl<T> Downcast<T> for T`

#### `downcast`

```rust
fn downcast(&self) -> &T
```

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Checks if this value is equivalent to the given key. Read more

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Compare self to `key` and return `true` if they are equal.

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
