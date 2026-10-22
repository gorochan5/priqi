---
title: "Action"
kind: enum
path: "iced_runtime::Action"
crate: "iced_runtime"
module: "iced_runtime"
file: "iced_runtime/enum.Action.md"
docs_url: "https://docs.iced.rs/iced_runtime/enum.Action.html"
source_file: "src/iced_runtime/lib.rs"
source_lines: "34-78"
---

# `iced_runtime::Action`

```rust
pub enum Action<T> {
    Output(T),
    Widget(Box<dyn Operation>),
    Clipboard(Action),
    Window(Action),
    System(Action),
    Font(Action),
    Image(Action),
    Backend(Action),
    Event {
        window: Id,
        event: Event,
    },
    Tick,
    Reload,
    Exit,
}
```

An action that the iced runtime can perform.

**Source:** [lib.rs](../src/iced_runtime/lib.rs.md) (lines 34-78)

## Variants

### `Output(T)`

Output some value.

### `Widget(Box<dyn Operation>)`

Run a widget operation.

### `Clipboard(Action)`

Run a clipboard action.

### `Window(Action)`

Run a window action.

### `System(Action)`

Run a system action.

### `Font(Action)`

Run a font action.

### `Image(Action)`

Run an image action.

### `Backend(Action)`

Run a backend action.

### `Event`

Produce an event.

**Fields**

- `window: Id` — The `window::Id` of the event.
- `event: Event` — The [`Event`] to be produced.

### `Tick`

Poll any resources that may have pending computations.

### `Reload`

Recreate all user interfaces and redraw all windows.

### `Exit`

Exits the runtime.

This will normally close any application windows and terminate the runtime loop.

## Methods

### `impl<T> Action<T>`

#### `widget`

```rust
pub fn widget(operation: impl Operation + 'static) -> Self
```

Creates a new [`Action::Widget`](enum.Action.md) with the given `widget::Operation`.

## Trait Implementations

### `impl<T> Debug for Action<T> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl<T> !Freeze for Action<T>`
- `impl<T> !RefUnwindSafe for Action<T>`
- `impl<T> Send for Action<T> where T: Send,`
- `impl<T> !Sync for Action<T>`
- `impl<T> Unpin for Action<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Action<T> where T: UnsafeUnpin,`
- `impl<T> !UnwindSafe for Action<T>`
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
