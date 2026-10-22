---
title: "PlainText"
kind: struct
path: "iced_core::text::parser::PlainText"
crate: "iced_core"
module: "iced_core::text::parser"
file: "iced_core/text/parser/struct.PlainText.md"
docs_url: "https://docs.iced.rs/iced_core/text/parser/struct.PlainText.html"
source_file: "src/iced_core/text/parser.rs"
source_lines: "45"
---

# `iced_core::text::parser::PlainText`

```rust
pub struct PlainText;
```

A parser that produces no output.

**Source:** [parser.rs](../../../src/iced_core/text/parser.rs.md) (lines 45)

## Trait Implementations

### `impl Clone for PlainText`

#### `clone`

```rust
fn clone(&self) -> PlainText
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for PlainText`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Parser for PlainText`

#### `Settings`

```rust
type Settings = ()
```

The settings to configure the [`Parser`](trait.Parser.md).

#### `Output`

```rust
type Output = ()
```

The output of this [`Parser`](trait.Parser.md).

#### `Iterator`

```rust
type Iterator<'a> = Empty<(Range<usize>, ())>
```

The parse iterator type.

#### `new`

```rust
fn new(_settings: &Self::Settings) -> Self
```

Creates a new [`Parser`](trait.Parser.md) from its [`Self::Settings`](trait.Parser.md).

#### `update`

```rust
fn update(&mut self, _new_settings: &Self::Settings)
```

Updates the [`Parser`](trait.Parser.md) with some new [`Self::Settings`](trait.Parser.md).

#### `change_line`

```rust
fn change_line(&mut self, _line: usize)
```

Notifies the [`Parser`](trait.Parser.md) that the line at the given index has changed.

#### `parse_line`

```rust
fn parse_line(&mut self, _line: &str) -> Self::Iterator<'_>
```

Parses the given line. [Read more](trait.Parser.md)

#### `current_line`

```rust
fn current_line(&self) -> usize
```

Returns the current line of the [`Parser`](trait.Parser.md). [Read more](trait.Parser.md)

### `impl Copy for PlainText`

## Auto Trait Implementations

- `impl Freeze for PlainText`
- `impl RefUnwindSafe for PlainText`
- `impl Send for PlainText`
- `impl Sync for PlainText`
- `impl Unpin for PlainText`
- `impl UnsafeUnpin for PlainText`
- `impl UnwindSafe for PlainText`
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

### `impl<T> CloneToUninit for T where T: Clone,`

#### `clone_to_uninit`

```rust
unsafe fn clone_to_uninit(&self, dest: *mut u8)
```

*🔬This is a nightly-only experimental API. (clone_to_uninit)*

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

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

### `impl<T> ToOwned for T where T: Clone,`

#### `Owned`

```rust
type Owned = T
```

The resulting type after obtaining ownership.

#### `to_owned`

```rust
fn to_owned(&self) -> T
```

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

#### `clone_into`

```rust
fn clone_into(&self, target: &mut T)
```

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

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
