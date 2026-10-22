---
title: "Parser"
kind: struct
path: "iced_highlighter::Parser"
crate: "iced_highlighter"
module: "iced_highlighter"
file: "iced_highlighter/struct.Parser.md"
docs_url: "https://docs.iced.rs/iced_highlighter/struct.Parser.html"
source_file: "src/iced_highlighter/lib.rs"
source_lines: "19-23"
---

# `iced_highlighter::Parser`

```rust
pub struct Parser { /* private fields */ }
```

A syntax parser.

**Source:** [lib.rs](../src/iced_highlighter/lib.rs.md) (lines 19-23)

## Methods

### `impl Parser`

#### `new`

```rust
pub fn new(settings: &Settings) -> Self
```

Creates a new [`Parser`](struct.Parser.md) with the given [`Settings`](struct.Settings.md).

#### `update`

```rust
pub fn update(&mut self, new_settings: &Settings)
```

Updates the parser with the given [`Settings`](struct.Settings.md), restarting it from the first line.

#### `change_line`

```rust
pub fn change_line(&mut self, line: usize)
```

Changes the line the parser is currently on.

#### `parse_line`

```rust
pub fn parse_line(&mut self, line: &str) -> CodeIterator<'_>
```

Highlights the given line, returning a [`CodeIterator`](type.CodeIterator.md).

#### `current_line`

```rust
pub fn current_line(&self) -> usize
```

Returns the line the parser is currently on.

## Trait Implementations

### `impl Debug for Parser`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Parser for Parser`

#### `Settings`

```rust
type Settings = Settings
```

The settings to configure the [`Parser`].

#### `Output`

```rust
type Output = Code
```

The output of this [`Parser`].

#### `Iterator`

```rust
type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Code)> + 'a>
```

The parse iterator type.

#### `new`

```rust
fn new(settings: &Self::Settings) -> Self
```

Creates a new [`Parser`] from its [`Self::Settings`].

#### `update`

```rust
fn update(&mut self, new_settings: &Self::Settings)
```

Updates the [`Parser`] with some new [`Self::Settings`].

#### `change_line`

```rust
fn change_line(&mut self, line: usize)
```

Notifies the [`Parser`] that the line at the given index has changed.

#### `parse_line`

```rust
fn parse_line(&mut self, line: &str) -> Self::Iterator<'_>
```

Parses the given line. Read more

#### `current_line`

```rust
fn current_line(&self) -> usize
```

Returns the current line of the [`Parser`]. Read more

## Auto Trait Implementations

- `impl Freeze for Parser`
- `impl RefUnwindSafe for Parser`
- `impl Send for Parser`
- `impl Sync for Parser`
- `impl Unpin for Parser`
- `impl UnsafeUnpin for Parser`
- `impl UnwindSafe for Parser`
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
