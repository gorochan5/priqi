---
title: "Input"
kind: struct
path: "iced::advanced::text::Input"
crate: "iced"
module: "iced::advanced::text"
file: "iced/advanced/text/struct.Input.md"
docs_url: "https://docs.iced.rs/iced/advanced/text/struct.Input.html"
---

# `iced::advanced::text::Input`

```rust
pub struct Input<R> where
    R: Renderer, { /* private fields */ }
```

## Methods

### `impl<R> Input<R> where R: Renderer,`

#### `new`

```rust
pub fn new() -> Input<R>
```

#### `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

#### `value`

```rust
pub fn value(&self) -> String
```

#### `placeholder`

```rust
pub fn placeholder(&self) -> &str
```

#### `overwrite`

```rust
pub fn overwrite(&mut self, value: &str)
```

#### `layout`

```rust
pub fn layout( &mut self, renderer: &R, limits: &Limits, layout: Layout<'_>, ) -> Size
```

#### `update`

```rust
pub fn update<Message>( &mut self, event: &Event, bounds: Rectangle, cursor: Cursor, shell: &mut Shell<'_, Message>, key_binding: impl Fn(KeyPress) -> Option<Binding<Message>>, ) -> Option<Edit>
```

#### `draw`

```rust
pub fn draw( &self, renderer: &mut R, bounds: Rectangle, viewport: Rectangle, style: Style, )
```

#### `input_method`

```rust
pub fn input_method(&self, position: Point) -> InputMethod<&str>
```

## Trait Implementations

### `impl<R> Default for Input<R> where R: Renderer,`

#### `default`

```rust
fn default() -> Input<R>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<R> Focusable for Input<R> where R: Renderer,`

#### `is_focused`

```rust
fn is_focused(&self) -> bool
```

Returns whether the widget is focused or not.

#### `focus`

```rust
fn focus(&mut self)
```

Focuses the widget.

#### `unfocus`

```rust
fn unfocus(&mut self)
```

Unfocuses the widget.

### `impl<R> TextInput for Input<R> where R: Renderer,`

#### `text`

```rust
fn text(&self) -> Cow<'_, str>
```

Returns the current *visible* text of the text input. [Read more](../widget/operation/trait.TextInput.md)

#### `move_cursor_to`

```rust
fn move_cursor_to(&mut self, position: Position)
```

Moves the cursor of the text input to an arbitrary location.

#### `move_cursor_to_front`

```rust
fn move_cursor_to_front(&mut self)
```

Moves the cursor of the text input to the front of the input text.

#### `move_cursor_to_end`

```rust
fn move_cursor_to_end(&mut self)
```

Moves the cursor of the text input to the end of the input text.

#### `select_all`

```rust
fn select_all(&mut self)
```

Selects all the content of the text input.

#### `select_range`

```rust
fn select_range(&mut self, start: Position, end: Position)
```

Selects the given content range of the text input.

## Auto Trait Implementations

- `impl<R> Freeze for Input<R> where <R as Renderer>::Editor: Freeze, <R as Renderer>::Paragraph: Freeze,`
- `impl<R> RefUnwindSafe for Input<R> where <R as Renderer>::Editor: RefUnwindSafe, <R as Renderer>::Paragraph: RefUnwindSafe,`
- `impl<R> Send for Input<R> where <R as Renderer>::Editor: Send, <R as Renderer>::Paragraph: Send,`
- `impl<R> Sync for Input<R> where <R as Renderer>::Editor: Sync, <R as Renderer>::Paragraph: Sync,`
- `impl<R> Unpin for Input<R> where <R as Renderer>::Editor: Unpin, <R as Renderer>::Paragraph: Unpin,`
- `impl<R> UnsafeUnpin for Input<R> where <R as Renderer>::Editor: UnsafeUnpin, <R as Renderer>::Paragraph: UnsafeUnpin,`
- `impl<R> UnwindSafe for Input<R> where <R as Renderer>::Editor: UnwindSafe, <R as Renderer>::Paragraph: UnwindSafe,`
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
