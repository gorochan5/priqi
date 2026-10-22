---
title: "Map"
kind: struct
path: "iced_core::widget::Map"
crate: "iced_core"
module: "iced_core::widget"
file: "iced_core/widget/struct.Map.md"
docs_url: "https://docs.iced.rs/iced_core/widget/struct.Map.html"
source_file: "src/iced_core/widget.rs"
source_lines: "298-302"
---

# `iced_core::widget::Map`

```rust
pub struct Map<W, F, A> { /* private fields */ }
```

A [`Widget`](trait.Widget.md) that transforms the produced message of another widget.

This widget is returned by [`Widget::map`](trait.Widget.md).

**Source:** [widget.rs](../../src/iced_core/widget.rs.md) (lines 298-302)

## Trait Implementations

### `impl<W, F, A> Meta for Map<W, F, A> where W: Meta,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`](trait.Widget.md) is [`Void`](struct.Void.md).

### `impl<W, A, B, F, Theme, Renderer> Widget<B, Theme, Renderer> for Map<W, F, A> where W: Widget<A, Theme, Renderer>, B: 'static, F: Fn(A) -> B, Theme: 'static, Renderer: Renderer + 'static,`

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

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, viewport: &Rectangle, renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`](operation/trait.Operation.md) to the [`Widget`](trait.Widget.md).

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, renderer: &Renderer, shell: &mut Shell<'_, B>, viewport: &Rectangle, )
```

Processes a runtime [`Event`](../event/enum.Event.md). [Read more](trait.Widget.md)

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout, cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`](trait.Widget.md) using the associated `Renderer`.

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, tree: &Tree, layout: Layout, cursor: Cursor, viewport: &Rectangle, renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../mouse/enum.Interaction.md) of the [`Widget`](trait.Widget.md). [Read more](trait.Widget.md)

#### `overlay`

```rust
fn overlay<'b>( &'b mut self, tree: &'b mut Tree, layout: Layout, renderer: &Renderer, viewport: &Rectangle, translation: Vector, window: Size, ) -> Vec<Element<'b, B, Theme, Renderer>>
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

- `impl<W, F, A> Freeze for Map<W, F, A> where W: Freeze, F: Freeze,`
- `impl<W, F, A> RefUnwindSafe for Map<W, F, A> where W: RefUnwindSafe, F: RefUnwindSafe, A: RefUnwindSafe,`
- `impl<W, F, A> Send for Map<W, F, A> where W: Send, F: Send, A: Send,`
- `impl<W, F, A> Sync for Map<W, F, A> where W: Sync, F: Sync, A: Sync,`
- `impl<W, F, A> Unpin for Map<W, F, A> where W: Unpin, F: Unpin, A: Unpin,`
- `impl<W, F, A> UnsafeUnpin for Map<W, F, A> where W: UnsafeUnpin, F: UnsafeUnpin,`
- `impl<W, F, A> UnwindSafe for Map<W, F, A> where W: UnwindSafe, F: UnwindSafe, A: UnwindSafe,`
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
