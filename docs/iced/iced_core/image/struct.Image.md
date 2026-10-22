---
title: "Image"
kind: struct
path: "iced_core::image::Image"
crate: "iced_core"
module: "iced_core::image"
file: "iced_core/image/struct.Image.md"
docs_url: "https://docs.iced.rs/iced_core/image/struct.Image.html"
source_file: "src/iced_core/image.rs"
source_lines: "14-33"
---

# `iced_core::image::Image`

```rust
pub struct Image<H = Handle> {
    pub handle: H,
    pub filter_method: FilterMethod,
    pub rotation: Radians,
    pub border_radius: Radius,
    pub opacity: f32,
}
```

A raster image that can be drawn.

**Source:** [image.rs](../../src/iced_core/image.rs.md) (lines 14-33)

## Fields

### `handle: H`

The handle of the image.

### `filter_method: FilterMethod`

The filter method of the image.

### `rotation: Radians`

The rotation to be applied to the image; on its center.

### `border_radius: Radius`

The border radius of the [`Image`](struct.Image.md).

Currently, this will only be applied to the `clip_bounds`.

### `opacity: f32`

The opacity of the image.

0 means transparent. 1 means opaque.

## Methods

### `impl Image<Handle>`

#### `new`

```rust
pub fn new(handle: impl Into<Handle>) -> Self
```

Creates a new [`Image`](struct.Image.md) with the given handle.

#### `filter_method`

```rust
pub fn filter_method(self, filter_method: FilterMethod) -> Self
```

Sets the filter method of the [`Image`](struct.Image.md).

#### `rotation`

```rust
pub fn rotation(self, rotation: impl Into<Radians>) -> Self
```

Sets the rotation of the [`Image`](struct.Image.md).

#### `opacity`

```rust
pub fn opacity(self, opacity: impl Into<f32>) -> Self
```

Sets the opacity of the [`Image`](struct.Image.md).

## Trait Implementations

### `impl<H: Clone> Clone for Image<H>`

#### `clone`

```rust
fn clone(&self) -> Image<H>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<H: Debug> Debug for Image<H>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<&Handle> for Image`

#### `from`

```rust
fn from(handle: &Handle) -> Self
```

Converts to this type from the input type.

### `impl<H: PartialEq> PartialEq for Image<H>`

#### `eq`

```rust
fn eq(&self, other: &Image<H>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<H> StructuralPartialEq for Image<H>`

## Auto Trait Implementations

- `impl<H> Freeze for Image<H> where H: Freeze,`
- `impl<H> RefUnwindSafe for Image<H> where H: RefUnwindSafe,`
- `impl<H> Send for Image<H> where H: Send,`
- `impl<H> Sync for Image<H> where H: Sync,`
- `impl<H> Unpin for Image<H> where H: Unpin,`
- `impl<H> UnsafeUnpin for Image<H> where H: UnsafeUnpin,`
- `impl<H> UnwindSafe for Image<H> where H: UnwindSafe,`
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
