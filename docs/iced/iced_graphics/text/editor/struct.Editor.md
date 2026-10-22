---
title: "Editor"
kind: struct
path: "iced_graphics::text::editor::Editor"
crate: "iced_graphics"
module: "iced_graphics::text::editor"
file: "iced_graphics/text/editor/struct.Editor.md"
docs_url: "https://docs.iced.rs/iced_graphics/text/editor/struct.Editor.html"
source_file: "src/iced_graphics/text/editor.rs"
source_lines: "16"
---

# `iced_graphics::text::editor::Editor`

```rust
pub struct Editor(/* private fields */);
```

A multi-line text editor.

**Source:** [editor.rs](../../../src/iced_graphics/text/editor.rs.md) (lines 16)

## Methods

### `impl Editor`

#### `new`

```rust
pub fn new() -> Self
```

Creates a new empty [`Editor`](struct.Editor.md).

#### `buffer`

```rust
pub fn buffer(&self) -> &Buffer
```

Returns the buffer of the [`Editor`](struct.Editor.md).

#### `downgrade`

```rust
pub fn downgrade(&self) -> Weak
```

Creates a [`Weak`](struct.Weak.md) reference to the [`Editor`](struct.Editor.md).

This is useful to avoid cloning the [`Editor`](struct.Editor.md) when referential guarantees are unnecessary. For instance, when creating a rendering tree.

## Trait Implementations

### `impl Debug for Editor`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Editor`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Editor for Editor`

#### `with_text`

```rust
fn with_text(text: &str) -> Self
```

Creates a new [`Editor`] laid out with the given text.

#### `is_empty`

```rust
fn is_empty(&self) -> bool
```

Returns true if the [`Editor`] has no contents.

#### `line`

```rust
fn line(&self, index: usize) -> Option<Line<'_>>
```

Returns the text of the given line in the [`Editor`], if it exists.

#### `line_count`

```rust
fn line_count(&self) -> usize
```

Returns the amount of lines in the [`Editor`].

#### `copy`

```rust
fn copy(&self) -> Option<String>
```

Returns the current selected text of the [`Editor`].

#### `selection`

```rust
fn selection(&self) -> Option<Selection>
```

Returns the current [`Selection`] of the [`Editor`], if visible.

#### `cursor`

```rust
fn cursor(&self) -> Cursor
```

Returns the current [`Cursor`] of the [`Editor`].

#### `perform`

```rust
fn perform(&mut self, action: Action)
```

Performs an [`Action`] on the [`Editor`].

#### `move_to`

```rust
fn move_to(&mut self, cursor: Cursor)
```

Moves the cursor to the given position.

#### `bounds`

```rust
fn bounds(&self) -> Size
```

Returns the current boundaries of the [`Editor`].

#### `min_bounds`

```rust
fn min_bounds(&self) -> Size
```

Returns the minimum boundaries to fit the current contents of the [`Editor`].

#### `hint_factor`

```rust
fn hint_factor(&self) -> Option<f32>
```

Returns the hint factor of the [`Editor`].

#### `update`

```rust
fn update( &mut self, new_bounds: Size, new_font: Font, new_size: Pixels, new_line_height: LineHeight, new_wrapping: Wrapping, new_alignment: Alignment, new_hint_factor: Option<f32>, new_parser: &mut impl Parser, )
```

Updates the [`Editor`] with some new attributes.

#### `overwrite`

```rust
fn overwrite(&mut self, new_text: &str)
```

Overwrites the current contents of the [`Editor`].

#### `highlight`

```rust
fn highlight<P: Parser>( &mut self, font: Font, parser: &mut P, highlight: impl Fn(P::Output) -> Style, )
```

Runs a [`text::Highlighter`] in the [`Editor`].

#### `text_size`

```rust
fn text_size(&self) -> Pixels
```

Returns the current text size of the [`Editor`].

#### `line_height`

```rust
fn line_height(&self) -> LineHeight
```

Returns the current [`LineHeight`] of the [`Editor`].

#### `font`

```rust
fn font(&self) -> Font
```

Returns the current [`Font`] of the [`Editor`].

#### `lines`

```rust
fn lines(&self) -> impl Iterator<Item = Line<'_>>
```

Returns an iterator of the text of the lines in the [`Editor`].

#### `text`

```rust
fn text(&self) -> String
```

Returns the text of the [`Editor`].

### `impl PartialEq for Editor`

#### `eq`

```rust
fn eq(&self, other: &Editor) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl StructuralPartialEq for Editor`

## Auto Trait Implementations

- `impl Freeze for Editor`
- `impl RefUnwindSafe for Editor`
- `impl Send for Editor`
- `impl Sync for Editor`
- `impl Unpin for Editor`
- `impl UnsafeUnpin for Editor`
- `impl UnwindSafe for Editor`
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

### `impl<T> TextInput for T where T: Editor,`

#### `text`

```rust
fn text(&self) -> Cow<'_, str>
```

Returns the current *visible* text of the text input. Read more

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

#### `move_cursor_to`

```rust
fn move_cursor_to(&mut self, position: Position)
```

Moves the cursor of the text input to an arbitrary location.

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

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
