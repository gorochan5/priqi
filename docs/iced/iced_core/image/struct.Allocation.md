---
title: "Allocation"
kind: struct
path: "iced_core::image::Allocation"
crate: "iced_core"
module: "iced_core::image"
file: "iced_core/image/struct.Allocation.md"
docs_url: "https://docs.iced.rs/iced_core/image/struct.Allocation.html"
source_file: "src/iced_core/image.rs"
source_lines: "237"
---

# `iced_core::image::Allocation`

```rust
pub struct Allocation(/* private fields */);
```

A memory allocation of a [`Handle`](enum.Handle.md), often in GPU memory.

Renderers tend to decode and upload image data concurrently to avoid blocking the user interface. This means that when you use a [`Handle`](enum.Handle.md) in a widget, there may be a slight frame delay until it is finally visible. If you are animating images, this can cause undesirable flicker.

When you obtain an [`Allocation`](struct.Allocation.md) explicitly, you get the guarantee that using a [`Handle`](enum.Handle.md) will draw the corresponding [`Image`](struct.Image.md) immediately in the next frame.

This guarantee is valid as long as you hold an [`Allocation`](struct.Allocation.md). Only when you drop all its clones, the renderer may choose to free the memory of the [`Handle`](enum.Handle.md). Be careful!

**Source:** [image.rs](../../src/iced_core/image.rs.md) (lines 237)

## Methods

### `impl Allocation`

#### `downgrade`

```rust
pub fn downgrade(&self) -> Weak<Memory>
```

Returns a weak reference to the [`Memory`](struct.Memory.md) of the [`Allocation`](struct.Allocation.md).

#### `upgrade`

```rust
pub fn upgrade(weak: &Weak<Memory>) -> Option<Allocation>
```

Upgrades a [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html) memory reference to an [`Allocation`](struct.Allocation.md).

#### `handle`

```rust
pub fn handle(&self) -> &Handle
```

Returns the [`Handle`](enum.Handle.md) of this [`Allocation`](struct.Allocation.md).

#### `size`

```rust
pub fn size(&self) -> Size<u32>
```

Returns the [`Size`](../struct.Size.md) of the image of this [`Allocation`](struct.Allocation.md).

## Trait Implementations

### `impl Clone for Allocation`

#### `clone`

```rust
fn clone(&self) -> Allocation
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Allocation`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl PartialEq for Allocation`

#### `eq`

```rust
fn eq(&self, other: &Allocation) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Eq for Allocation`

### `impl StructuralPartialEq for Allocation`

## Auto Trait Implementations

- `impl Freeze for Allocation`
- `impl RefUnwindSafe for Allocation`
- `impl Send for Allocation`
- `impl Sync for Allocation`
- `impl Unpin for Allocation`
- `impl UnsafeUnpin for Allocation`
- `impl UnwindSafe for Allocation`
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
