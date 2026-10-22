---
title: "Image"
kind: struct
path: "iced::widget::Image"
crate: "iced"
module: "iced::widget"
file: "iced/widget/struct.Image.md"
docs_url: "https://docs.iced.rs/iced/widget/struct.Image.html"
source_file: "src/iced_widget/image.rs"
source_lines: "55"
---

# `iced::widget::Image`

```rust
pub struct Image<Handle = Handle> { /* private fields */ }
```

A frame that displays an image while keeping aspect ratio.

## Example

```rust
use iced::widget::image;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    image("ferris.png")
}
```

![](https://github.com/iced-rs/iced/blob/9712b319bb7a32848001b96bd84977430f14b623/examples/resources/ferris.png?raw=true)

**Source:** [image.rs](../../src/iced_widget/image.rs.md) (lines 55)

## Methods

### `impl<Handle> Image<Handle>`

#### `new`

```rust
pub fn new(handle: impl Into<Handle>) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Creates a new [`Image`](struct.Image.md) with the given path.

#### `width`

```rust
pub fn width(self, width: impl Into<Length>) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Sets the width of the [`Image`](struct.Image.md) boundaries.

#### `height`

```rust
pub fn height(self, height: impl Into<Length>) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Sets the height of the [`Image`](struct.Image.md) boundaries.

#### `expand`

```rust
pub fn expand(self, expand: bool) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Sets whether the [`Image`](struct.Image.md) should try to fill as much space available as possible while keeping aspect ratio and without allocating extra space in any axis with a [`Length::Shrink`](../enum.Length.md) sizing strategy.

This is similar to using [`Length::Fill`](../enum.Length.md) for both the [`width`](struct.Image.md) and the [`height`](struct.Image.md), but without the downside of blank space.

#### `content_fit`

```rust
pub fn content_fit(self, content_fit: ContentFit) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Sets the [`ContentFit`](../enum.ContentFit.md) of the [`Image`](struct.Image.md).

Defaults to [`ContentFit::Contain`](../enum.ContentFit.md)

#### `filter_method`

```rust
pub fn filter_method(self, filter_method: FilterMethod) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Sets the [`FilterMethod`](../advanced/image/enum.FilterMethod.md) of the [`Image`](struct.Image.md).

#### `rotation`

```rust
pub fn rotation(self, rotation: impl Into<Rotation>) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Applies the given [`Rotation`](../enum.Rotation.md) to the [`Image`](struct.Image.md).

#### `opacity`

```rust
pub fn opacity(self, opacity: impl Into<f32>) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Sets the opacity of the [`Image`](struct.Image.md).

It should be in the [0.0, 1.0] range—`0.0` meaning completely transparent, and `1.0` meaning completely opaque.

#### `scale`

```rust
pub fn scale(self, scale: impl Into<f32>) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Sets the scale of the [`Image`](struct.Image.md).

The region of the [`Image`](struct.Image.md) drawn will be scaled from the center by the given scale factor. This can be useful to create certain effects and animations, like smooth zoom in / out.

#### `crop`

```rust
pub fn crop(self, region: Rectangle<u32>) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Crops the [`Image`](struct.Image.md) to the given region described by the [`Rectangle`](../struct.Rectangle.md) in absolute coordinates.

Cropping is done before applying any transformation or [`ContentFit`](../enum.ContentFit.md). In practice, this means that cropping an [`Image`](struct.Image.md) with this method should produce the same result as cropping it externally (e.g. with an image editor) and creating a new [`Handle`](../advanced/image/enum.Handle.md) for the cropped version.

However, this method is much more efficient; since it just leverages scissoring during rendering and no image cropping actually takes place. Instead, it reuses the existing image allocations and should be as efficient as not cropping at all!

The `region` coordinates will be clamped to the image dimensions, if necessary.

#### `border_radius`

```rust
pub fn border_radius(self, border_radius: impl Into<Radius>) -> Image<Handle>
```

*Available on crate feature image-without-codecs only.*

Sets the [`border::Radius`](../border/struct.Radius.md) of the [`Image`](struct.Image.md).

Currently, it will only be applied around the rectangular bounding box of the [`Image`](struct.Image.md).

## Trait Implementations

### `impl<Handle> Meta for Image<Handle>`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`](../advanced/trait.Widget.md) is [`Void`](struct.Void.md).

### `impl<Message, Theme, Renderer, Handle> Widget<Message, Theme, Renderer> for Image<Handle> where Renderer: Renderer<Handle = Handle>, Handle: Clone,`

#### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`](../struct.Size.md) of the [`Widget`](../advanced/trait.Widget.md) in lengths.

#### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`](../advanced/trait.Widget.md). [Read more](../advanced/trait.Widget.md)

#### `draw`

```rust
fn draw( &self, _tree: &Tree, renderer: &mut Renderer, _theme: &Theme, _style: &Style, layout: Layout, _cursor: Cursor, _viewport: &Rectangle, )
```

Draws the [`Widget`](../advanced/trait.Widget.md) using the associated `Renderer`.

#### `tag`

```rust
fn tag(&self) -> Tag
```

Returns the [`Tag`](../advanced/widget/tree/struct.Tag.md) of the [`Widget`](../advanced/trait.Widget.md).

#### `state`

```rust
fn state(&self) -> State
```

Returns the [`State`](../advanced/widget/tree/enum.State.md) of the [`Widget`](../advanced/trait.Widget.md).

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`](../advanced/trait.Widget.md) with the provided [`Tree`](../advanced/widget/struct.Tree.md).

#### `operate`

```rust
fn operate( &mut self, _tree: &mut Tree, _layout: Layout, _viewport: &Rectangle, _renderer: &Renderer, _operation: &mut dyn Operation, )
```

Applies an [`Operation`](../advanced/widget/trait.Operation.md) to the [`Widget`](../advanced/trait.Widget.md).

#### `update`

```rust
fn update( &mut self, _tree: &mut Tree, _event: &Event, _layout: Layout, _cursor: Cursor, _renderer: &Renderer, _shell: &mut Shell<'_, Message>, _viewport: &Rectangle, )
```

Processes a runtime [`Event`](../enum.Event.md). [Read more](../advanced/trait.Widget.md)

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, _tree: &Tree, _layout: Layout, _cursor: Cursor, _viewport: &Rectangle, _renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../mouse/enum.Interaction.md) of the [`Widget`](../advanced/trait.Widget.md). [Read more](../advanced/trait.Widget.md)

#### `overlay`

```rust
fn overlay<'a>( &'a mut self, _tree: &'a mut Tree, _layout: Layout, _renderer: &Renderer, _viewport: &Rectangle, _translation: Vector, _window: Size, ) -> Vec<Element<'a, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`](../advanced/trait.Widget.md).

#### `_map`

```rust
fn _map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](../advanced/trait.Widget.md), returning a [`Map`](../advanced/widget/struct.Map.md). [Read more](../advanced/trait.Widget.md)

#### `_boxed`

```rust
fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](../advanced/trait.Widget.md), turning it into a generic [`Element`](../advanced/widget/struct.Element.md).

## Auto Trait Implementations

- `impl<Handle> Freeze for Image<Handle> where Handle: Freeze,`
- `impl<Handle> RefUnwindSafe for Image<Handle> where Handle: RefUnwindSafe,`
- `impl<Handle> Send for Image<Handle> where Handle: Send,`
- `impl<Handle> Sync for Image<Handle> where Handle: Sync,`
- `impl<Handle> Unpin for Image<Handle> where Handle: Unpin,`
- `impl<Handle> UnsafeUnpin for Image<Handle> where Handle: UnsafeUnpin,`
- `impl<Handle> UnwindSafe for Image<Handle> where Handle: UnwindSafe,`
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

### `impl<State, Message> IntoBoot<State, Message> for State`

#### `into_boot`

```rust
fn into_boot(self) -> (State, Task<Message>)
```

Turns some type into the initial state of some [`Application`](../application/struct.Application.md).

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

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
