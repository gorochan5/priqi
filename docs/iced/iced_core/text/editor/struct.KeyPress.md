---
title: "KeyPress"
kind: struct
path: "iced_core::text::editor::KeyPress"
crate: "iced_core"
module: "iced_core::text::editor"
file: "iced_core/text/editor/struct.KeyPress.md"
docs_url: "https://docs.iced.rs/iced_core/text/editor/struct.KeyPress.html"
source_file: "src/iced_core/text/editor.rs"
source_lines: "828-847"
---

# `iced_core::text::editor::KeyPress`

```rust
pub struct KeyPress {
    pub key: Key,
    pub modified_key: Key,
    pub physical_key: Physical,
    pub modifiers: Modifiers,
    pub text: Option<SmolStr>,
    pub is_focused: bool,
}
```

A key press.

**Source:** [editor.rs](../../../src/iced_core/text/editor.rs.md) (lines 828-847)

## Fields

### `key: Key`

The original key pressed without modifiers applied to it.

You should use this key for combinations (e.g. Ctrl+C).

### `modified_key: Key`

The key pressed with modifiers applied to it.

You should use this key for any single key bindings (e.g. motions).

### `physical_key: Physical`

The physical key pressed.

You should use this key for layout-independent bindings.

### `modifiers: Modifiers`

The state of the keyboard modifiers.

### `text: Option<SmolStr>`

The text produced by the key press.

### `is_focused: bool`

Whether the [`Editor`](trait.Editor.md) is focused.

## Trait Implementations

### `impl Clone for KeyPress`

#### `clone`

```rust
fn clone(&self) -> KeyPress
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for KeyPress`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl PartialEq for KeyPress`

#### `eq`

```rust
fn eq(&self, other: &KeyPress) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Eq for KeyPress`

### `impl StructuralPartialEq for KeyPress`

## Auto Trait Implementations

- `impl Freeze for KeyPress`
- `impl RefUnwindSafe for KeyPress`
- `impl Send for KeyPress`
- `impl Sync for KeyPress`
- `impl Unpin for KeyPress`
- `impl UnsafeUnpin for KeyPress`
- `impl UnwindSafe for KeyPress`
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
