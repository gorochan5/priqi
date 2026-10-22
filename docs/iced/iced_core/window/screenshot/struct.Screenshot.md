---
title: "Screenshot"
kind: struct
path: "iced_core::window::screenshot::Screenshot"
crate: "iced_core"
module: "iced_core::window::screenshot"
file: "iced_core/window/screenshot/struct.Screenshot.md"
docs_url: "https://docs.iced.rs/iced_core/window/screenshot/struct.Screenshot.html"
source_file: "src/iced_core/window/screenshot.rs"
source_lines: "10-18"
---

# `iced_core::window::screenshot::Screenshot`

```rust
pub struct Screenshot {
    pub rgba: Bytes,
    pub size: Size<u32>,
    pub scale_factor: f32,
}
```

Data of a screenshot, captured with `window::screenshot()`.

The `bytes` of this screenshot will always be ordered as `RGBA` in the `sRGB` color space.

**Source:** [screenshot.rs](../../../src/iced_core/window/screenshot.rs.md) (lines 10-18)

## Fields

### `rgba: Bytes`

The RGBA bytes of the [`Screenshot`](struct.Screenshot.md).

### `size: Size<u32>`

The size of the [`Screenshot`](struct.Screenshot.md) in physical pixels.

### `scale_factor: f32`

The scale factor of the [`Screenshot`](struct.Screenshot.md). This can be useful when converting between widget bounds (which are in logical pixels) to crop screenshots.

## Methods

### `impl Screenshot`

#### `new`

```rust
pub fn new(rgba: impl Into<Bytes>, size: Size<u32>, scale_factor: f32) -> Self
```

Creates a new [`Screenshot`](struct.Screenshot.md).

#### `crop`

```rust
pub fn crop(&self, region: Rectangle<u32>) -> Result<Self, CropError>
```

Crops a [`Screenshot`](struct.Screenshot.md) to the provided `region`. This will always be relative to the top-left corner of the [`Screenshot`](struct.Screenshot.md).

## Trait Implementations

### `impl AsRef<[u8]> for Screenshot`

#### `as_ref`

```rust
fn as_ref(&self) -> &[u8] ⓘ
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl Clone for Screenshot`

#### `clone`

```rust
fn clone(&self) -> Screenshot
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Screenshot`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<Screenshot> for Bytes`

#### `from`

```rust
fn from(screenshot: Screenshot) -> Self
```

Converts to this type from the input type.

## Auto Trait Implementations

- `impl !Freeze for Screenshot`
- `impl RefUnwindSafe for Screenshot`
- `impl Send for Screenshot`
- `impl Sync for Screenshot`
- `impl Unpin for Screenshot`
- `impl UnsafeUnpin for Screenshot`
- `impl UnwindSafe for Screenshot`
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
