---
title: "Handle"
kind: enum
path: "iced_core::image::Handle"
crate: "iced_core"
module: "iced_core::image"
file: "iced_core/image/enum.Handle.md"
docs_url: "https://docs.iced.rs/iced_core/image/enum.Handle.html"
source_file: "src/iced_core/image.rs"
source_lines: "74-105"
---

# `iced_core::image::Handle`

```rust
pub enum Handle {
    Path(Id, PathBuf),
    Bytes(Id, Bytes),
    Rgba {
        id: Id,
        width: u32,
        height: u32,
        pixels: Bytes,
    },
}
```

A handle of some image data.

**Source:** [image.rs](../../src/iced_core/image.rs.md) (lines 74-105)

## Variants

### `Path(Id, PathBuf)`

A file handle. The image data will be read from the file path.

Use [`from_path`](enum.Handle.md) to create this variant.

### `Bytes(Id, Bytes)`

A handle pointing to some encoded image bytes in-memory.

Use [`from_bytes`](enum.Handle.md) to create this variant.

### `Rgba`

A handle pointing to decoded image pixels in RGBA format.

Use [`from_rgba`](enum.Handle.md) to create this variant.

**Fields**

- `id: Id` — The id of this handle.
- `width: u32` — The width of the image.
- `height: u32` — The height of the image.
- `pixels: Bytes` — The pixels.

## Methods

### `impl Handle`

#### `from_path`

```rust
pub fn from_path<T: Into<PathBuf>>(path: T) -> Handle
```

Creates an image [`Handle`](enum.Handle.md) pointing to the image of the given path.

Makes an educated guess about the image format by examining the data in the file.

#### `from_bytes`

```rust
pub fn from_bytes(bytes: impl Into<Bytes>) -> Handle
```

Creates an image [`Handle`](enum.Handle.md) containing the encoded image data directly.

Makes an educated guess about the image format by examining the given data.

This is useful if you already have your image loaded in-memory, maybe because you downloaded or generated it procedurally.

#### `from_rgba`

```rust
pub fn from_rgba(width: u32, height: u32, pixels: impl Into<Bytes>) -> Handle
```

Creates an image [`Handle`](enum.Handle.md) containing the decoded image pixels directly.

This function expects the pixel data to be provided as a collection of [`Bytes`](../struct.Bytes.md) of RGBA pixels. Therefore, the length of the pixel data should always be `width * height * 4`.

This is useful if you have already decoded your image.

#### `id`

```rust
pub fn id(&self) -> Id
```

Returns the unique identifier of the [`Handle`](enum.Handle.md).

## Trait Implementations

### `impl Clone for Handle`

#### `clone`

```rust
fn clone(&self) -> Handle
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Handle`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<&Handle> for Handle`

#### `from`

```rust
fn from(value: &Handle) -> Self
```

Converts to this type from the input type.

### `impl From<&Handle> for Image`

#### `from`

```rust
fn from(handle: &Handle) -> Self
```

Converts to this type from the input type.

### `impl<T> From<T> for Handle where T: Into<PathBuf>,`

#### `from`

```rust
fn from(path: T) -> Handle
```

Converts to this type from the input type.

### `impl PartialEq for Handle`

#### `eq`

```rust
fn eq(&self, other: &Handle) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Eq for Handle`

### `impl StructuralPartialEq for Handle`

## Auto Trait Implementations

- `impl !Freeze for Handle`
- `impl RefUnwindSafe for Handle`
- `impl Send for Handle`
- `impl Sync for Handle`
- `impl Unpin for Handle`
- `impl UnsafeUnpin for Handle`
- `impl UnwindSafe for Handle`
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
