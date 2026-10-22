---
title: "Svg"
kind: struct
path: "iced_core::svg::Svg"
crate: "iced_core"
module: "iced_core::svg"
file: "iced_core/svg/struct.Svg.md"
docs_url: "https://docs.iced.rs/iced_core/svg/struct.Svg.html"
source_file: "src/iced_core/svg.rs"
source_lines: "12-32"
---

# `iced_core::svg::Svg`

```rust
pub struct Svg<H = Handle> {
    pub handle: H,
    pub color: Option<Color>,
    pub rotation: Radians,
    pub opacity: f32,
}
```

A raster image that can be drawn.

**Source:** [svg.rs](../../src/iced_core/svg.rs.md) (lines 12-32)

## Fields

### `handle: H`

The handle of the [`Svg`](struct.Svg.md).

### `color: Option<Color>`

The [`Color`](../color/struct.Color.md) filter to be applied to the [`Svg`](struct.Svg.md).

If some [`Color`](../color/struct.Color.md) is set, the whole [`Svg`](struct.Svg.md) will be painted with it—ignoring any intrinsic colors.

This can be useful for coloring icons programmatically (e.g. with a theme).

### `rotation: Radians`

The rotation to be applied to the image; on its center.

### `opacity: f32`

The opacity of the [`Svg`](struct.Svg.md).

0 means transparent. 1 means opaque.

## Methods

### `impl Svg<Handle>`

#### `new`

```rust
pub fn new(handle: impl Into<Handle>) -> Self
```

Creates a new [`Svg`](struct.Svg.md) with the given handle.

#### `color`

```rust
pub fn color(self, color: impl Into<Color>) -> Self
```

Sets the [`Color`](../color/struct.Color.md) filter of the [`Svg`](struct.Svg.md).

#### `rotation`

```rust
pub fn rotation(self, rotation: impl Into<Radians>) -> Self
```

Sets the rotation of the [`Svg`](struct.Svg.md).

#### `opacity`

```rust
pub fn opacity(self, opacity: impl Into<f32>) -> Self
```

Sets the opacity of the [`Svg`](struct.Svg.md).

## Trait Implementations

### `impl<H: Clone> Clone for Svg<H>`

#### `clone`

```rust
fn clone(&self) -> Svg<H>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<H: Debug> Debug for Svg<H>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<&Handle> for Svg`

#### `from`

```rust
fn from(handle: &Handle) -> Self
```

Converts to this type from the input type.

### `impl<H: PartialEq> PartialEq for Svg<H>`

#### `eq`

```rust
fn eq(&self, other: &Svg<H>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<H> StructuralPartialEq for Svg<H>`

## Auto Trait Implementations

- `impl<H> Freeze for Svg<H> where H: Freeze,`
- `impl<H> RefUnwindSafe for Svg<H> where H: RefUnwindSafe,`
- `impl<H> Send for Svg<H> where H: Send,`
- `impl<H> Sync for Svg<H> where H: Sync,`
- `impl<H> Unpin for Svg<H> where H: Unpin,`
- `impl<H> UnsafeUnpin for Svg<H> where H: UnsafeUnpin,`
- `impl<H> UnwindSafe for Svg<H> where H: UnwindSafe,`
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
