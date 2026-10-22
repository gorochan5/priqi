---
title: "RelativeOffset"
kind: struct
path: "iced_core::widget::operation::scrollable::RelativeOffset"
crate: "iced_core"
module: "iced_core::widget::operation::scrollable"
file: "iced_core/widget/operation/scrollable/struct.RelativeOffset.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/scrollable/struct.RelativeOffset.html"
source_file: "src/iced_core/widget/operation/scrollable.rs"
source_lines: "182-187"
---

# `iced_core::widget::operation::scrollable::RelativeOffset`

```rust
pub struct RelativeOffset<T = f32> {
    pub x: T,
    pub y: T,
}
```

The amount of relative offset in each direction of a [`Scrollable`](trait.Scrollable.md).

A value of `0.0` means start, while `1.0` means end.

**Source:** [scrollable.rs](../../../../src/iced_core/widget/operation/scrollable.rs.md) (lines 182-187)

## Fields

### `x: T`

The amount of horizontal offset

### `y: T`

The amount of vertical offset

## Methods

### `impl RelativeOffset`

#### `START`

```rust
pub const START: Self
```

A relative offset that points to the top-left of a [`Scrollable`](trait.Scrollable.md).

#### `END`

```rust
pub const END: Self
```

A relative offset that points to the bottom-right of a [`Scrollable`](trait.Scrollable.md).

## Trait Implementations

### `impl<T: Clone> Clone for RelativeOffset<T>`

#### `clone`

```rust
fn clone(&self) -> RelativeOffset<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T: Debug> Debug for RelativeOffset<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T: Default> Default for RelativeOffset<T>`

#### `default`

```rust
fn default() -> RelativeOffset<T>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl From<RelativeOffset> for RelativeOffset<Option<f32>>`

#### `from`

```rust
fn from(offset: RelativeOffset) -> Self
```

Converts to this type from the input type.

### `impl<T: PartialEq> PartialEq for RelativeOffset<T>`

#### `eq`

```rust
fn eq(&self, other: &RelativeOffset<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T: Copy> Copy for RelativeOffset<T>`

### `impl<T> StructuralPartialEq for RelativeOffset<T>`

## Auto Trait Implementations

- `impl<T> Freeze for RelativeOffset<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for RelativeOffset<T> where T: RefUnwindSafe,`
- `impl<T> Send for RelativeOffset<T> where T: Send,`
- `impl<T> Sync for RelativeOffset<T> where T: Sync,`
- `impl<T> Unpin for RelativeOffset<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for RelativeOffset<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for RelativeOffset<T> where T: UnwindSafe,`
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
