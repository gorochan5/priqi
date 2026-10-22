---
title: "PickList"
kind: struct
path: "iced::widget::PickList"
crate: "iced"
module: "iced::widget"
file: "iced/widget/struct.PickList.md"
docs_url: "https://docs.iced.rs/iced/widget/struct.PickList.html"
source_file: "src/iced_widget/pick_list.rs"
source_lines: "146"
---

# `iced::widget::PickList`

```rust
pub struct PickList<'a, T, L, V, Message, Theme = Theme> where
    T: PartialEq + Clone,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Theme: Catalog, { /* private fields */ }
```

A widget for selecting a single value from a list of options.

## Example

```rust
use iced::widget::pick_list;

struct State {
   favorite: Option<Fruit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Orange,
    Strawberry,
    Tomato,
}

#[derive(Debug, Clone)]
enum Message {
    FruitSelected(Fruit),
}

fn view(state: &State) -> impl Widget<Message> {
    let fruits = [
        Fruit::Apple,
        Fruit::Orange,
        Fruit::Strawberry,
        Fruit::Tomato,
    ];

    pick_list(
        state.favorite,
        fruits,
        Fruit::to_string,
    )
    .on_select(Message::FruitSelected)
    .placeholder("Select your favorite fruit...")
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::FruitSelected(fruit) => {
            state.favorite = Some(fruit);
        }
    }
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Apple => "Apple",
            Self::Orange => "Orange",
            Self::Strawberry => "Strawberry",
            Self::Tomato => "Tomato",
        })
    }
}
```

**Source:** [pick_list.rs](../../src/iced_widget/pick_list.rs.md) (lines 146)

## Methods

### `impl<'a, T, L, V, Message, Theme> PickList<'a, T, L, V, Message, Theme> where T: PartialEq + Clone, L: Borrow<[T]> + 'a, V: Borrow<T> + 'a, Message: Clone, Theme: Catalog,`

#### `new`

```rust
pub fn new( selected: Option<V>, options: L, to_string: impl Fn(&T) -> String + 'a, ) -> PickList<'a, T, L, V, Message, Theme>
```

Creates a new [`PickList`](struct.PickList.md) with the given list of options, the current selected value, and the message to produce when an option is selected.

#### `placeholder`

```rust
pub fn placeholder( self, placeholder: impl Into<String>, ) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the placeholder of the [`PickList`](struct.PickList.md).

#### `width`

```rust
pub fn width( self, width: impl Into<Length>, ) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the width of the [`PickList`](struct.PickList.md).

#### `menu_height`

```rust
pub fn menu_height( self, menu_height: impl Into<Length>, ) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the height of the [`Menu`](../overlay/menu/struct.Menu.md).

#### `padding`

```rust
pub fn padding<P>(self, padding: P) -> PickList<'a, T, L, V, Message, Theme> where P: Into<Padding>,
```

Sets the [`Padding`](../struct.Padding.md) of the [`PickList`](struct.PickList.md).

#### `text_size`

```rust
pub fn text_size( self, size: impl Into<Pixels>, ) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the text size of the [`PickList`](struct.PickList.md).

#### `line_height`

```rust
pub fn line_height( self, line_height: impl Into<LineHeight>, ) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the text [`text::LineHeight`](../advanced/text/enum.LineHeight.md) of the [`PickList`](struct.PickList.md).

#### `shaping`

```rust
pub fn shaping(self, shaping: Shaping) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the [`text::Shaping`](../advanced/text/enum.Shaping.md) strategy of the [`PickList`](struct.PickList.md).

#### `ellipsis`

```rust
pub fn ellipsis( self, ellipsis: Ellipsis, ) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the [`text::Ellipsis`](../advanced/text/enum.Ellipsis.md) strategy of the [`PickList`](struct.PickList.md).

#### `font`

```rust
pub fn font( self, font: impl Into<Font>, ) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the font of the [`PickList`](struct.PickList.md).

#### `handle`

```rust
pub fn handle(self, handle: Handle) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the [`Handle`](pick_list/enum.Handle.md) of the [`PickList`](struct.PickList.md).

#### `on_select`

```rust
pub fn on_select( self, on_select: impl Fn(T) -> Message + 'a, ) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the message that will be produced when the [`PickList`](struct.PickList.md) selected value changes.

#### `on_open`

```rust
pub fn on_open(self, on_open: Message) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the message that will be produced when the [`PickList`](struct.PickList.md) is opened.

#### `on_close`

```rust
pub fn on_close( self, on_close: Message, ) -> PickList<'a, T, L, V, Message, Theme>
```

Sets the message that will be produced when the [`PickList`](struct.PickList.md) is closed.

#### `style`

```rust
pub fn style( self, style: impl Fn(&Theme, Status) -> Style + 'a, ) -> PickList<'a, T, L, V, Message, Theme> where <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme, Status) -> Style + 'a>>,
```

Sets the style of the [`PickList`](struct.PickList.md).

#### `menu_style`

```rust
pub fn menu_style( self, style: impl Fn(&Theme) -> Style + 'a, ) -> PickList<'a, T, L, V, Message, Theme> where <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme) -> Style + 'a>>,
```

Sets the style of the [`Menu`](../overlay/menu/struct.Menu.md).

#### `class`

```rust
pub fn class( self, class: impl Into<<Theme as Catalog>::Class<'a>>, ) -> PickList<'a, T, L, V, Message, Theme>
```

*Available on crate feature advanced only.*

Sets the style class of the [`PickList`](struct.PickList.md).

#### `menu_class`

```rust
pub fn menu_class( self, class: impl Into<<Theme as Catalog>::Class<'a>>, ) -> PickList<'a, T, L, V, Message, Theme>
```

*Available on crate feature advanced only.*

Sets the style class of the [`Menu`](../overlay/menu/struct.Menu.md).

## Trait Implementations

### `impl<'a, T, L, V, Message, Theme> Meta for PickList<'a, T, L, V, Message, Theme> where T: PartialEq + Clone, L: Borrow<[T]> + 'a, V: Borrow<T> + 'a, Theme: Catalog,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`](../advanced/trait.Widget.md) is [`Void`](struct.Void.md).

### `impl<'a, T, L, V, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for PickList<'a, T, L, V, Message, Theme> where T: Clone + PartialEq + 'a, L: Borrow<[T]>, V: Borrow<T>, Message: Clone + 'a, Theme: Catalog + 'a, Renderer: Renderer + 'a,`

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

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, _renderer: &Renderer, shell: &mut Shell<'_, Message>, _viewport: &Rectangle, )
```

Processes a runtime [`Event`](../enum.Event.md). [Read more](../advanced/trait.Widget.md)

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, _tree: &Tree, layout: Layout, cursor: Cursor, _viewport: &Rectangle, _renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../mouse/enum.Interaction.md) of the [`Widget`](../advanced/trait.Widget.md). [Read more](../advanced/trait.Widget.md)

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, _style: &Style, layout: Layout, _cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`](../advanced/trait.Widget.md) using the associated `Renderer`.

#### `operate`

```rust
fn operate( &mut self, _tree: &mut Tree, layout: Layout, _viewport: &Rectangle, _renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`](../advanced/widget/trait.Operation.md) to the [`Widget`](../advanced/trait.Widget.md).

#### `overlay`

```rust
fn overlay<'b>( &'b mut self, tree: &'b mut Tree, layout: Layout, renderer: &Renderer, _viewport: &Rectangle, translation: Vector, window: Size, ) -> Vec<Element<'b, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`](../advanced/trait.Widget.md).

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`](../advanced/trait.Widget.md) with the provided [`Tree`](../advanced/widget/struct.Tree.md).

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

- `impl<'a, T, L, V, Message, Theme> Freeze for PickList<'a, T, L, V, Message, Theme> where L: Freeze, <Theme as Catalog>::Class<'a>: Freeze, <Theme as Catalog>::Class<'a>: Freeze, Message: Freeze, V: Freeze,`
- `impl<'a, T, L, V, Message, Theme = Theme> !RefUnwindSafe for PickList<'a, T, L, V, Message, Theme>`
- `impl<'a, T, L, V, Message, Theme = Theme> !Send for PickList<'a, T, L, V, Message, Theme>`
- `impl<'a, T, L, V, Message, Theme = Theme> !Sync for PickList<'a, T, L, V, Message, Theme>`
- `impl<'a, T, L, V, Message, Theme> Unpin for PickList<'a, T, L, V, Message, Theme> where L: Unpin, <Theme as Catalog>::Class<'a>: Unpin, <Theme as Catalog>::Class<'a>: Unpin, Message: Unpin, V: Unpin,`
- `impl<'a, T, L, V, Message, Theme> UnsafeUnpin for PickList<'a, T, L, V, Message, Theme> where L: UnsafeUnpin, <Theme as Catalog>::Class<'a>: UnsafeUnpin, <Theme as Catalog>::Class<'a>: UnsafeUnpin, Message: UnsafeUnpin, V: UnsafeUnpin,`
- `impl<'a, T, L, V, Message, Theme = Theme> !UnwindSafe for PickList<'a, T, L, V, Message, Theme>`
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

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
