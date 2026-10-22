---
title: "Style"
kind: struct
path: "iced_core::text::editor::Style"
crate: "iced_core"
module: "iced_core::text::editor"
file: "iced_core/text/editor/struct.Style.md"
docs_url: "https://docs.iced.rs/iced_core/text/editor/struct.Style.html"
source_file: "src/iced_core/text/editor.rs"
source_lines: "723-729"
---

# `iced_core::text::editor::Style`

```rust
pub struct Style {
    pub value: Color,
    pub selection: Color,
}
```

The visual style of an [`Editor`](trait.Editor.md).

**Source:** [editor.rs](../../../src/iced_core/text/editor.rs.md) (lines 723-729)

## Fields

### `value: Color`

The [`Color`](../../color/struct.Color.md) of the contents.

### `selection: Color`

The background [`Color`](../../color/struct.Color.md) of any selection.

## Auto Trait Implementations

- `impl Freeze for Style`
- `impl RefUnwindSafe for Style`
- `impl Send for Style`
- `impl Sync for Style`
- `impl Unpin for Style`
- `impl UnsafeUnpin for Style`
- `impl UnwindSafe for Style`
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
