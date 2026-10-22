---
title: "Tree"
kind: struct
path: "iced_core::widget::tree::Tree"
crate: "iced_core"
module: "iced_core::widget::tree"
file: "iced_core/widget/tree/struct.Tree.md"
docs_url: "https://docs.iced.rs/iced_core/widget/tree/struct.Tree.html"
source_file: "src/iced_core/widget/tree.rs"
source_lines: "11-26"
---

# `iced_core::widget::tree::Tree`

```rust
pub struct Tree {
    pub tag: Tag,
    pub state: State,
    pub translation: Vector,
    pub size: Size,
    pub children: Vec<Tree>,
}
```

A persistent state widget tree.

A [`Tree`](struct.Tree.md) is normally associated with a specific widget in the widget tree.

**Source:** [tree.rs](../../../src/iced_core/widget/tree.rs.md) (lines 11-26)

## Fields

### `tag: Tag`

The tag of the [`Tree`](struct.Tree.md).

### `state: State`

The [`State`](enum.State.md) of the [`Tree`](struct.Tree.md).

### `translation: Vector`

The translation of the widget, relative to its parent.

### `size: Size`

The size of the widget.

### `children: Vec<Tree>`

The children of the widget.

## Methods

### `impl Tree`

#### `empty`

```rust
pub fn empty() -> Self
```

Creates an empty, stateless [`Tree`](struct.Tree.md) with no children.

#### `new`

```rust
pub fn new<Message, Theme, Renderer>( widget: &impl Widget<Message, Theme, Renderer>, ) -> Self where Renderer: Renderer,
```

Creates a new [`Tree`](struct.Tree.md) for the provided [`Widget`](../trait.Widget.md).

#### `diff`

```rust
pub fn diff<Message, Theme, Renderer>( &mut self, new: &mut impl Widget<Message, Theme, Renderer>, ) where Renderer: Renderer,
```

Reconciles the current tree with the provided [`Widget`](../trait.Widget.md).

If the tag of the [`Widget`](../trait.Widget.md) matches the tag of the [`Tree`](struct.Tree.md), then the [`Widget`](../trait.Widget.md) proceeds with the reconciliation (i.e. [`Widget::diff`](../trait.Widget.md) is called).

Otherwise, the whole [`Tree`](struct.Tree.md) is recreated.

#### `diff_children`

```rust
pub fn diff_children<Message, Theme, Renderer>( &mut self, new_children: &mut [impl Widget<Message, Theme, Renderer>], ) where Renderer: Renderer,
```

Reconciles the children of the tree with the provided list of widgets.

#### `diff_children_custom`

```rust
pub fn diff_children_custom<T>( &mut self, new_children: &mut [T], diff: impl Fn(&mut Tree, &mut T), new_state: impl Fn(&T) -> Self, )
```

Reconciles the children of the tree with the provided list of widgets using custom logic both for diffing and creating new widget state.

## Trait Implementations

### `impl Debug for Tree`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl Freeze for Tree`
- `impl !RefUnwindSafe for Tree`
- `impl !Send for Tree`
- `impl !Sync for Tree`
- `impl Unpin for Tree`
- `impl UnsafeUnpin for Tree`
- `impl !UnwindSafe for Tree`
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
