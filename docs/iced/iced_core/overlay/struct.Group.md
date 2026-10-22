---
title: "Group"
kind: struct
path: "iced_core::overlay::Group"
crate: "iced_core"
module: "iced_core::overlay"
file: "iced_core/overlay/struct.Group.md"
docs_url: "https://docs.iced.rs/iced_core/overlay/struct.Group.html"
source_file: "src/iced_core/overlay/group.rs"
source_lines: "9-11"
---

# `iced_core::overlay::Group`

```rust
pub struct Group<'a, Message, Theme, Renderer> { /* private fields */ }
```

A container of nested overlays.

**Source:** [group.rs](../../src/iced_core/overlay/group.rs.md) (lines 9-11)

## Methods

### `impl<'a, Message, Theme, Renderer> Group<'a, Message, Theme, Renderer> where Renderer: Renderer,`

#### `new`

```rust
pub fn new(children: Vec<Element<'a, Message, Theme, Renderer>>) -> Self
```

Creates a [`Group`](struct.Group.md) container for the given overlays.

The overlays are sorted by their [`index`](trait.Overlay.md).

#### `draw`

```rust
pub fn draw( &mut self, renderer: &mut Renderer, theme: &Theme, style: &Style, cursor: Cursor, )
```

Draws the [`Group`](struct.Group.md) overlay using the associated `Renderer`.

#### `operate`

```rust
pub fn operate(&mut self, renderer: &Renderer, operation: &mut dyn Operation)
```

Applies a [`widget::Operation`](../widget/operation/trait.Operation.md) to the [`Group`](struct.Group.md) overlay.

#### `update`

```rust
pub fn update( &mut self, event: &Event, cursor: Cursor, renderer: &Renderer, shell: &mut Shell<'_, Message>, )
```

Processes a runtime [`Event`](../event/enum.Event.md).

#### `mouse_interaction`

```rust
pub fn mouse_interaction( &mut self, cursor: Cursor, renderer: &Renderer, ) -> Interaction
```

Returns the current [`mouse::Interaction`](../mouse/enum.Interaction.md) of the [`Group`](struct.Group.md) overlay.

## Auto Trait Implementations

- `impl<'a, Message, Theme, Renderer> Freeze for Group<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !RefUnwindSafe for Group<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !Send for Group<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !Sync for Group<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> Unpin for Group<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> UnsafeUnpin for Group<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !UnwindSafe for Group<'a, Message, Theme, Renderer>`
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
