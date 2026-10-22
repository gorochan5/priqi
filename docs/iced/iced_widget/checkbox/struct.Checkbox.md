---
title: "Checkbox"
kind: struct
path: "iced_widget::checkbox::Checkbox"
crate: "iced_widget"
module: "iced_widget::checkbox"
file: "iced_widget/checkbox/struct.Checkbox.md"
docs_url: "https://docs.iced.rs/iced_widget/checkbox/struct.Checkbox.html"
source_file: "src/iced_widget/checkbox.rs"
source_lines: "82-102"
---

# `iced_widget::checkbox::Checkbox`

```rust
pub struct Checkbox<'a, Message, Theme = Theme, Renderer = Renderer> where
    Theme: Catalog,
    Renderer: Renderer, { /* private fields */ }
```

A box that can be checked.

## Example

```rust
use iced::widget::checkbox;

struct State {
   is_checked: bool,
}

enum Message {
    CheckboxToggled(bool),
}

fn view(state: &State) -> impl Widget<Message> {
    checkbox(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::CheckboxToggled)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::CheckboxToggled(is_checked) => {
            state.is_checked = is_checked;
        }
    }
}
```

![Checkbox drawn by iced_wgpu](https://github.com/iced-rs/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/checkbox.png?raw=true)

**Source:** [checkbox.rs](../../src/iced_widget/checkbox.rs.md) (lines 82-102)

## Methods

### `impl<'a, Message, Theme, Renderer> Checkbox<'a, Message, Theme, Renderer> where Renderer: Renderer, Theme: Catalog,`

#### `new`

```rust
pub fn new(is_checked: bool) -> Self
```

Creates a new [`Checkbox`](struct.Checkbox.md).

It expects:

- a boolean describing whether the [`Checkbox`](struct.Checkbox.md) is checked or not

#### `label`

```rust
pub fn label(self, label: impl IntoFragment<'a>) -> Self
```

Sets the label of the [`Checkbox`](struct.Checkbox.md).

#### `on_toggle`

```rust
pub fn on_toggle<F>(self, f: F) -> Self where F: 'a + Fn(bool) -> Message,
```

Sets the function that will be called when the [`Checkbox`](struct.Checkbox.md) is toggled. It will receive the new state of the [`Checkbox`](struct.Checkbox.md) and must produce a `Message`.

Unless `on_toggle` is called, the [`Checkbox`](struct.Checkbox.md) will be disabled.

#### `on_toggle_maybe`

```rust
pub fn on_toggle_maybe<F>(self, f: Option<F>) -> Self where F: Fn(bool) -> Message + 'a,
```

Sets the function that will be called when the [`Checkbox`](struct.Checkbox.md) is toggled, if `Some`.

If `None`, the checkbox will be disabled.

#### `size`

```rust
pub fn size(self, size: impl Into<Pixels>) -> Self
```

Sets the size of the [`Checkbox`](struct.Checkbox.md).

#### `width`

```rust
pub fn width(self, width: impl Into<Length>) -> Self
```

Sets the width of the [`Checkbox`](struct.Checkbox.md).

#### `spacing`

```rust
pub fn spacing(self, spacing: impl Into<Pixels>) -> Self
```

Sets the spacing between the [`Checkbox`](struct.Checkbox.md) and the text.

#### `text_size`

```rust
pub fn text_size(self, text_size: impl Into<Pixels>) -> Self
```

Sets the text size of the [`Checkbox`](struct.Checkbox.md).

#### `line_height`

```rust
pub fn line_height(self, line_height: impl Into<LineHeight>) -> Self
```

Sets the text [`text::LineHeight`](../text/enum.LineHeight.md) of the [`Checkbox`](struct.Checkbox.md).

#### `shaping`

```rust
pub fn shaping(self, shaping: Shaping) -> Self
```

Sets the [`text::Shaping`](../text/enum.Shaping.md) strategy of the [`Checkbox`](struct.Checkbox.md).

#### `wrapping`

```rust
pub fn wrapping(self, wrapping: Wrapping) -> Self
```

Sets the [`text::Wrapping`](../text/enum.Wrapping.md) strategy of the [`Checkbox`](struct.Checkbox.md).

#### `font`

```rust
pub fn font(self, font: impl Into<Font>) -> Self
```

Sets the `Font` of the text of the [`Checkbox`](struct.Checkbox.md).

#### `icon`

```rust
pub fn icon(self, icon: Icon) -> Self
```

Sets the [`Icon`](struct.Icon.md) of the [`Checkbox`](struct.Checkbox.md).

#### `style`

```rust
pub fn style(self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the style of the [`Checkbox`](struct.Checkbox.md).

#### `class`

```rust
pub fn class(self, class: impl Into<Theme::Class<'a>>) -> Self
```

*Available on crate feature advanced only.*

Sets the style class of the [`Checkbox`](struct.Checkbox.md).

## Trait Implementations

### `impl<Message, Theme, Renderer> Meta for Checkbox<'_, Message, Theme, Renderer> where Theme: Catalog, Renderer: Renderer,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`] is [`Void`](../struct.Void.md).

### `impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Checkbox<'_, Message, Theme, Renderer> where Renderer: Renderer, Theme: Catalog,`

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

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`] with the provided [`Tree`].

#### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`]. Read more

#### `update`

```rust
fn update( &mut self, _tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, _renderer: &Renderer, shell: &mut Shell<'_, Message>, _viewport: &Rectangle, )
```

Processes a runtime [`Event`](../canvas/enum.Event.md). Read more

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, _tree: &Tree, layout: Layout, cursor: Cursor, _viewport: &Rectangle, _renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`] of the [`Widget`]. Read more

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, defaults: &Style, layout: Layout, _cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`] using the associated `Renderer`.

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, _viewport: &Rectangle, _renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`] to the [`Widget`].

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

- `impl<'a, Message, Theme, Renderer> Freeze for Checkbox<'a, Message, Theme, Renderer> where <Theme as Catalog>::Class<'a>: Freeze,`
- `impl<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !RefUnwindSafe for Checkbox<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Send for Checkbox<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Sync for Checkbox<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> Unpin for Checkbox<'a, Message, Theme, Renderer> where <Theme as Catalog>::Class<'a>: Unpin, Renderer: Unpin,`
- `impl<'a, Message, Theme, Renderer> UnsafeUnpin for Checkbox<'a, Message, Theme, Renderer> where <Theme as Catalog>::Class<'a>: UnsafeUnpin,`
- `impl<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !UnwindSafe for Checkbox<'a, Message, Theme, Renderer>`
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
