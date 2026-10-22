---
title: "Stream"
kind: struct
path: "iced_highlighter::Stream"
crate: "iced_highlighter"
module: "iced_highlighter"
file: "iced_highlighter/struct.Stream.md"
docs_url: "https://docs.iced.rs/iced_highlighter/struct.Stream.html"
source_file: "src/iced_highlighter/lib.rs"
source_lines: "155-160"
---

# `iced_highlighter::Stream`

```rust
pub struct Stream { /* private fields */ }
```

A streaming syntax parser.

It can efficiently highlight an immutable stream of tokens.

**Source:** [lib.rs](../src/iced_highlighter/lib.rs.md) (lines 155-160)

## Methods

### `impl Stream`

#### `new`

```rust
pub fn new(settings: &Settings) -> Self
```

Creates a new [`Stream`](struct.Stream.md) parser.

#### `parse_line`

```rust
pub fn parse_line( &mut self, line: &str, ) -> impl Iterator<Item = (Range<usize>, Code)> + '_
```

Highlights the given line from the last commit.

#### `commit`

```rust
pub fn commit(&mut self)
```

Commits the last highlighted line.

#### `reset`

```rust
pub fn reset(&mut self)
```

Resets the [`Stream`](struct.Stream.md) parser.

## Trait Implementations

### `impl Debug for Stream`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl Freeze for Stream`
- `impl RefUnwindSafe for Stream`
- `impl Send for Stream`
- `impl Sync for Stream`
- `impl Unpin for Stream`
- `impl UnsafeUnpin for Stream`
- `impl UnwindSafe for Stream`
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
