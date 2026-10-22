---
title: "Padding"
kind: struct
path: "iced_core::padding::Padding"
crate: "iced_core"
module: "iced_core::padding"
file: "iced_core/padding/struct.Padding.md"
docs_url: "https://docs.iced.rs/iced_core/padding/struct.Padding.html"
source_file: "src/iced_core/padding.rs"
source_lines: "36-45"
---

# `iced_core::padding::Padding`

```rust
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}
```

An amount of space to pad for each side of a box

You can leverage the `From` trait to build [`Padding`](struct.Padding.md) conveniently:

```rust
let padding = Padding::from(20);              // 20px on all sides
let padding = Padding::from([10, 20]);        // top/bottom, left/right
```

Normally, the `padding` method of a widget will ask for an `Into<Padding>`, so you can easily write:

```rust
impl Widget {
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        // ...
        self
    }
}

let widget = Widget::new().padding(20);              // 20px on all sides
let widget = Widget::new().padding([10, 20]);        // top/bottom, left/right
```

**Source:** [padding.rs](../../src/iced_core/padding.rs.md) (lines 36-45)

## Fields

### `top: f32`

Top padding

### `right: f32`

Right padding

### `bottom: f32`

Bottom padding

### `left: f32`

Left padding

## Methods

### `impl Padding`

#### `ZERO`

```rust
pub const ZERO: Padding
```

Padding of zero

#### `new`

```rust
pub const fn new(padding: f32) -> Padding
```

Create a [`Padding`](struct.Padding.md) that is equal on all sides.

#### `top`

```rust
pub fn top(self, top: impl Into<Pixels>) -> Self
```

Sets the [`top`](struct.Padding.md) of the [`Padding`](struct.Padding.md).

#### `bottom`

```rust
pub fn bottom(self, bottom: impl Into<Pixels>) -> Self
```

Sets the [`bottom`](struct.Padding.md) of the [`Padding`](struct.Padding.md).

#### `left`

```rust
pub fn left(self, left: impl Into<Pixels>) -> Self
```

Sets the [`left`](struct.Padding.md) of the [`Padding`](struct.Padding.md).

#### `right`

```rust
pub fn right(self, right: impl Into<Pixels>) -> Self
```

Sets the [`right`](struct.Padding.md) of the [`Padding`](struct.Padding.md).

#### `horizontal`

```rust
pub fn horizontal(self, horizontal: impl Into<Pixels>) -> Self
```

Sets the [`left`](struct.Padding.md) and [`right`](struct.Padding.md) of the [`Padding`](struct.Padding.md).

#### `vertical`

```rust
pub fn vertical(self, vertical: impl Into<Pixels>) -> Self
```

Sets the [`top`](struct.Padding.md) and [`bottom`](struct.Padding.md) of the [`Padding`](struct.Padding.md).

#### `x`

```rust
pub fn x(self) -> f32
```

Returns the total amount of horizontal [`Padding`](struct.Padding.md).

#### `y`

```rust
pub fn y(self) -> f32
```

Returns the total amount of vertical [`Padding`](struct.Padding.md).

#### `fit`

```rust
pub fn fit(self, inner: Size, outer: Size) -> Self
```

Fits the [`Padding`](struct.Padding.md) between the provided `inner` and `outer` [`Size`](../struct.Size.md).

## Trait Implementations

### `impl Clone for Padding`

#### `clone`

```rust
fn clone(&self) -> Padding
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Padding`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Padding`

#### `default`

```rust
fn default() -> Padding
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl From<[f32; 2]> for Padding`

#### `from`

```rust
fn from(p: [f32; 2]) -> Self
```

Converts to this type from the input type.

### `impl From<[u16; 2]> for Padding`

#### `from`

```rust
fn from(p: [u16; 2]) -> Self
```

Converts to this type from the input type.

### `impl From<Padding> for Size`

#### `from`

```rust
fn from(padding: Padding) -> Self
```

Converts to this type from the input type.

### `impl From<Pixels> for Padding`

#### `from`

```rust
fn from(pixels: Pixels) -> Self
```

Converts to this type from the input type.

### `impl From<f32> for Padding`

#### `from`

```rust
fn from(p: f32) -> Self
```

Converts to this type from the input type.

### `impl From<u16> for Padding`

#### `from`

```rust
fn from(p: u16) -> Self
```

Converts to this type from the input type.

### `impl PartialEq for Padding`

#### `eq`

```rust
fn eq(&self, other: &Padding) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Padding`

### `impl StructuralPartialEq for Padding`

## Auto Trait Implementations

- `impl Freeze for Padding`
- `impl RefUnwindSafe for Padding`
- `impl Send for Padding`
- `impl Sync for Padding`
- `impl Unpin for Padding`
- `impl UnsafeUnpin for Padding`
- `impl UnwindSafe for Padding`
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
