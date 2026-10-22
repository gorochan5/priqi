---
title: "Input"
kind: struct
path: "iced_core::text::input::Input"
crate: "iced_core"
module: "iced_core::text::input"
file: "iced_core/text/input/struct.Input.md"
docs_url: "https://docs.iced.rs/iced_core/text/input/struct.Input.html"
source_file: "src/iced_core/text/input.rs"
source_lines: "20-27"
---

# `iced_core::text::input::Input`

```rust
pub struct Input<R: Renderer> { /* private fields */ }
```

**Source:** [input.rs](../../../src/iced_core/text/input.rs.md) (lines 20-27)

## Methods

### `impl<R: Renderer> Input<R>`

#### `new`

```rust
pub fn new() -> Self
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

### `impl<R: Renderer> Default for Input<R>`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<R: Renderer> Focusable for Input<R>`

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

### `impl<R: Renderer> TextInput for Input<R>`

#### `text`

```rust
fn text(&self) -> Fragment<'_>
```

Returns the current *visible* text of the text input. [Read more](../../widget/operation/text_input/trait.TextInput.md)

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
