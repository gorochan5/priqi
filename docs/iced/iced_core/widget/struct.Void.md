---
title: "Void"
kind: struct
path: "iced_core::widget::Void"
crate: "iced_core"
module: "iced_core::widget"
file: "iced_core/widget/struct.Void.md"
docs_url: "https://docs.iced.rs/iced_core/widget/struct.Void.html"
source_file: "src/iced_core/widget.rs"
source_lines: "264"
---

# `iced_core::widget::Void`

```rust
pub struct Void;
```

A zero-sized [`Widget`](trait.Widget.md) that does nothing and will be filtered out by containers.

**Source:** [widget.rs](../../src/iced_core/widget.rs.md) (lines 264)

## Trait Implementations

### `impl Meta for Void`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`](trait.Widget.md) is [`Void`](struct.Void.md).

### `impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Void`

#### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`](../struct.Size.md) of the [`Widget`](trait.Widget.md) in lengths.

#### `layout`

```rust
fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, _limits: &Limits)
```

Lays out the [`Widget`](trait.Widget.md). [Read more](trait.Widget.md)

#### `draw`

```rust
fn draw( &self, _tree: &Tree, _renderer: &mut Renderer, _theme: &Theme, _style: &Style, _layout: Layout, _cursor: Cursor, _viewport: &Rectangle, )
```

Draws the [`Widget`](trait.Widget.md) using the associated `Renderer`.

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

#### `operate`

```rust
fn operate( &mut self, _tree: &mut Tree, _layout: Layout, _viewport: &Rectangle, _renderer: &Renderer, _operation: &mut dyn Operation, )
```

Applies an [`Operation`](operation/trait.Operation.md) to the [`Widget`](trait.Widget.md).

#### `update`

```rust
fn update( &mut self, _tree: &mut Tree, _event: &Event, _layout: Layout, _cursor: Cursor, _renderer: &Renderer, _shell: &mut Shell<'_, Message>, _viewport: &Rectangle, )
```

Processes a runtime [`Event`](../event/enum.Event.md). [Read more](trait.Widget.md)

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, _tree: &Tree, _layout: Layout, _cursor: Cursor, _viewport: &Rectangle, _renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../mouse/enum.Interaction.md) of the [`Widget`](trait.Widget.md). [Read more](trait.Widget.md)

#### `overlay`

```rust
fn overlay<'a>( &'a mut self, _tree: &'a mut Tree, _layout: Layout, _renderer: &Renderer, _viewport: &Rectangle, _translation: Vector, _window: Size, ) -> Vec<Element<'a, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`](trait.Widget.md).

#### `_map`

```rust
fn _map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](trait.Widget.md), returning a [`Map`](struct.Map.md). [Read more](trait.Widget.md)

#### `_boxed`

```rust
fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](trait.Widget.md), turning it into a generic [`Element`](struct.Element.md).

## Auto Trait Implementations

- `impl Freeze for Void`
- `impl RefUnwindSafe for Void`
- `impl Send for Void`
- `impl Sync for Void`
- `impl Unpin for Void`
- `impl UnsafeUnpin for Void`
- `impl UnwindSafe for Void`
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
