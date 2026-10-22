---
title: "Menu"
kind: struct
path: "iced_widget::overlay::menu::Menu"
crate: "iced_widget"
module: "iced_widget::overlay::menu"
file: "iced_widget/overlay/menu/struct.Menu.md"
docs_url: "https://docs.iced.rs/iced_widget/overlay/menu/struct.Menu.html"
source_file: "src/iced_widget/overlay/menu.rs"
source_lines: "21-41"
---

# `iced_widget::overlay::menu::Menu`

```rust
pub struct Menu<'a, 'b, T, Message, Theme = Theme> where
    Theme: Catalog,
    'b: 'a, { /* private fields */ }
```

A list of selectable options.

**Source:** [menu.rs](../../../src/iced_widget/overlay/menu.rs.md) (lines 21-41)

## Methods

### `impl<'a, 'b, T, Message, Theme> Menu<'a, 'b, T, Message, Theme> where T: Clone, Message: 'a, Theme: Catalog + 'a, 'b: 'a,`

#### `new`

```rust
pub fn new( state: &'a mut State, options: &'a [T], hovered_option: &'a mut Option<usize>, to_string: &'a dyn Fn(&T) -> String, on_selected: impl FnMut(T) -> Message + 'a, on_option_hovered: Option<&'a dyn Fn(T) -> Message>, class: &'a <Theme as Catalog>::Class<'b>, ) -> Self
```

Creates a new [`Menu`](struct.Menu.md) with the given [`State`](struct.State.md), a list of options, the message to produced when an option is selected, and its [`Style`](struct.Style.md).

#### `width`

```rust
pub fn width(self, width: f32) -> Self
```

Sets the width of the [`Menu`](struct.Menu.md).

#### `height`

```rust
pub fn height(self, height: impl Into<Length>) -> Self
```

Sets the height of the [`Menu`](struct.Menu.md).

#### `padding`

```rust
pub fn padding<P: Into<Padding>>(self, padding: P) -> Self
```

Sets the [`Padding`] of the [`Menu`](struct.Menu.md).

#### `text_size`

```rust
pub fn text_size(self, text_size: impl Into<Pixels>) -> Self
```

Sets the text size of the [`Menu`](struct.Menu.md).

#### `line_height`

```rust
pub fn line_height(self, line_height: impl Into<LineHeight>) -> Self
```

Sets the text [`text::LineHeight`](../../text/enum.LineHeight.md) of the [`Menu`](struct.Menu.md).

#### `shaping`

```rust
pub fn shaping(self, shaping: Shaping) -> Self
```

Sets the [`text::Shaping`](../../text/enum.Shaping.md) strategy of the [`Menu`](struct.Menu.md).

#### `ellipsis`

```rust
pub fn ellipsis(self, ellipsis: Ellipsis) -> Self
```

Sets the [`text::Ellipsis`](../../text/enum.Ellipsis.md) strategy of the [`Menu`](struct.Menu.md).

#### `font`

```rust
pub fn font(self, font: impl Into<Font>) -> Self
```

Sets the font of the [`Menu`](struct.Menu.md).

#### `overlay`

```rust
pub fn overlay<Renderer>( self, renderer: &Renderer, position: Point, window: Size, target_height: f32, ) -> Element<'a, Message, Theme, Renderer> where Renderer: Renderer + 'a,
```

Turns the [`Menu`](struct.Menu.md) into an [`overlay::Element`] at the given target position.

`position` is the target’s position in screen coordinates, and `window` is the size of the window.

The `target_height` will be used to display the menu either on top of the target or under it, depending on the screen position and the dimensions of the [`Menu`](struct.Menu.md).

## Auto Trait Implementations

- `impl<'a, 'b, T, Message, Theme> Freeze for Menu<'a, 'b, T, Message, Theme>`
- `impl<'a, 'b, T, Message, Theme = Theme> !RefUnwindSafe for Menu<'a, 'b, T, Message, Theme>`
- `impl<'a, 'b, T, Message, Theme = Theme> !Send for Menu<'a, 'b, T, Message, Theme>`
- `impl<'a, 'b, T, Message, Theme = Theme> !Sync for Menu<'a, 'b, T, Message, Theme>`
- `impl<'a, 'b, T, Message, Theme> Unpin for Menu<'a, 'b, T, Message, Theme>`
- `impl<'a, 'b, T, Message, Theme> UnsafeUnpin for Menu<'a, 'b, T, Message, Theme>`
- `impl<'a, 'b, T, Message, Theme = Theme> !UnwindSafe for Menu<'a, 'b, T, Message, Theme>`
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
