---
title: "Scrollable"
kind: struct
path: "iced_widget::scrollable::Scrollable"
crate: "iced_widget"
module: "iced_widget::scrollable"
file: "iced_widget/scrollable/struct.Scrollable.md"
docs_url: "https://docs.iced.rs/iced_widget/scrollable/struct.Scrollable.html"
source_file: "src/iced_widget/scrollable.rs"
source_lines: "83-97"
---

# `iced_widget::scrollable::Scrollable`

```rust
pub struct Scrollable<'a, Message, W = Element<'a, Message>, Theme = Theme> where
    Theme: Catalog, { /* private fields */ }
```

A widget that can vertically display an infinite amount of content with a scrollbar.

## Example

```rust
use iced::widget::{column, scrollable, space};

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    scrollable(column![
        "Scroll me!",
        space().height(3000),
        "You did it!",
    ])
}
```

## Scrollbars

A scrollbar has two interactive parts: the *scroller* (the draggable thumb) and the *rail* (the track it slides on).

- Pressing and dragging the **scroller** moves it 1:1 with the pointer.
- Pressing the **rail** scrolls one page (animated like a wheel scroll when [`Self::smooth_scroll`](struct.Scrollable.md) is enabled). While the button is held, the scroller then slides toward the pointer at a constant speed, stopping when its edge reaches the pointer; moving the pointer moves the target with it. This mirrors the track autoscroll of most toolkits.
- `Shift`-clicking the **rail** instead jumps the scroller so that its center is under the pointer, and then drags it from there.

With [`Self::click_to_scroll`](struct.Scrollable.md) enabled, the behavior of the rail press and the `Shift`-click is inverted.

**Source:** [scrollable.rs](../../src/iced_widget/scrollable.rs.md) (lines 83-97)

## Methods

### `impl<'a, Message, W, Theme> Scrollable<'a, Message, W, Theme> where Theme: Catalog,`

#### `new`

```rust
pub fn new(content: W) -> Self
```

Creates a new vertical [`Scrollable`](struct.Scrollable.md).

#### `with_direction`

```rust
pub fn with_direction(content: W, direction: impl Into<Direction>) -> Self
```

Creates a new [`Scrollable`](struct.Scrollable.md) with the given [`Direction`](enum.Direction.md).

#### `horizontal`

```rust
pub fn horizontal(self) -> Self
```

Makes the [`Scrollable`](struct.Scrollable.md) scroll horizontally, with default [`Scrollbar`](struct.Scrollbar.md) settings.

#### `direction`

```rust
pub fn direction(self, direction: impl Into<Direction>) -> Self
```

Sets the [`Direction`](enum.Direction.md) of the [`Scrollable`](struct.Scrollable.md).

#### `id`

```rust
pub fn id(self, id: impl Into<Id>) -> Self
```

Sets the [`widget::Id`](../struct.Id.md) of the [`Scrollable`](struct.Scrollable.md).

#### `width`

```rust
pub fn width(self, width: impl Into<Length>) -> Self
```

Sets the width of the [`Scrollable`](struct.Scrollable.md).

#### `height`

```rust
pub fn height(self, height: impl Into<Length>) -> Self
```

Sets the height of the [`Scrollable`](struct.Scrollable.md).

#### `on_scroll`

```rust
pub fn on_scroll<T>(self, f: impl Fn(Scroll) -> T + 'a) -> Self where T: Into<Action<Message>>,
```

Sets a handler to call when the [`Scrollable`](struct.Scrollable.md) is scrolled.

The function takes a [`Scroll`](struct.Scroll.md), which contains the [`Viewport`](struct.Viewport.md) of the [`Scrollable`](struct.Scrollable.md) and the [`Source`](enum.Source.md) of the scroll that caused the notification.

#### `anchor_top`

```rust
pub fn anchor_top(self) -> Self
```

Anchors the vertical [`Scrollable`](struct.Scrollable.md) direction to the top.

#### `anchor_bottom`

```rust
pub fn anchor_bottom(self) -> Self
```

Anchors the vertical [`Scrollable`](struct.Scrollable.md) direction to the bottom.

#### `anchor_left`

```rust
pub fn anchor_left(self) -> Self
```

Anchors the horizontal [`Scrollable`](struct.Scrollable.md) direction to the left.

#### `anchor_right`

```rust
pub fn anchor_right(self) -> Self
```

Anchors the horizontal [`Scrollable`](struct.Scrollable.md) direction to the right.

#### `anchor_x`

```rust
pub fn anchor_x(self, alignment: Anchor) -> Self
```

Sets the [`Anchor`](enum.Anchor.md) of the horizontal direction of the [`Scrollable`](struct.Scrollable.md), if applicable.

#### `anchor_y`

```rust
pub fn anchor_y(self, alignment: Anchor) -> Self
```

Sets the [`Anchor`](enum.Anchor.md) of the vertical direction of the [`Scrollable`](struct.Scrollable.md), if applicable.

#### `spacing`

```rust
pub fn spacing(self, new_spacing: impl Into<Pixels>) -> Self
```

Embeds the [`Scrollbar`](struct.Scrollbar.md) into the [`Scrollable`](struct.Scrollable.md), instead of floating on top of the content.

The `spacing` provided will be used as space between the [`Scrollbar`](struct.Scrollbar.md) and the contents of the [`Scrollable`](struct.Scrollable.md).

#### `padding`

```rust
pub fn padding(self, new_padding: impl Into<Pixels>) -> Self
```

Adds padding at the ends of the [`Scrollbar`](struct.Scrollbar.md)s of the [`Scrollable`](struct.Scrollable.md).

The `padding` provided will be used as space at the top and bottom of a vertical [`Scrollbar`](struct.Scrollbar.md), and at the left and right ends of a horizontal [`Scrollbar`](struct.Scrollbar.md), when they are visible.

Unlike [`Self::spacing`](struct.Scrollable.md), the padding does not affect the layout of the [`Scrollable`](struct.Scrollable.md).

#### `auto_scroll`

```rust
pub fn auto_scroll(self, auto_scroll: bool) -> Self
```

Sets whether the user should be allowed to auto-scroll the [`Scrollable`](struct.Scrollable.md) with the middle mouse button.

By default, it is disabled.

#### `smooth_scroll`

```rust
pub fn smooth_scroll(self, smooth_scroll: bool) -> Self
```

Sets whether wheel scrolling should be smoothed out over time, instead of moving the [`Scrollable`](struct.Scrollable.md) immediately.

When enabled, discrete scrolls (e.g. from a mouse wheel) move a target scroll offset — carrying the momentum of the wheel notches — and the [`Scrollable`](struct.Scrollable.md) eases towards it over a few frames. High-precision scrolls (e.g. from a touchpad), which are already smooth, are always applied immediately.

The setting also acts as the default behavior of the scroll operations (`snap_to`, `scroll_to`, `scroll_by`): operations with [`Animation::Auto`] scroll smoothly if and only if it is enabled.

By default, it is enabled.

#### `click_to_scroll`

```rust
pub fn click_to_scroll(self, click_to_scroll: bool) -> Self
```

Sets whether a plain click on the scrollbar rail should jump the scroller to the click position, instead of scrolling one page.

By default, a press on the rail scrolls one page (and autoscrolls while the button is held), while a `Shift`-click jumps the scroller so that its center is under the pointer and drags it from there.

When enabled, the behavior is inverted: a plain click jumps the scroller so that its center is under the pointer (and drags it from there), while a `Shift`-click scrolls one page (and autoscrolls while the button is held).

By default, it is disabled.

#### `style`

```rust
pub fn style(self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the style of this [`Scrollable`](struct.Scrollable.md).

#### `class`

```rust
pub fn class(self, class: impl Into<Theme::Class<'a>>) -> Self
```

*Available on crate feature advanced only.*

Sets the style class of the [`Scrollable`](struct.Scrollable.md).

## Trait Implementations

### `impl<Message, W, Theme> Meta for Scrollable<'_, Message, W, Theme> where Theme: Catalog,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`] is [`Void`](../struct.Void.md).

### `impl<Message, W, Theme, Renderer> Widget<Message, Theme, Renderer> for Scrollable<'_, Message, W, Theme> where Theme: Catalog, Renderer: Renderer, W: Widget<Message, Theme, Renderer>,`

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

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`] with the provided [`Tree`].

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

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, viewport: &Rectangle, renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`] to the [`Widget`].

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, renderer: &Renderer, shell: &mut Shell<'_, Message>, viewport: &Rectangle, )
```

Processes a runtime [`Event`](../canvas/enum.Event.md). Read more

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, defaults: &Style, layout: Layout, cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`] using the associated `Renderer`.

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, tree: &Tree, layout: Layout, cursor: Cursor, viewport: &Rectangle, renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`] of the [`Widget`]. Read more

#### `overlay`

```rust
fn overlay<'b>( &'b mut self, tree: &'b mut Tree, layout: Layout, renderer: &Renderer, viewport: &Rectangle, translation: Vector, window: Size, ) -> Vec<Element<'b, Message, Theme, Renderer>>
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

- `impl<'a, Message, W, Theme> Freeze for Scrollable<'a, Message, W, Theme> where W: Freeze, <Theme as Catalog>::Class<'a>: Freeze,`
- `impl<'a, Message, W = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>, Theme = Theme> !RefUnwindSafe for Scrollable<'a, Message, W, Theme>`
- `impl<'a, Message, W = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>, Theme = Theme> !Send for Scrollable<'a, Message, W, Theme>`
- `impl<'a, Message, W = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>, Theme = Theme> !Sync for Scrollable<'a, Message, W, Theme>`
- `impl<'a, Message, W, Theme> Unpin for Scrollable<'a, Message, W, Theme> where W: Unpin, <Theme as Catalog>::Class<'a>: Unpin,`
- `impl<'a, Message, W, Theme> UnsafeUnpin for Scrollable<'a, Message, W, Theme> where W: UnsafeUnpin, <Theme as Catalog>::Class<'a>: UnsafeUnpin,`
- `impl<'a, Message, W = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>, Theme = Theme> !UnwindSafe for Scrollable<'a, Message, W, Theme>`
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
