---
title: "Rich"
kind: struct
path: "iced_widget::text::Rich"
crate: "iced_widget"
module: "iced_widget::text"
file: "iced_widget/text/struct.Rich.md"
docs_url: "https://docs.iced.rs/iced_widget/text/struct.Rich.html"
source_file: "src/iced_widget/text/rich.rs"
source_lines: "20-40"
---

# `iced_widget::text::Rich`

```rust
pub struct Rich<'a, Link, Message, Theme = Theme> where
    Link: Clone + 'static,
    Theme: Catalog, { /* private fields */ }
```

A bunch of [`Rich`](struct.Rich.md) text.

**Source:** [rich.rs](../../src/iced_widget/text/rich.rs.md) (lines 20-40)

## Methods

### `impl<'a, Link, Message, Theme> Rich<'a, Link, Message, Theme> where Link: Clone + 'static, Theme: Catalog,`

#### `new`

```rust
pub fn new() -> Self
```

Creates a new empty [`Rich`](struct.Rich.md) text.

#### `with_spans`

```rust
pub fn with_spans(spans: impl AsRef<[Span<'a, Link>]> + 'a) -> Self
```

Creates a new [`Rich`](struct.Rich.md) text with the given text spans.

#### `size`

```rust
pub fn size(self, size: impl Into<Pixels>) -> Self
```

Sets the default size of the [`Rich`](struct.Rich.md) text.

#### `line_height`

```rust
pub fn line_height(self, line_height: impl Into<LineHeight>) -> Self
```

Sets the default [`LineHeight`](enum.LineHeight.md) of the [`Rich`](struct.Rich.md) text.

#### `font`

```rust
pub fn font(self, font: impl Into<Font>) -> Self
```

Sets the default font of the [`Rich`](struct.Rich.md) text.

#### `width`

```rust
pub fn width(self, width: impl Into<Length>) -> Self
```

Sets the width of the [`Rich`](struct.Rich.md) text boundaries.

#### `height`

```rust
pub fn height(self, height: impl Into<Length>) -> Self
```

Sets the height of the [`Rich`](struct.Rich.md) text boundaries.

#### `center`

```rust
pub fn center(self) -> Self
```

Centers the [`Rich`](struct.Rich.md) text, both horizontally and vertically.

#### `align_x`

```rust
pub fn align_x(self, alignment: impl Into<Alignment>) -> Self
```

Sets the [`alignment::Horizontal`] of the [`Rich`](struct.Rich.md) text.

#### `align_y`

```rust
pub fn align_y(self, alignment: impl Into<Vertical>) -> Self
```

Sets the [`alignment::Vertical`] of the [`Rich`](struct.Rich.md) text.

#### `wrapping`

```rust
pub fn wrapping(self, wrapping: Wrapping) -> Self
```

Sets the [`Wrapping`](enum.Wrapping.md) strategy of the [`Rich`](struct.Rich.md) text.

#### `ellipsis`

```rust
pub fn ellipsis(self, ellipsis: Ellipsis) -> Self
```

Sets the [`Ellipsis`](enum.Ellipsis.md) strategy of the [`Rich`](struct.Rich.md) text.

#### `on_link_click`

```rust
pub fn on_link_click(self, on_link_click: impl Fn(Link) -> Message + 'a) -> Self
```

Sets the message that will be produced when a link of the [`Rich`](struct.Rich.md) text is clicked.

If the spans of the [`Rich`](struct.Rich.md) text contain no links, you may need to call this method with `on_link_click(never)` in order for the compiler to infer the proper `Link` generic type.

#### `selectable`

```rust
pub fn selectable(self, selectable: bool) -> Self
```

Sets whether the [`Rich`](struct.Rich.md) text can be selected.

By default, it is `false`.

#### `style`

```rust
pub fn style(self, style: impl Fn(&Theme) -> Style + 'a) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the default style of the [`Rich`](struct.Rich.md) text.

#### `color`

```rust
pub fn color(self, color: impl Into<Color>) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the default [`Color`] of the [`Rich`](struct.Rich.md) text.

#### `color_maybe`

```rust
pub fn color_maybe(self, color: Option<impl Into<Color>>) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the default [`Color`] of the [`Rich`](struct.Rich.md) text, if `Some`.

#### `class`

```rust
pub fn class(self, class: impl Into<Theme::Class<'a>>) -> Self
```

*Available on crate feature advanced only.*

Sets the default style class of the [`Rich`](struct.Rich.md) text.

## Trait Implementations

### `impl<'a, Link, Message, Theme> Default for Rich<'a, Link, Message, Theme> where Link: Clone + 'a, Theme: Catalog,`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<'a, Link, Message, Theme> FromIterator<Span<'a, Link>> for Rich<'a, Link, Message, Theme> where Link: Clone + 'a, Theme: Catalog,`

#### `from_iter`

```rust
fn from_iter<T: IntoIterator<Item = Span<'a, Link>>>(spans: T) -> Self
```

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

### `impl<Link, Message, Theme> Meta for Rich<'_, Link, Message, Theme> where Link: Clone + 'static, Theme: Catalog,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`] is [`Void`](../struct.Void.md).

### `impl<Link, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Rich<'_, Link, Message, Theme> where Link: Clone + 'static, Theme: Catalog, Renderer: Renderer,`

#### `tag`

```rust
fn tag(&self) -> Tag
```

Returns the `Tag` of the [`Widget`].

#### `state`

```rust
fn state(&self) -> State
```

Returns the `State` of the [`Widget`].

#### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`] of the [`Widget`] in lengths.

#### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`]. Read more

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, defaults: &Style, layout: Layout, _cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`] using the associated `Renderer`.

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, _renderer: &Renderer, shell: &mut Shell<'_, Message>, _viewport: &Rectangle, )
```

Processes a runtime [`Event`](../canvas/enum.Event.md). Read more

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, _tree: &Tree, _layout: Layout, _cursor: Cursor, _viewport: &Rectangle, _renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`] of the [`Widget`]. Read more

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, _viewport: &Rectangle, _renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`] to the [`Widget`].

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`] with the provided [`Tree`].

#### `overlay`

```rust
fn overlay<'a>( &'a mut self, _tree: &'a mut Tree, _layout: Layout, _renderer: &Renderer, _viewport: &Rectangle, _translation: Vector, _window: Size, ) -> Vec<Element<'a, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`].

#### `_map`

```rust
fn _map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`], returning a [`Map`]. Read more

#### `_boxed`

```rust
fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`], turning it into a generic [`Element`].

## Auto Trait Implementations

- `impl<'a, Link, Message, Theme> Freeze for Rich<'a, Link, Message, Theme> where <Theme as Catalog>::Class<'a>: Freeze,`
- `impl<'a, Link, Message, Theme = Theme> !RefUnwindSafe for Rich<'a, Link, Message, Theme>`
- `impl<'a, Link, Message, Theme = Theme> !Send for Rich<'a, Link, Message, Theme>`
- `impl<'a, Link, Message, Theme = Theme> !Sync for Rich<'a, Link, Message, Theme>`
- `impl<'a, Link, Message, Theme> Unpin for Rich<'a, Link, Message, Theme> where <Theme as Catalog>::Class<'a>: Unpin,`
- `impl<'a, Link, Message, Theme> UnsafeUnpin for Rich<'a, Link, Message, Theme> where <Theme as Catalog>::Class<'a>: UnsafeUnpin,`
- `impl<'a, Link, Message, Theme = Theme> !UnwindSafe for Rich<'a, Link, Message, Theme>`
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

### `impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for T where T: Widget<Message, Theme, Renderer>,`

#### `map`

```rust
fn map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](../trait.Widget.md). [Read more](../trait.Widget.md)

#### `boxed`

```rust
fn boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](../trait.Widget.md), turning it into a generic [`Element`](../type.Element.md).

#### `explain`

```rust
fn explain( self, color: impl Into<Color>, ) -> impl Widget<Message, Theme, Renderer> where Self: Sized, Renderer: Renderer,
```

Marks this [`Widget`](../trait.Widget.md) as *to-be-explained*. [Read more](../trait.Widget.md)

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

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
