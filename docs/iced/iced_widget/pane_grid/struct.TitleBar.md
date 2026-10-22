---
title: "TitleBar"
kind: struct
path: "iced_widget::pane_grid::TitleBar"
crate: "iced_widget"
module: "iced_widget::pane_grid"
file: "iced_widget/pane_grid/struct.TitleBar.md"
docs_url: "https://docs.iced.rs/iced_widget/pane_grid/struct.TitleBar.html"
source_file: "src/iced_widget/pane_grid/title_bar.rs"
source_lines: "13-28"
---

# `iced_widget::pane_grid::TitleBar`

```rust
pub struct TitleBar<'a, Message, C = Element<'a, Message>, Theme = Theme, Renderer = Renderer> where
    Theme: Catalog,
    Renderer: Renderer, { /* private fields */ }
```

The title bar of a [`Pane`](struct.Pane.md).

**Source:** [title_bar.rs](../../src/iced_widget/pane_grid/title_bar.rs.md) (lines 13-28)

## Methods

### `impl<'a, Message, C, Theme, Renderer> TitleBar<'a, Message, C, Theme, Renderer> where Theme: Catalog, Renderer: Renderer,`

#### `new`

```rust
pub fn new(content: C) -> Self
```

Creates a new [`TitleBar`](struct.TitleBar.md) with the given content.

#### `controls`

```rust
pub fn controls( self, controls: impl Into<Controls<'a, Message, Theme, Renderer>>, ) -> Self
```

Sets the controls of the [`TitleBar`](struct.TitleBar.md).

#### `padding`

```rust
pub fn padding<P: Into<Padding>>(self, padding: P) -> Self
```

Sets the [`Padding`] of the [`TitleBar`](struct.TitleBar.md).

#### `always_show_controls`

```rust
pub fn always_show_controls(self) -> Self
```

Sets whether or not the [`controls`](struct.TitleBar.md) attached to this [`TitleBar`](struct.TitleBar.md) are always visible.

By default, the controls are only visible when the [`Pane`](struct.Pane.md) of this [`TitleBar`](struct.TitleBar.md) is hovered.

#### `style`

```rust
pub fn style(self, style: impl Fn(&Theme) -> Style + 'a) -> Self where Theme::Class<'a>: From<StyleFn<'a, Theme>>,
```

Sets the style of the [`TitleBar`](struct.TitleBar.md).

#### `class`

```rust
pub fn class(self, class: impl Into<Theme::Class<'a>>) -> Self
```

*Available on crate feature advanced only.*

Sets the style class of the [`TitleBar`](struct.TitleBar.md).

### `impl<Message, C, Theme, Renderer> TitleBar<'_, Message, C, Theme, Renderer> where C: Widget<Message, Theme, Renderer>, Theme: Catalog, Renderer: Renderer,`

#### `draw`

```rust
pub fn draw( &self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, inherited_style: &Style, layout: Layout, cursor: Cursor, viewport: &Rectangle, show_controls: bool, )
```

Draws the [`TitleBar`](struct.TitleBar.md) with the provided `Renderer` and [`Layout`].

#### `is_over_pick_area`

```rust
pub fn is_over_pick_area( &self, tree: &Tree, layout: Layout, cursor_position: Point, ) -> bool
```

Returns whether the mouse cursor is over the pick area of the [`TitleBar`](struct.TitleBar.md) or not.

The whole [`TitleBar`](struct.TitleBar.md) is a pick area, except its controls.

## Auto Trait Implementations

- `impl<'a, Message, C, Theme, Renderer> Freeze for TitleBar<'a, Message, C, Theme, Renderer> where C: Freeze, <Theme as Catalog>::Class<'a>: Freeze,`
- `impl<'a, Message, C = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !RefUnwindSafe for TitleBar<'a, Message, C, Theme, Renderer>`
- `impl<'a, Message, C = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Send for TitleBar<'a, Message, C, Theme, Renderer>`
- `impl<'a, Message, C = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !Sync for TitleBar<'a, Message, C, Theme, Renderer>`
- `impl<'a, Message, C, Theme, Renderer> Unpin for TitleBar<'a, Message, C, Theme, Renderer> where C: Unpin, <Theme as Catalog>::Class<'a>: Unpin,`
- `impl<'a, Message, C, Theme, Renderer> UnsafeUnpin for TitleBar<'a, Message, C, Theme, Renderer> where C: UnsafeUnpin, <Theme as Catalog>::Class<'a>: UnsafeUnpin,`
- `impl<'a, Message, C = Element<'a, Message, Theme, Renderer<Renderer, Renderer>>, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> !UnwindSafe for TitleBar<'a, Message, C, Theme, Renderer>`
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
