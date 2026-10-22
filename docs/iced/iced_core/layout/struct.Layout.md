---
title: "Layout"
kind: struct
path: "iced_core::layout::Layout"
crate: "iced_core"
module: "iced_core::layout"
file: "iced_core/layout/struct.Layout.md"
docs_url: "https://docs.iced.rs/iced_core/layout/struct.Layout.html"
source_file: "src/iced_core/layout.rs"
source_lines: "13-17"
---

# `iced_core::layout::Layout`

```rust
pub struct Layout { /* private fields */ }
```

The bounds of a widget and its children, using absolute coordinates.

**Source:** [layout.rs](../../src/iced_core/layout.rs.md) (lines 13-17)

## Methods

### `impl Layout`

#### `new`

```rust
pub fn new(size: Size) -> Self
```

Creates a new [`Layout`](struct.Layout.md) for the given [`Size`](../struct.Size.md) at the origin.

#### `position`

```rust
pub fn position(&self) -> Point
```

Returns the position of the [`Layout`](struct.Layout.md).

#### `move_to`

```rust
pub fn move_to(self, position: impl Into<Point>) -> Self
```

Moves the [`Layout`](struct.Layout.md) to the given position.

#### `bounds`

```rust
pub fn bounds(&self) -> Rectangle
```

Returns the bounds of the [`Layout`](struct.Layout.md).

The returned [`Rectangle`](../struct.Rectangle.md) describes the position and size of the laid out widget.

#### `size`

```rust
pub fn size(&self) -> Size
```

Returns the size of the [`Layout`](struct.Layout.md).

#### `parent`

```rust
pub fn parent(&self) -> Option<Rectangle>
```

Returns the bounds of the parent of this [`Layout`](struct.Layout.md), if any.

#### `iter`

```rust
pub fn iter( self, children: &[Tree], ) -> impl DoubleEndedIterator<Item = (Self, &Tree)> + ExactSizeIterator
```

Returns an iterator over the children of this [`Layout`](struct.Layout.md).

#### `iter_mut`

```rust
pub fn iter_mut( self, children: &mut [Tree], ) -> impl DoubleEndedIterator<Item = (Self, &mut Tree)> + ExactSizeIterator
```

Returns a mutable iterator over the children of this [`Layout`](struct.Layout.md).

## Trait Implementations

### `impl Clone for Layout`

#### `clone`

```rust
fn clone(&self) -> Layout
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Layout`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl PartialEq for Layout`

#### `eq`

```rust
fn eq(&self, other: &Layout) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Layout`

### `impl StructuralPartialEq for Layout`

## Auto Trait Implementations

- `impl Freeze for Layout`
- `impl RefUnwindSafe for Layout`
- `impl Send for Layout`
- `impl Sync for Layout`
- `impl Unpin for Layout`
- `impl UnsafeUnpin for Layout`
- `impl UnwindSafe for Layout`
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
