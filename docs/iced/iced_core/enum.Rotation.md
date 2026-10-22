---
title: "Rotation"
kind: enum
path: "iced_core::Rotation"
crate: "iced_core"
module: "iced_core"
file: "iced_core/enum.Rotation.md"
docs_url: "https://docs.iced.rs/iced_core/enum.Rotation.html"
source_file: "src/iced_core/rotation.rs"
source_lines: "9-24"
---

# `iced_core::Rotation`

```rust
pub enum Rotation {
    Floating(Radians),
    Solid(Radians),
}
```

The strategy used to rotate the content.

This is used to control the behavior of the layout when the content is rotated by a certain angle.

**Source:** [rotation.rs](../src/iced_core/rotation.rs.md) (lines 9-24)

## Variants

### `Floating(Radians)`

The element will float while rotating. The layout will be kept exactly as it was before the rotation.

This is especially useful when used for animations, as it will avoid the layout being shifted or resized when smoothly i.e. an icon.

This is the default.

### `Solid(Radians)`

The element will be solid while rotating. The layout will be adjusted to fit the rotated content.

This allows you to rotate an image and have the layout adjust to fit the new size of the image.

## Methods

### `impl Rotation`

#### `radians`

```rust
pub fn radians(self) -> Radians
```

Returns the angle of the [`Rotation`](enum.Rotation.md) in [`Radians`](struct.Radians.md).

#### `radians_mut`

```rust
pub fn radians_mut(&mut self) -> &mut Radians
```

Returns a mutable reference to the angle of the [`Rotation`](enum.Rotation.md) in [`Radians`](struct.Radians.md).

#### `degrees`

```rust
pub fn degrees(self) -> Degrees
```

Returns the angle of the [`Rotation`](enum.Rotation.md) in [`Degrees`](struct.Degrees.md).

#### `apply`

```rust
pub fn apply(self, size: Size) -> Size
```

Applies the [`Rotation`](enum.Rotation.md) to the given [`Size`](struct.Size.md), returning the minimum [`Size`](struct.Size.md) containing the rotated one.

## Trait Implementations

### `impl Clone for Rotation`

#### `clone`

```rust
fn clone(&self) -> Rotation
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Rotation`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Rotation`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl From<Radians> for Rotation`

#### `from`

```rust
fn from(radians: Radians) -> Self
```

Converts to this type from the input type.

### `impl From<f32> for Rotation`

#### `from`

```rust
fn from(radians: f32) -> Self
```

Converts to this type from the input type.

### `impl PartialEq for Rotation`

#### `eq`

```rust
fn eq(&self, other: &Rotation) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Rotation`

### `impl StructuralPartialEq for Rotation`

## Auto Trait Implementations

- `impl Freeze for Rotation`
- `impl RefUnwindSafe for Rotation`
- `impl Send for Rotation`
- `impl Sync for Rotation`
- `impl Unpin for Rotation`
- `impl UnsafeUnpin for Rotation`
- `impl UnwindSafe for Rotation`
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
