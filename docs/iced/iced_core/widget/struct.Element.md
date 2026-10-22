---
title: "Element"
kind: struct
path: "iced_core::widget::Element"
crate: "iced_core"
module: "iced_core::widget"
file: "iced_core/widget/struct.Element.md"
docs_url: "https://docs.iced.rs/iced_core/widget/struct.Element.html"
source_file: "src/iced_core/widget/element.rs"
source_lines: "16-18"
---

# `iced_core::widget::Element`

```rust
pub struct Element<'a, Message, Theme, Renderer> { /* private fields */ }
```

A generic [`Widget`](trait.Widget.md).

It is useful to build composable user interfaces that do not leak implementation details in their **view logic**.

If you have any [`Widget`](trait.Widget.md), you should be able to use [`Widget::boxed`](trait.Widget.md) to turn it into an [`Element`](struct.Element.md).

**Source:** [element.rs](../../src/iced_core/widget/element.rs.md) (lines 16-18)

## Trait Implementations

### `impl<'a, Message, Theme, Renderer> Meta for Element<'a, Message, Theme, Renderer>`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`](trait.Widget.md) is [`Void`](struct.Void.md).

### `impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Element<'a, Message, Theme, Renderer>`

#### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`](../struct.Size.md) of the [`Widget`](trait.Widget.md) in lengths.

#### `tag`

```rust
fn tag(&self) -> Tag
```

Returns the [`Tag`](tree/struct.Tag.md) of the [`Widget`](trait.Widget.md).

#### `state`

```rust
fn state(&self) -> State
```

Returns the [`State`](tree/enum.State.md) of the [`Widget`](trait.Widget.md).

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`](trait.Widget.md) with the provided [`Tree`](tree/struct.Tree.md).

#### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`](trait.Widget.md). [Read more](trait.Widget.md)

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout, cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`](trait.Widget.md) using the associated `Renderer`.

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, viewport: &Rectangle, renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`](operation/trait.Operation.md) to the [`Widget`](trait.Widget.md).

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, renderer: &Renderer, shell: &mut Shell<'_, Message>, viewport: &Rectangle, )
```

Processes a runtime [`Event`](../event/enum.Event.md). [Read more](trait.Widget.md)

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, tree: &Tree, layout: Layout, cursor: Cursor, viewport: &Rectangle, renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../mouse/enum.Interaction.md) of the [`Widget`](trait.Widget.md). [Read more](trait.Widget.md)

#### `overlay`

```rust
fn overlay<'b>( &'b mut self, tree: &'b mut Tree, layout: Layout, renderer: &Renderer, viewport: &Rectangle, translation: Vector, window: Size, ) -> Vec<Element<'b, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`](trait.Widget.md).

#### `_boxed`

```rust
fn _boxed<'b>(self) -> Element<'b, Message, Theme, Renderer> where Self: Sized + 'b,
```

Boxes this [`Widget`](trait.Widget.md), turning it into a generic [`Element`](struct.Element.md).

#### `_map`

```rust
fn _map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](trait.Widget.md), returning a [`Map`](struct.Map.md). [Read more](trait.Widget.md)

## Auto Trait Implementations

- `impl<'a, Message, Theme, Renderer> Freeze for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !RefUnwindSafe for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !Send for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !Sync for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> Unpin for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> UnsafeUnpin for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !UnwindSafe for Element<'a, Message, Theme, Renderer>`
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
