---
title: "Element"
kind: struct
path: "iced_core::overlay::Element"
crate: "iced_core"
module: "iced_core::overlay"
file: "iced_core/overlay/struct.Element.md"
docs_url: "https://docs.iced.rs/iced_core/overlay/struct.Element.html"
source_file: "src/iced_core/overlay/element.rs"
source_lines: "10-12"
---

# `iced_core::overlay::Element`

```rust
pub struct Element<'a, Message, Theme, Renderer> { /* private fields */ }
```

A generic [`Overlay`](trait.Overlay.md).

**Source:** [element.rs](../../src/iced_core/overlay/element.rs.md) (lines 10-12)

## Methods

### `impl<'a, Message, Theme, Renderer> Element<'a, Message, Theme, Renderer> where Renderer: Renderer,`

#### `new`

```rust
pub fn new(overlay: Box<dyn Overlay<Message, Theme, Renderer> + 'a>) -> Self
```

Creates a new [`Element`](struct.Element.md) containing the given [`Overlay`](trait.Overlay.md).

#### `as_overlay`

```rust
pub fn as_overlay(&self) -> &dyn Overlay<Message, Theme, Renderer>
```

Returns a reference to the [`Overlay`](trait.Overlay.md) of the [`Element`](struct.Element.md),

#### `as_overlay_mut`

```rust
pub fn as_overlay_mut(&mut self) -> &mut dyn Overlay<Message, Theme, Renderer>
```

Returns a mutable reference to the [`Overlay`](trait.Overlay.md) of the [`Element`](struct.Element.md),

#### `map`

```rust
pub fn map<B>( self, f: &'a dyn Fn(Message) -> B, ) -> Element<'a, B, Theme, Renderer> where Message: 'a, Theme: 'a, Renderer: 'a, B: 'a,
```

Applies a transformation to the produced message of the [`Element`](struct.Element.md).

## Auto Trait Implementations

- `impl<'a, Message, Theme, Renderer> Freeze for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !RefUnwindSafe for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !Send for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !Sync for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> Unpin for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> UnsafeUnpin for Element<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !UnwindSafe for Element<'a, Message, Theme, Renderer>`
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
