---
title: "Span"
kind: struct
path: "iced_widget::text::Span"
crate: "iced_widget"
module: "iced_widget::text"
file: "iced_widget/text/struct.Span.md"
docs_url: "https://docs.iced.rs/iced_widget/text/struct.Span.html"
---

# `iced_widget::text::Span`

```rust
pub struct Span<'a, Link = ()> {
    pub text: Cow<'a, str>,
    pub size: Option<Pixels>,
    pub line_height: Option<LineHeight>,
    pub font: Option<Font>,
    pub color: Option<Color>,
    pub link: Option<Link>,
    pub highlight: Option<Highlight>,
    pub padding: Padding,
    pub underline: bool,
    pub strikethrough: bool,
}
```

A span of text.

## Fields

### `text: Cow<'a, str>`

The [`Fragment`](type.Fragment.md) of text.

### `size: Option<Pixels>`

The size of the [`Span`](struct.Span.md) in [`Pixels`].

### `line_height: Option<LineHeight>`

The [`LineHeight`](enum.LineHeight.md) of the [`Span`](struct.Span.md).

### `font: Option<Font>`

The [`Font`] of the [`Span`](struct.Span.md).

### `color: Option<Color>`

The [`Color`] of the [`Span`](struct.Span.md).

### `link: Option<Link>`

The link of the [`Span`](struct.Span.md).

### `highlight: Option<Highlight>`

The [`Highlight`](../markdown/struct.Highlight.md) of the [`Span`](struct.Span.md).

### `padding: Padding`

The [`Padding`] of the [`Span`](struct.Span.md).

Currently, it only affects the bounds of the [`Highlight`](../markdown/struct.Highlight.md).

### `underline: bool`

Whether the [`Span`](struct.Span.md) should be underlined or not.

### `strikethrough: bool`

Whether the [`Span`](struct.Span.md) should be struck through or not.

## Methods

### `impl<'a, Link> Span<'a, Link>`

#### `new`

```rust
pub fn new(fragment: impl IntoFragment<'a>) -> Span<'a, Link>
```

Creates a new [`Span`](struct.Span.md) of text with the given text fragment.

#### `size`

```rust
pub fn size(self, size: impl Into<Pixels>) -> Span<'a, Link>
```

Sets the size of the [`Span`](struct.Span.md).

#### `line_height`

```rust
pub fn line_height(self, line_height: impl Into<LineHeight>) -> Span<'a, Link>
```

Sets the [`LineHeight`](enum.LineHeight.md) of the [`Span`](struct.Span.md).

#### `font`

```rust
pub fn font(self, font: impl Into<Font>) -> Span<'a, Link>
```

Sets the font of the [`Span`](struct.Span.md).

#### `font_maybe`

```rust
pub fn font_maybe(self, font: Option<impl Into<Font>>) -> Span<'a, Link>
```

Sets the font of the [`Span`](struct.Span.md), if any.

#### `color`

```rust
pub fn color(self, color: impl Into<Color>) -> Span<'a, Link>
```

Sets the [`Color`] of the [`Span`](struct.Span.md).

#### `color_maybe`

```rust
pub fn color_maybe(self, color: Option<impl Into<Color>>) -> Span<'a, Link>
```

Sets the [`Color`] of the [`Span`](struct.Span.md), if any.

#### `link`

```rust
pub fn link(self, link: impl Into<Link>) -> Span<'a, Link>
```

Sets the link of the [`Span`](struct.Span.md).

#### `link_maybe`

```rust
pub fn link_maybe(self, link: Option<impl Into<Link>>) -> Span<'a, Link>
```

Sets the link of the [`Span`](struct.Span.md), if any.

#### `background`

```rust
pub fn background(self, background: impl Into<Background>) -> Span<'a, Link>
```

Sets the [`Background`] of the [`Span`](struct.Span.md).

#### `background_maybe`

```rust
pub fn background_maybe( self, background: Option<impl Into<Background>>, ) -> Span<'a, Link>
```

Sets the [`Background`] of the [`Span`](struct.Span.md), if any.

#### `border`

```rust
pub fn border(self, border: impl Into<Border>) -> Span<'a, Link>
```

Sets the [`Border`] of the [`Span`](struct.Span.md).

#### `border_maybe`

```rust
pub fn border_maybe(self, border: Option<impl Into<Border>>) -> Span<'a, Link>
```

Sets the [`Border`] of the [`Span`](struct.Span.md), if any.

#### `padding`

```rust
pub fn padding(self, padding: impl Into<Padding>) -> Span<'a, Link>
```

Sets the [`Padding`] of the [`Span`](struct.Span.md).

It only affects the [`background`](struct.Span.md) and [`border`](struct.Span.md) of the [`Span`](struct.Span.md), currently.

#### `underline`

```rust
pub fn underline(self, underline: bool) -> Span<'a, Link>
```

Sets whether the [`Span`](struct.Span.md) should be underlined or not.

#### `strikethrough`

```rust
pub fn strikethrough(self, strikethrough: bool) -> Span<'a, Link>
```

Sets whether the [`Span`](struct.Span.md) should be struck through or not.

#### `to_static`

```rust
pub fn to_static(self) -> Span<'static, Link>
```

Turns the [`Span`](struct.Span.md) into a static one.

## Trait Implementations

### `impl<'a, Link> Clone for Span<'a, Link> where Link: Clone,`

#### `clone`

```rust
fn clone(&self) -> Span<'a, Link>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<'a, Link> Debug for Span<'a, Link> where Link: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<Link> Default for Span<'_, Link>`

#### `default`

```rust
fn default() -> Span<'_, Link>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<'a, Link> From<&'a str> for Span<'a, Link>`

#### `from`

```rust
fn from(value: &'a str) -> Span<'a, Link>
```

Converts to this type from the input type.

### `impl<'a, Link, Message, Theme> FromIterator<Span<'a, Link>> for Rich<'a, Link, Message, Theme> where Link: Clone + 'a, Theme: Catalog,`

#### `from_iter`

```rust
fn from_iter<T: IntoIterator<Item = Span<'a, Link>>>(spans: T) -> Self
```

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

### `impl<Link> PartialEq for Span<'_, Link>`

#### `eq`

```rust
fn eq(&self, other: &Span<'_, Link>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations

- `impl<'a, Link> Freeze for Span<'a, Link> where Link: Freeze,`
- `impl<'a, Link> RefUnwindSafe for Span<'a, Link> where Link: RefUnwindSafe,`
- `impl<'a, Link> Send for Span<'a, Link> where Link: Send,`
- `impl<'a, Link> Sync for Span<'a, Link> where Link: Sync,`
- `impl<'a, Link> Unpin for Span<'a, Link> where Link: Unpin,`
- `impl<'a, Link> UnsafeUnpin for Span<'a, Link> where Link: UnsafeUnpin,`
- `impl<'a, Link> UnwindSafe for Span<'a, Link> where Link: UnwindSafe,`
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
