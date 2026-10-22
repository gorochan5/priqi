---
title: "Layout"
kind: struct
path: "iced_core::text::input::Layout"
crate: "iced_core"
module: "iced_core::text::input"
file: "iced_core/text/input/struct.Layout.md"
docs_url: "https://docs.iced.rs/iced_core/text/input/struct.Layout.html"
source_file: "src/iced_core/text/input.rs"
source_lines: "29-40"
---

# `iced_core::text::input::Layout`

```rust
pub struct Layout<'a> {
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub placeholder: &'a str,
    pub font: Option<Font>,
    pub size: Option<Pixels>,
    pub line_height: Option<LineHeight>,
    pub alignment: Alignment,
    pub multiline: Option<Wrapping>,
    pub is_secure: bool,
}
```

**Source:** [input.rs](../../../src/iced_core/text/input.rs.md) (lines 29-40)

## Fields

### `width: Length`

### `height: Length`

### `padding: Padding`

### `placeholder: &'a str`

### `font: Option<Font>`

### `size: Option<Pixels>`

### `line_height: Option<LineHeight>`

### `alignment: Alignment`

### `multiline: Option<Wrapping>`

### `is_secure: bool`

## Auto Trait Implementations

- `impl<'a> Freeze for Layout<'a>`
- `impl<'a> RefUnwindSafe for Layout<'a>`
- `impl<'a> Send for Layout<'a>`
- `impl<'a> Sync for Layout<'a>`
- `impl<'a> Unpin for Layout<'a>`
- `impl<'a> UnsafeUnpin for Layout<'a>`
- `impl<'a> UnwindSafe for Layout<'a>`
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
