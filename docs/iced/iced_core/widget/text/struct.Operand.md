---
title: "Operand"
kind: struct
path: "iced_core::widget::text::Operand"
crate: "iced_core"
module: "iced_core::widget::text"
file: "iced_core/widget/text/struct.Operand.md"
docs_url: "https://docs.iced.rs/iced_core/widget/text/struct.Operand.html"
source_file: "src/iced_core/widget/text.rs"
source_lines: "399-408"
---

# `iced_core::widget::text::Operand`

```rust
pub struct Operand<'a, P: Paragraph> {
    pub paragraph: &'a mut Plain<P>,
    pub layout: Layout,
    pub selectable: bool,
}
```

The state of a widget with text, operated on by [`super::Operation::text`](../operation/trait.Operation.md).

**Source:** [text.rs](../../../src/iced_core/widget/text.rs.md) (lines 399-408)

## Fields

### `paragraph: &'a mut Plain<P>`

The [`Paragraph`](../../text/paragraph/trait.Paragraph.md) of the widget.

### `layout: Layout`

The layout of the widget.

### `selectable: bool`

Whether the text of the widget can be selected.

## Trait Implementations

### `impl<P: Paragraph> Text for Operand<'_, P>`

#### `text`

```rust
fn text(&self) -> Fragment<'_>
```

Returns the text of the widget.

#### `select`

```rust
fn select(&mut self, start: Point, end: Point, target: Target)
```

Selects text between the given positions in layout coordinates. [Read more](../operation/text/trait.Text.md)

#### `select_all`

```rust
fn select_all(&mut self)
```

Selects all the text of the widget.

#### `deselect`

```rust
fn deselect(&mut self)
```

Deselects any selected text;

#### `copy`

```rust
fn copy(&mut self) -> Option<String>
```

Returns the selected text, if any.

## Auto Trait Implementations

- `impl<'a, P> Freeze for Operand<'a, P>`
- `impl<'a, P> RefUnwindSafe for Operand<'a, P> where P: RefUnwindSafe,`
- `impl<'a, P> Send for Operand<'a, P> where P: Send,`
- `impl<'a, P> Sync for Operand<'a, P> where P: Sync,`
- `impl<'a, P> Unpin for Operand<'a, P>`
- `impl<'a, P> UnsafeUnpin for Operand<'a, P>`
- `impl<'a, P> !UnwindSafe for Operand<'a, P>`
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
