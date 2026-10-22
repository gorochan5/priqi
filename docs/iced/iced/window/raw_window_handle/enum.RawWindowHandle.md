---
title: "RawWindowHandle"
kind: enum
path: "iced::window::raw_window_handle::RawWindowHandle"
crate: "iced"
module: "iced::window::raw_window_handle"
file: "iced/window/raw_window_handle/enum.RawWindowHandle.md"
docs_url: "https://docs.iced.rs/iced/window/raw_window_handle/enum.RawWindowHandle.html"
---

# `iced::window::raw_window_handle::RawWindowHandle`

```rust
#[non_exhaustive]pub enum RawWindowHandle {
Show 16 variants    UiKit(UiKitWindowHandle),
    AppKit(AppKitWindowHandle),
    Orbital(OrbitalWindowHandle),
    OhosNdk(OhosNdkWindowHandle),
    Xlib(XlibWindowHandle),
    Xcb(XcbWindowHandle),
    Wayland(WaylandWindowHandle),
    Drm(DrmWindowHandle),
    Gbm(GbmWindowHandle),
    Win32(Win32WindowHandle),
    WinRt(WinRtWindowHandle),
    Web(WebWindowHandle),
    WebCanvas(WebCanvasWindowHandle),
    WebOffscreenCanvas(WebOffscreenCanvasWindowHandle),
    AndroidNdk(AndroidNdkWindowHandle),
    Haiku(HaikuWindowHandle),
}
```

A window handle for a particular windowing system.

Each variant contains a struct with fields specific to that windowing system (e.g. [`Win32WindowHandle`](struct.Win32WindowHandle.md) will include a [HWND](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-windows#window-handle), [`WaylandWindowHandle`](struct.WaylandWindowHandle.md) uses [wl_surface](https://wayland.freedesktop.org/docs/html/apa.html#protocol-spec-wl_surface), etc.)

## Variant Availability

Note that all variants are present on all targets (none are disabled behind `#[cfg]`s), but see the “Availability Hints” section on each variant for some hints on where this variant might be expected.

Note that these “Availability Hints” are not normative. That is to say, a [`HasWindowHandle`](trait.HasWindowHandle.md) implementor is completely allowed to return something unexpected. (For example, it’s legal for someone to return a [`RawWindowHandle::Xlib`](enum.RawWindowHandle.md) on macOS, it would just be weird, and probably requires something like XQuartz be used).

## Variants

### `UiKit(UiKitWindowHandle)`

A raw window handle for UIKit (Apple’s non-macOS windowing library).

###### Availability Hints

This variant is likely to be used on iOS, tvOS, (in theory) watchOS, and Mac Catalyst (`$arch-apple-ios-macabi` targets, which can notably use UIKit *or* AppKit), as these are the targets that (currently) support UIKit.

### `AppKit(AppKitWindowHandle)`

A raw window handle for AppKit.

###### Availability Hints

This variant is likely to be used on macOS, although Mac Catalyst (`$arch-apple-ios-macabi` targets, which can notably use UIKit *or* AppKit) can also use it despite being `target_os = "ios"`.

### `Orbital(OrbitalWindowHandle)`

A raw window handle for the Redox operating system.

###### Availability Hints

This variant is used by the Orbital Windowing System in the Redox operating system.

### `OhosNdk(OhosNdkWindowHandle)`

A raw window handle for the OpenHarmony OS NDK

###### Availability Hints

This variant is used on OpenHarmony OS (`target_env = "ohos"`).

### `Xlib(XlibWindowHandle)`

A raw window handle for Xlib.

###### Availability Hints

This variant is likely to show up anywhere someone manages to get X11 working that Xlib can be built for, which is to say, most (but not all) Unix systems.

### `Xcb(XcbWindowHandle)`

A raw window handle for Xcb.

###### Availability Hints

This variant is likely to show up anywhere someone manages to get X11 working that XCB can be built for, which is to say, most (but not all) Unix systems.

### `Wayland(WaylandWindowHandle)`

A raw window handle for Wayland.

###### Availability Hints

This variant should be expected anywhere Wayland works, which is currently some subset of unix systems.

### `Drm(DrmWindowHandle)`

A raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager

###### Availability Hints

This variant is used on Linux when neither X nor Wayland are available

### `Gbm(GbmWindowHandle)`

A raw window handle for the Linux Generic Buffer Manager.

###### Availability Hints

This variant is present regardless of windowing backend and likely to be used with EGL_MESA_platform_gbm or EGL_KHR_platform_gbm.

### `Win32(Win32WindowHandle)`

A raw window handle for Win32.

###### Availability Hints

This variant is used on Windows systems.

### `WinRt(WinRtWindowHandle)`

A raw window handle for WinRT.

###### Availability Hints

This variant is used on Windows systems.

### `Web(WebWindowHandle)`

A raw window handle for the Web.

###### Availability Hints

This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.

### `WebCanvas(WebCanvasWindowHandle)`

A raw window handle for a Web canvas registered via [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen).

###### Availability Hints

This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.

### `WebOffscreenCanvas(WebOffscreenCanvasWindowHandle)`

A raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen).

###### Availability Hints

This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.

### `AndroidNdk(AndroidNdkWindowHandle)`

A raw window handle for Android NDK.

###### Availability Hints

This variant is used on Android targets.

### `Haiku(HaikuWindowHandle)`

A raw window handle for Haiku.

###### Availability Hints

This variant is used on HaikuOS.

## Trait Implementations

### `impl AsRef<RawWindowHandle> for WindowHandle<'_>`

#### `as_ref`

```rust
fn as_ref(&self) -> &RawWindowHandle
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl Borrow<RawWindowHandle> for WindowHandle<'_>`

#### `borrow`

```rust
fn borrow(&self) -> &RawWindowHandle
```

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

### `impl Clone for RawWindowHandle`

#### `clone`

```rust
fn clone(&self) -> RawWindowHandle
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for RawWindowHandle`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<AndroidNdkWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: AndroidNdkWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<AppKitWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: AppKitWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<DrmWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: DrmWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<GbmWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: GbmWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<HaikuWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: HaikuWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<OhosNdkWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: OhosNdkWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<OrbitalWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: OrbitalWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<UiKitWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: UiKitWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<WaylandWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: WaylandWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<WebCanvasWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: WebCanvasWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<WebOffscreenCanvasWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: WebOffscreenCanvasWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<WebWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: WebWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<Win32WindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: Win32WindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<WinRtWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: WinRtWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<WindowHandle<'_>> for RawWindowHandle`

#### `from`

```rust
fn from(handle: WindowHandle<'_>) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<XcbWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: XcbWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl From<XlibWindowHandle> for RawWindowHandle`

#### `from`

```rust
fn from(value: XlibWindowHandle) -> RawWindowHandle
```

Converts to this type from the input type.

### `impl Hash for RawWindowHandle`

#### `hash`

```rust
fn hash<__H>(&self, state: &mut __H) where __H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl PartialEq for RawWindowHandle`

#### `eq`

```rust
fn eq(&self, other: &RawWindowHandle) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for RawWindowHandle`

### `impl Eq for RawWindowHandle`

### `impl StructuralPartialEq for RawWindowHandle`

## Auto Trait Implementations

- `impl Freeze for RawWindowHandle`
- `impl RefUnwindSafe for RawWindowHandle`
- `impl !Send for RawWindowHandle`
- `impl !Sync for RawWindowHandle`
- `impl Unpin for RawWindowHandle`
- `impl UnsafeUnpin for RawWindowHandle`
- `impl UnwindSafe for RawWindowHandle`
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

### `impl<T> CloneToUninit for T where T: Clone,`

#### `clone_to_uninit`

```rust
unsafe fn clone_to_uninit(&self, dest: *mut u8)
```

*🔬This is a nightly-only experimental API. (clone_to_uninit)*

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

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

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Checks if this value is equivalent to the given key. Read more

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

### `impl<State, Message> IntoBoot<State, Message> for State`

#### `into_boot`

```rust
fn into_boot(self) -> (State, Task<Message>)
```

Turns some type into the initial state of some [`Application`](../../application/struct.Application.md).

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

### `impl<T> Same for T`

#### `Output`

```rust
type Output = T
```

Should always be `Self`

### `impl<T> ToOwned for T where T: Clone,`

#### `Owned`

```rust
type Owned = T
```

The resulting type after obtaining ownership.

#### `to_owned`

```rust
fn to_owned(&self) -> T
```

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

#### `clone_into`

```rust
fn clone_into(&self, target: &mut T)
```

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

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

### `impl<V, T> VZip<V> for T where V: MultiLane<T>,`

#### `vzip`

```rust
fn vzip(self) -> V
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

### `impl<T> MaybeClone for T where T: Clone,`

### `impl<T> MaybeDebug for T where T: Debug,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
