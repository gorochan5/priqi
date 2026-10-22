---
title: "State"
kind: enum
path: "iced_runtime::user_interface::State"
crate: "iced_runtime"
module: "iced_runtime::user_interface"
file: "iced_runtime/user_interface/enum.State.md"
docs_url: "https://docs.iced.rs/iced_runtime/user_interface/enum.State.html"
source_file: "src/iced_runtime/user_interface.rs"
source_lines: "684-702"
---

# `iced_runtime::user_interface::State`

```rust
pub enum State {
    Outdated,
    Updated {
        mouse_interaction: Interaction,
        redraw_request: RedrawRequest,
        input_method: InputMethod,
        clipboard: Clipboard,
        change: Change,
    },
}
```

The resulting state after updating a [`UserInterface`](struct.UserInterface.md).

**Source:** [user_interface.rs](../../src/iced_runtime/user_interface.rs.md) (lines 684-702)

## Variants

### `Outdated`

The [`UserInterface`](struct.UserInterface.md) is outdated and needs to be rebuilt.

### `Updated`

The [`UserInterface`](struct.UserInterface.md) is up-to-date and can be reused without rebuilding.

**Fields**

- `mouse_interaction: Interaction` — The current [`mouse::Interaction`] of the user interface.
- `redraw_request: RedrawRequest` — The [`window::RedrawRequest`] describing when a redraw should be performed.
- `input_method: InputMethod` — The current [`InputMethod`] strategy of the user interface.
- `clipboard: Clipboard` — The set of [`Clipboard`] requests that the user interface has produced.
- `change: Change` — The [`Change`](enum.Change.md) that the [`UserInterface`](struct.UserInterface.md) experienced.

## Trait Implementations

### `impl Debug for State`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl !Freeze for State`
- `impl RefUnwindSafe for State`
- `impl Send for State`
- `impl Sync for State`
- `impl Unpin for State`
- `impl UnsafeUnpin for State`
- `impl UnwindSafe for State`
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

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`
