---
title: "AbsoluteOffset"
kind: struct
path: "iced_core::widget::operation::scrollable::AbsoluteOffset"
crate: "iced_core"
module: "iced_core::widget::operation::scrollable"
file: "iced_core/widget/operation/scrollable/struct.AbsoluteOffset.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/scrollable/struct.AbsoluteOffset.html"
source_file: "src/iced_core/widget/operation/scrollable.rs"
source_lines: "151-156"
---

# `iced_core::widget::operation::scrollable::AbsoluteOffset`

```rust
pub struct AbsoluteOffset<T = f32> {
    pub x: T,
    pub y: T,
}
```

The amount of absolute offset in each direction of a [`Scrollable`](trait.Scrollable.md).

**Source:** [scrollable.rs](../../../../src/iced_core/widget/operation/scrollable.rs.md) (lines 151-156)

## Fields

### `x: T`

The amount of horizontal offset

### `y: T`

The amount of vertical offset

## Trait Implementations

### `impl Add<Vector> for AbsoluteOffset`

#### `Output`

```rust
type Output = AbsoluteOffset
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, translation: Vector) -> Self::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T: Clone> Clone for AbsoluteOffset<T>`

#### `clone`

```rust
fn clone(&self) -> AbsoluteOffset<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T: Debug> Debug for AbsoluteOffset<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T: Default> Default for AbsoluteOffset<T>`

#### `default`

```rust
fn default() -> AbsoluteOffset<T>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl From<AbsoluteOffset> for AbsoluteOffset<Option<f32>>`

#### `from`

```rust
fn from(offset: AbsoluteOffset) -> Self
```

Converts to this type from the input type.

### `impl<T: PartialEq> PartialEq for AbsoluteOffset<T>`

#### `eq`

```rust
fn eq(&self, other: &AbsoluteOffset<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T: Copy> Copy for AbsoluteOffset<T>`

### `impl<T> StructuralPartialEq for AbsoluteOffset<T>`

## Auto Trait Implementations

- `impl<T> Freeze for AbsoluteOffset<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for AbsoluteOffset<T> where T: RefUnwindSafe,`
- `impl<T> Send for AbsoluteOffset<T> where T: Send,`
- `impl<T> Sync for AbsoluteOffset<T> where T: Sync,`
- `impl<T> Unpin for AbsoluteOffset<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for AbsoluteOffset<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for AbsoluteOffset<T> where T: UnwindSafe,`
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
