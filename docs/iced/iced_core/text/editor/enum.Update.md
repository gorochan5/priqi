---
title: "Update"
kind: enum
path: "iced_core::text::editor::Update"
crate: "iced_core"
module: "iced_core::text::editor"
file: "iced_core/text/editor/enum.Update.md"
docs_url: "https://docs.iced.rs/iced_core/text/editor/enum.Update.html"
source_file: "src/iced_core/text/editor.rs"
source_lines: "945-966"
---

# `iced_core::text::editor::Update`

```rust
pub enum Update<Message> {
    Action(Action),
    Focus,
    Unfocus,
    InputMethod,
    Release,
    Copy(String),
    Paste,
    RedrawAt(Instant),
    Custom(Message),
    Sequence(Vec<Update<Message>>),
}
```

The update of an [`Editor`](trait.Editor.md), returned by [`State::update`](struct.State.md).

**Source:** [editor.rs](../../../src/iced_core/text/editor.rs.md) (lines 945-966)

## Variants

### `Action(Action)`

An [`Action`](enum.Action.md) must be performed in the [`Editor`](trait.Editor.md).

### `Focus`

The [`Editor`](trait.Editor.md) just gained focus.

### `Unfocus`

The [`Editor`](trait.Editor.md) just lost focus.

### `InputMethod`

The [`Editor`](trait.Editor.md) changed its [`InputMethod`](../../input_method/enum.InputMethod.md).

### `Release`

A mouse press was released in the [`Editor`](trait.Editor.md).

### `Copy(String)`

The [`Editor`](trait.Editor.md) must copy some text to the clipboard.

### `Paste`

The [`Editor`](trait.Editor.md) must paste the clipboard contents.

### `RedrawAt(Instant)`

The [`Editor`](trait.Editor.md) must be redrawn at the given [`Instant`](../../time/struct.Instant.md).

### `Custom(Message)`

The [`Editor`](trait.Editor.md) produced a custom message that must be published.

### `Sequence(Vec<Update<Message>>)`

The [`Editor`](trait.Editor.md) produced a sequence of updates.

## Auto Trait Implementations

- `impl<Message> Freeze for Update<Message> where Message: Freeze,`
- `impl<Message> RefUnwindSafe for Update<Message> where Message: RefUnwindSafe,`
- `impl<Message> Send for Update<Message> where Message: Send,`
- `impl<Message> Sync for Update<Message> where Message: Sync,`
- `impl<Message> Unpin for Update<Message> where Message: Unpin,`
- `impl<Message> UnsafeUnpin for Update<Message> where Message: UnsafeUnpin,`
- `impl<Message> UnwindSafe for Update<Message> where Message: UnwindSafe,`
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
