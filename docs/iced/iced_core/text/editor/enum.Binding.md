---
title: "Binding"
kind: enum
path: "iced_core::text::editor::Binding"
crate: "iced_core"
module: "iced_core::text::editor"
file: "iced_core/text/editor/enum.Binding.md"
docs_url: "https://docs.iced.rs/iced_core/text/editor/enum.Binding.html"
source_file: "src/iced_core/text/editor.rs"
source_lines: "781-824"
---

# `iced_core::text::editor::Binding`

```rust
pub enum Binding<Message> {
Show 21 variants    Unfocus,
    Copy,
    Cut,
    Paste,
    Undo,
    Redo,
    Move(Motion),
    Select(Motion),
    SelectWord,
    SelectLine,
    SelectAll,
    Insert(char),
    Enter,
    Backspace,
    BackspaceWord,
    BackspaceLine,
    Delete,
    DeleteWord,
    DeleteLine,
    Sequence(Vec<Self>),
    Custom(Message),
}
```

A binding to an action in the [`Editor`](trait.Editor.md).

**Source:** [editor.rs](../../../src/iced_core/text/editor.rs.md) (lines 781-824)

## Variants

### `Unfocus`

Unfocus the [`Editor`](trait.Editor.md).

### `Copy`

Copy the selection of the [`Editor`](trait.Editor.md).

### `Cut`

Cut the selection of the [`Editor`](trait.Editor.md).

### `Paste`

Paste the clipboard contents in the [`Editor`](trait.Editor.md).

### `Undo`

Undo the last change peformed in the [`Editor`](trait.Editor.md).

### `Redo`

Redo the last change undone in the [`Editor`](trait.Editor.md).

### `Move(Motion)`

Apply a [`Motion`](enum.Motion.md).

### `Select(Motion)`

Select text with a given [`Motion`](enum.Motion.md).

### `SelectWord`

Select the word at the current cursor.

### `SelectLine`

Select the line at the current cursor.

### `SelectAll`

Select the entire buffer.

### `Insert(char)`

Insert the given character.

### `Enter`

Break the current line.

### `Backspace`

Delete the previous character.

### `BackspaceWord`

Delete the word before the cursor.

### `BackspaceLine`

Delete the line before the cursor.

### `Delete`

Delete the next character.

### `DeleteWord`

Delete the word after the cursor.

### `DeleteLine`

Delete the line after the cursor.

### `Sequence(Vec<Self>)`

A sequence of bindings to execute.

### `Custom(Message)`

Produce the given message.

## Methods

### `impl<Message> Binding<Message>`

#### `from_key_press`

```rust
pub fn from_key_press(event: KeyPress) -> Option<Self>
```

Returns the default [`Binding`](enum.Binding.md) for the given key press.

## Trait Implementations

### `impl<Message: Clone> Clone for Binding<Message>`

#### `clone`

```rust
fn clone(&self) -> Binding<Message>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<Message: Debug> Debug for Binding<Message>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<Message: PartialEq> PartialEq for Binding<Message>`

#### `eq`

```rust
fn eq(&self, other: &Binding<Message>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<Message> StructuralPartialEq for Binding<Message>`

## Auto Trait Implementations

- `impl<Message> Freeze for Binding<Message> where Message: Freeze,`
- `impl<Message> RefUnwindSafe for Binding<Message> where Message: RefUnwindSafe,`
- `impl<Message> Send for Binding<Message> where Message: Send,`
- `impl<Message> Sync for Binding<Message> where Message: Sync,`
- `impl<Message> Unpin for Binding<Message> where Message: Unpin,`
- `impl<Message> UnsafeUnpin for Binding<Message> where Message: UnsafeUnpin,`
- `impl<Message> UnwindSafe for Binding<Message> where Message: UnwindSafe,`
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
