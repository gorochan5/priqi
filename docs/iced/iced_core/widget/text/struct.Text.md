---
title: "Text"
kind: struct
path: "iced_core::widget::text::Text"
crate: "iced_core"
module: "iced_core::widget::text"
file: "iced_core/widget/text/struct.Text.md"
docs_url: "https://docs.iced.rs/iced_core/widget/text/struct.Text.html"
source_file: "src/iced_core/widget/text.rs"
source_lines: "60-68"
---

# `iced_core::widget::text::Text`

```rust
pub struct Text<'a, Theme> where
    Theme: Catalog, { /* private fields */ }
```

A bunch of text.

## Example

```rust
use iced::widget::text;
use iced::color;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    text("Hello, this is iced!")
        .size(20)
        .color(color!(0x0000ff))
}
```

**Source:** [text.rs](../../../src/iced_core/widget/text.rs.md) (lines 60-68)

## Methods

### `impl<'a, Theme> Text<'a, Theme> where Theme: Catalog,`

#### `new`

```rust
pub fn new(fragment: impl IntoFragment<'a>) -> Self
```

Create a new fragment of [`Text`](struct.Text.md) with the given contents.

#### `size`

```rust
pub fn size(self, size: impl Into<Pixels>) -> Self
```

Sets the size of the [`Text`](struct.Text.md).

#### `line_height`

```rust
pub fn line_height(self, line_height: impl Into<LineHeight>) -> Self
```

Sets the [`LineHeight`](../../text/enum.LineHeight.md) of the [`Text`](struct.Text.md).

#### `font`

```rust
pub fn font(self, font: impl Into<Font>) -> Self
```

Sets the [`Font`](../../font/struct.Font.md) of the [`Text`](struct.Text.md).

#### `font_maybe`

```rust
pub fn font_maybe(self, font: Option<impl Into<Font>>) -> Self
```

Sets the [`Font`](../../font/struct.Font.md) of the [`Text`](struct.Text.md), if `Some`.

#### `width`

```rust
pub fn width(self, width: impl Into<Length>) -> Self
```

Sets the width of the [`Text`](struct.Text.md) boundaries.

#### `height`

```rust
pub fn height(self, height: impl Into<Length>) -> Self
```

Sets the height of the [`Text`](struct.Text.md) boundaries.

#### `center`

```rust
pub fn center(self) -> Self
```

Centers the [`Text`](struct.Text.md), both horizontally and vertically.

#### `align_x`

```rust
pub fn align_x(self, alignment: impl Into<Alignment>) -> Self
```

Sets the [`alignment::Horizontal`](../../alignment/enum.Horizontal.md) of the [`Text`](struct.Text.md).

#### `align_y`

```rust
pub fn align_y(self, alignment: impl Into<Vertical>) -> Self
```

Sets the [`alignment::Vertical`](../../alignment/enum.Vertical.md) of the [`Text`](struct.Text.md).

#### `shaping`

```rust
pub fn shaping(self, shaping: Shaping) -> Self
```

Sets the [`Shaping`](../../text/enum.Shaping.md) strategy of the [`Text`](struct.Text.md).

#### `wrapping`

```rust
pub fn wrapping(self, wrapping: Wrapping) -> Self
```

Sets the [`Wrapping`](../../text/enum.Wrapping.md) strategy of the [`Text`](struct.Text.md).

#### `ellipsis`

```rust
pub fn ellipsis(self, ellipsis: Ellipsis) -> Self
```

Sets the [`Ellipsis`](../../text/enum.Ellipsis.md) strategy of the [`Text`](struct.Text.md).

#### `selectable`

```rust
pub fn selectable(self, selectable: bool) -> Self
```

Sets whether the [`Text`](struct.Text.md) can be selected.

By default, it is `false`.

#### `style`

```rust
pub fn style(self, style: impl Fn(&Theme) -> Style + 'a) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the style of the [`Text`](struct.Text.md).

#### `color`

```rust
pub fn color(self, color: impl Into<Color>) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the [`Color`](../../color/struct.Color.md) of the [`Text`](struct.Text.md).

#### `color_maybe`

```rust
pub fn color_maybe(self, color: Option<impl Into<Color>>) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the [`Color`](../../color/struct.Color.md) of the [`Text`](struct.Text.md), if `Some`.

#### `class`

```rust
pub fn class(self, class: impl Into<Theme::Class<'a>>) -> Self
```

Sets the style class of the [`Text`](struct.Text.md).

## Trait Implementations

### `impl<'a, Theme> From<&'a str> for Text<'a, Theme> where Theme: Catalog + 'a,`

#### `from`

```rust
fn from(content: &'a str) -> Self
```

Converts to this type from the input type.

### `impl<Theme> Meta for Text<'_, Theme> where Theme: Catalog,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`](../trait.Widget.md) is [`Void`](../struct.Void.md).

### `impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Text<'_, Theme> where Theme: Catalog, Renderer: Renderer,`

#### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`](../../struct.Size.md) of the [`Widget`](../trait.Widget.md) in lengths.

#### `tag`

```rust
fn tag(&self) -> Tag
```

Returns the [`Tag`](../tree/struct.Tag.md) of the [`Widget`](../trait.Widget.md).

#### `state`

```rust
fn state(&self) -> State
```

Returns the [`State`](../tree/enum.State.md) of the [`Widget`](../trait.Widget.md).

#### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`](../trait.Widget.md). [Read more](../trait.Widget.md)

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, _renderer: &Renderer, _shell: &mut Shell<'_, Message>, _viewport: &Rectangle, )
```

Processes a runtime [`Event`](../../event/enum.Event.md). [Read more](../trait.Widget.md)

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, tree: &Tree, _layout: Layout, _cursor: Cursor, _viewport: &Rectangle, _renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../../mouse/enum.Interaction.md) of the [`Widget`](../trait.Widget.md). [Read more](../trait.Widget.md)

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, defaults: &Style, layout: Layout, _cursor_position: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`](../trait.Widget.md) using the associated `Renderer`.

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, _viewport: &Rectangle, _renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`](../operation/trait.Operation.md) to the [`Widget`](../trait.Widget.md).

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`](../trait.Widget.md) with the provided [`Tree`](../tree/struct.Tree.md).

#### `overlay`

```rust
fn overlay<'a>( &'a mut self, _tree: &'a mut Tree, _layout: Layout, _renderer: &Renderer, _viewport: &Rectangle, _translation: Vector, _window: Size, ) -> Vec<Element<'a, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`](../trait.Widget.md).

#### `_map`

```rust
fn _map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](../trait.Widget.md), returning a [`Map`](../struct.Map.md). [Read more](../trait.Widget.md)

#### `_boxed`

```rust
fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](../trait.Widget.md), turning it into a generic [`Element`](../struct.Element.md).

## Auto Trait Implementations

- `impl<'a, Theme> Freeze for Text<'a, Theme> where <Theme as Catalog>::Class<'a>: Freeze,`
- `impl<'a, Theme> RefUnwindSafe for Text<'a, Theme> where <Theme as Catalog>::Class<'a>: RefUnwindSafe,`
- `impl<'a, Theme> Send for Text<'a, Theme> where <Theme as Catalog>::Class<'a>: Send,`
- `impl<'a, Theme> Sync for Text<'a, Theme> where <Theme as Catalog>::Class<'a>: Sync,`
- `impl<'a, Theme> Unpin for Text<'a, Theme> where <Theme as Catalog>::Class<'a>: Unpin,`
- `impl<'a, Theme> UnsafeUnpin for Text<'a, Theme> where <Theme as Catalog>::Class<'a>: UnsafeUnpin,`
- `impl<'a, Theme> UnwindSafe for Text<'a, Theme> where <Theme as Catalog>::Class<'a>: UnwindSafe,`
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
