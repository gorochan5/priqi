---
title: "Column"
kind: struct
path: "iced::widget::keyed::Column"
crate: "iced"
module: "iced::widget::keyed"
file: "iced/widget/keyed/struct.Column.md"
docs_url: "https://docs.iced.rs/iced/widget/keyed/struct.Column.html"
source_file: "src/iced_widget/keyed/column.rs"
source_lines: "31"
---

# `iced::widget::keyed::Column`

```rust
pub struct Column<Key, W> where
    Key: Copy + PartialEq, { /* private fields */ }
```

A container that distributes its contents vertically while keeping continuity.

## Example

```rust
use iced::widget::{keyed_column, text};

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    keyed_column((0..=100).map(|i| {
        (i, text!("Item {i}"))
    }))
}
```

**Source:** [column.rs](../../../src/iced_widget/keyed/column.rs.md) (lines 31)

## Methods

### `impl<Key, W> Column<Key, W> where Key: Copy + PartialEq,`

#### `new`

```rust
pub fn new() -> Column<Key, W>
```

Creates an empty [`Column`](struct.Column.md).

#### `from_vecs`

```rust
pub fn from_vecs(keys: Vec<Key>, children: Vec<W>) -> Column<Key, W>
```

Creates a [`Column`](struct.Column.md) from already allocated [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html)s.

Keep in mind that the [`Column`](struct.Column.md) will not inspect the [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html)s, which means it won’t automatically adapt to the sizing strategy of its contents.

If any of the children have a [`Length::Fill`](../../enum.Length.md) strategy, you will need to call [`Column::width`](struct.Column.md) or [`Column::height`](struct.Column.md) accordingly.

#### `with_capacity`

```rust
pub fn with_capacity(capacity: usize) -> Column<Key, W>
```

Creates a [`Column`](struct.Column.md) with the given capacity.

#### `with_children`

```rust
pub fn with_children( children: impl IntoIterator<Item = (Key, W)>, ) -> Column<Key, W> where W: Meta,
```

Creates a [`Column`](struct.Column.md) with the given elements.

#### `spacing`

```rust
pub fn spacing(self, amount: impl Into<Pixels>) -> Column<Key, W>
```

Sets the vertical spacing *between* elements.

Custom margins per element do not exist in iced. You should use this method instead! While less flexible, it helps you keep spacing between elements consistent.

#### `padding`

```rust
pub fn padding<P>(self, padding: P) -> Column<Key, W> where P: Into<Padding>,
```

Sets the [`Padding`](../../struct.Padding.md) of the [`Column`](struct.Column.md).

#### `width`

```rust
pub fn width(self, width: impl Into<Length>) -> Column<Key, W>
```

Sets the width of the [`Column`](struct.Column.md).

#### `height`

```rust
pub fn height(self, height: impl Into<Length>) -> Column<Key, W>
```

Sets the height of the [`Column`](struct.Column.md).

#### `align_items`

```rust
pub fn align_items(self, align: Alignment) -> Column<Key, W>
```

Sets the horizontal alignment of the contents of the [`Column`](struct.Column.md) .

#### `push`

```rust
pub fn push(self, key: Key, child: W) -> Column<Key, W> where W: Meta,
```

Adds an element to the [`Column`](struct.Column.md).

#### `push_maybe`

```rust
pub fn push_maybe(self, key: Key, child: Option<W>) -> Column<Key, W> where W: Meta,
```

Adds an element to the [`Column`](struct.Column.md), if `Some`.

#### `extend`

```rust
pub fn extend( self, children: impl IntoIterator<Item = (Key, W)>, ) -> Column<Key, W> where W: Meta,
```

Extends the [`Column`](struct.Column.md) with the given children.

## Trait Implementations

### `impl<Key, W> Default for Column<Key, W> where Key: Copy + PartialEq,`

#### `default`

```rust
fn default() -> Column<Key, W>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<Key, W> Meta for Column<Key, W> where Key: Copy + PartialEq,`

#### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`](../../advanced/trait.Widget.md) is [`Void`](../struct.Void.md).

### `impl<Key, W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Column<Key, W> where Renderer: Renderer, Key: Copy + PartialEq + 'static, W: Widget<Message, Theme, Renderer>,`

#### `tag`

```rust
fn tag(&self) -> Tag
```

Returns the [`Tag`](../../advanced/widget/tree/struct.Tag.md) of the [`Widget`](../../advanced/trait.Widget.md).

#### `state`

```rust
fn state(&self) -> State
```

Returns the [`State`](../../advanced/widget/tree/enum.State.md) of the [`Widget`](../../advanced/trait.Widget.md).

#### `diff`

```rust
fn diff(&mut self, tree: &mut Tree)
```

Reconciles the [`Widget`](../../advanced/trait.Widget.md) with the provided [`Tree`](../../advanced/widget/struct.Tree.md).

#### `size`

```rust
fn size(&self) -> Size<Length>
```

Returns the [`Size`](../../struct.Size.md) of the [`Widget`](../../advanced/trait.Widget.md) in lengths.

#### `layout`

```rust
fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits)
```

Lays out the [`Widget`](../../advanced/trait.Widget.md). [Read more](../../advanced/trait.Widget.md)

#### `operate`

```rust
fn operate( &mut self, tree: &mut Tree, layout: Layout, viewport: &Rectangle, renderer: &Renderer, operation: &mut dyn Operation, )
```

Applies an [`Operation`](../../advanced/widget/trait.Operation.md) to the [`Widget`](../../advanced/trait.Widget.md).

#### `update`

```rust
fn update( &mut self, tree: &mut Tree, event: &Event, layout: Layout, cursor: Cursor, renderer: &Renderer, shell: &mut Shell<'_, Message>, viewport: &Rectangle, )
```

Processes a runtime [`Event`](../../enum.Event.md). [Read more](../../advanced/trait.Widget.md)

#### `mouse_interaction`

```rust
fn mouse_interaction( &self, tree: &Tree, layout: Layout, cursor: Cursor, viewport: &Rectangle, renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../../mouse/enum.Interaction.md) of the [`Widget`](../../advanced/trait.Widget.md). [Read more](../../advanced/trait.Widget.md)

#### `draw`

```rust
fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout, cursor: Cursor, viewport: &Rectangle, )
```

Draws the [`Widget`](../../advanced/trait.Widget.md) using the associated `Renderer`.

#### `overlay`

```rust
fn overlay<'b>( &'b mut self, tree: &'b mut Tree, layout: Layout, renderer: &Renderer, viewport: &Rectangle, translation: Vector, window: Size, ) -> Vec<Element<'b, Message, Theme, Renderer>>
```

Returns the overlays of the [`Widget`](../../advanced/trait.Widget.md).

#### `_map`

```rust
fn _map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](../../advanced/trait.Widget.md), returning a [`Map`](../../advanced/widget/struct.Map.md). [Read more](../../advanced/trait.Widget.md)

#### `_boxed`

```rust
fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](../../advanced/trait.Widget.md), turning it into a generic [`Element`](../../advanced/widget/struct.Element.md).

## Auto Trait Implementations

- `impl<Key, W> Freeze for Column<Key, W>`
- `impl<Key, W> RefUnwindSafe for Column<Key, W> where Key: RefUnwindSafe, W: RefUnwindSafe,`
- `impl<Key, W> Send for Column<Key, W> where Key: Send, W: Send,`
- `impl<Key, W> Sync for Column<Key, W> where Key: Sync, W: Sync,`
- `impl<Key, W> Unpin for Column<Key, W> where Key: Unpin, W: Unpin,`
- `impl<Key, W> UnsafeUnpin for Column<Key, W>`
- `impl<Key, W> UnwindSafe for Column<Key, W> where Key: UnwindSafe, W: UnwindSafe,`
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

Turns some type into the initial state of some [`Application`](../../application/struct.Application.md).

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

### `impl<T> NoneValue for T where T: Default,`

#### `NoneType`

```rust
type NoneType = T
```

#### `null_value`

```rust
fn null_value() -> T
```

The none-equivalent value.

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

Applies a transformation to the produced message of this [`Widget`](../../trait.Widget.md). [Read more](../../trait.Widget.md)

#### `boxed`

```rust
fn boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](../../trait.Widget.md), turning it into a generic [`Element`](../../type.Element.md).

#### `explain`

```rust
fn explain( self, color: impl Into<Color>, ) -> impl Widget<Message, Theme, Renderer> where Self: Sized, Renderer: Renderer,
```

Marks this [`Widget`](../../trait.Widget.md) as *to-be-explained*. [Read more](../../trait.Widget.md)

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
