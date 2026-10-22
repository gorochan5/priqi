---
title: "TextEditor"
kind: struct
path: "iced_widget::text_editor::TextEditor"
crate: "iced_widget"
module: "iced_widget::text_editor"
file: "iced_widget/text_editor/struct.TextEditor.md"
docs_url: "https://docs.iced.rs/iced_widget/text_editor/struct.TextEditor.html"
source_file: "src/iced_widget/text_editor.rs"
source_lines: "91-113"
---

# `iced_widget::text_editor::TextEditor`

```rust
pub struct TextEditor<'a, Parser, Message, Theme = Theme, Renderer = Renderer> where
    Parser: Parser,
    Theme: Catalog,
    Renderer: Renderer, { /* private fields */ }
```

A multi-line text input.

## Example

```rust
use iced::widget::text_editor;

struct State {
   content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action)
}

fn view(state: &State) -> impl Widget<Message> {
    text_editor(&state.content)
        .placeholder("Type something here...")
        .on_action(Message::Edit)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Edit(action) => {
            state.content.perform(action);
        }
    }
}
```

**Source:** [text_editor.rs](../../src/iced_widget/text_editor.rs.md) (lines 91-113)

## Methods

### `impl<'a, Message, Theme, Renderer> TextEditor<'a, PlainText, Message, Theme, Renderer> where Theme: Catalog, Renderer: Renderer,`

#### `new`

```rust
pub fn new(content: &'a Content<Renderer>) -> Self
```

Creates new [`TextEditor`](struct.TextEditor.md) with the given [`Content`](struct.Content.md).

### `impl<'a, Message, Renderer> TextEditor<'a, PlainText, Message, Theme, Renderer> where Renderer: Renderer,`

#### `highlight`

```rust
pub fn highlight( self, syntax: &str, ) -> TextEditor<'a, Parser, Message, Theme, Renderer> where Renderer: Renderer,
```

*Available on crate feature highlighter only.*

Highlights the [`TextEditor`](struct.TextEditor.md) using the given syntax.

```rust
use iced::color;
use iced::widget::text;
use iced::widget::text_editor;
use iced::Theme;

struct State {
   content: text_editor::Content,
}

fn view(state: &State) -> impl Widget<()> {
    text_editor(&state.content)
        .highlight("rust")
}
```

### `impl<'a, Parser, Message, Theme, Renderer> TextEditor<'a, Parser, Message, Theme, Renderer> where Parser: Parser, Theme: Catalog, Renderer: Renderer,`

#### `id`

```rust
pub fn id(self, id: impl Into<Id>) -> Self
```

Sets the [`Id`](../struct.Id.md) of the [`TextEditor`](struct.TextEditor.md).

#### `placeholder`

```rust
pub fn placeholder(self, placeholder: impl IntoFragment<'a>) -> Self
```

Sets the placeholder of the [`TextEditor`](struct.TextEditor.md).

#### `width`

```rust
pub fn width(self, width: impl Into<Pixels>) -> Self
```

Sets the width of the [`TextEditor`](struct.TextEditor.md).

#### `height`

```rust
pub fn height(self, height: impl Into<Length>) -> Self
```

Sets the height of the [`TextEditor`](struct.TextEditor.md).

#### `on_action`

```rust
pub fn on_action(self, on_edit: impl Fn(Action) -> Message + 'a) -> Self
```

Sets the message that should be produced when some action is performed in the [`TextEditor`](struct.TextEditor.md).

If this method is not called, the [`TextEditor`](struct.TextEditor.md) will be disabled.

#### `font`

```rust
pub fn font(self, font: impl Into<Font>) -> Self
```

Sets the `Font` of the [`TextEditor`](struct.TextEditor.md).

#### `size`

```rust
pub fn size(self, size: impl Into<Pixels>) -> Self
```

Sets the text size of the [`TextEditor`](struct.TextEditor.md).

#### `line_height`

```rust
pub fn line_height(self, line_height: impl Into<LineHeight>) -> Self
```

Sets the [`text::LineHeight`](../text/enum.LineHeight.md) of the [`TextEditor`](struct.TextEditor.md).

#### `padding`

```rust
pub fn padding(self, padding: impl Into<Padding>) -> Self
```

Sets the [`Padding`] of the [`TextEditor`](struct.TextEditor.md).

#### `wrapping`

```rust
pub fn wrapping(self, wrapping: Wrapping) -> Self
```

Sets the [`Wrapping`](../text/enum.Wrapping.md) strategy of the [`TextEditor`](struct.TextEditor.md).

#### `highlight_with`

```rust
pub fn highlight_with<P: Parser>( self, settings: P::Settings, highlighter: impl Highlighter<P::Output, Theme> + 'a, ) -> TextEditor<'a, P, Message, Theme, Renderer>
```

Highlights the [`TextEditor`](struct.TextEditor.md) with the given [`text::Parser`](../text/trait.Parser.md) and [`text::Highlighter`](../text/trait.Highlighter.md).

#### `key_binding`

```rust
pub fn key_binding( self, key_binding: impl Fn(KeyPress) -> Option<Binding<Message>> + 'a, ) -> Self
```

Sets the closure to produce key bindings on key presses.

See [`Binding`](enum.Binding.md) for the list of available bindings.

#### `style`

```rust
pub fn style(self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the style of the [`TextEditor`](struct.TextEditor.md).

#### `class`

```rust
pub fn class(self, class: impl Into<Theme::Class<'a>>) -> Self
```

*Available on crate feature advanced only.*

Sets the style class of the [`TextEditor`](struct.TextEditor.md).

## Trait Implementations

### `impl<Parser, Message, Theme, Renderer> Meta for TextEditor<'_, Parser, Message, Theme, Renderer> where Parser: Parser, Theme: Catalog, Renderer: Renderer,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`] is [`Void`](../struct.Void.md).

### `impl<Parser, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TextEditor<'_, Parser, Message, Theme, Renderer> where Parser: Parser, Theme: Catalog, Renderer: Renderer,`

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

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, _renderer: &Renderer, shell: &mut Shell<'_, Message>, _viewport: &Rectangle, )
```

Processes a runtime [`Event`](../canvas/enum.Event.md). Read more

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, _defaults: &Style, layout: Layout, _cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`] using the associated `Renderer`.

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, _tree: &Tree, layout: Layout, cursor: Cursor, _viewport: &Rectangle, _renderer: &Renderer, ) -> Interaction
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

- `impl<'a, Parser, Message, Theme, Renderer> Freeze for TextEditor<'a, Parser, Message, Theme, Renderer> where <Theme as Catalog>::Class<'a>: Freeze, <Parser as Parser>::Settings: Freeze,`
- `impl<'a, Parser, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !RefUnwindSafe for TextEditor<'a, Parser, Message, Theme, Renderer>`
- `impl<'a, Parser, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Send for TextEditor<'a, Parser, Message, Theme, Renderer>`
- `impl<'a, Parser, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Sync for TextEditor<'a, Parser, Message, Theme, Renderer>`
- `impl<'a, Parser, Message, Theme, Renderer> Unpin for TextEditor<'a, Parser, Message, Theme, Renderer> where <Theme as Catalog>::Class<'a>: Unpin, <Parser as Parser>::Settings: Unpin,`
- `impl<'a, Parser, Message, Theme, Renderer> UnsafeUnpin for TextEditor<'a, Parser, Message, Theme, Renderer> where <Theme as Catalog>::Class<'a>: UnsafeUnpin, <Parser as Parser>::Settings: UnsafeUnpin,`
- `impl<'a, Parser, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !UnwindSafe for TextEditor<'a, Parser, Message, Theme, Renderer>`
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
