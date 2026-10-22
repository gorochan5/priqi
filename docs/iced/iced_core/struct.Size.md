---
title: "Size"
kind: struct
path: "iced_core::Size"
crate: "iced_core"
module: "iced_core"
file: "iced_core/struct.Size.md"
docs_url: "https://docs.iced.rs/iced_core/struct.Size.html"
source_file: "src/iced_core/size.rs"
source_lines: "5-10"
---

# `iced_core::Size`

```rust
pub struct Size<T = f32> {
    pub width: T,
    pub height: T,
}
```

An amount of space in 2 dimensions.

**Source:** [size.rs](../src/iced_core/size.rs.md) (lines 5-10)

## Fields

### `width: T`

The width.

### `height: T`

The height.

## Methods

### `impl<T> Size<T>`

#### `new`

```rust
pub const fn new(width: T, height: T) -> Self
```

Creates a new [`Size`](struct.Size.md) with the given width and height.

### `impl Size`

#### `ZERO`

```rust
pub const ZERO: Size
```

A [`Size`](struct.Size.md) with zero width and height.

#### `UNIT`

```rust
pub const UNIT: Size
```

A [`Size`](struct.Size.md) with a width and height of 1 unit.

#### `INFINITE`

```rust
pub const INFINITE: Size
```

A [`Size`](struct.Size.md) with infinite width and height.

#### `min`

```rust
pub fn min(self, other: Self) -> Self
```

Returns the minimum of each component of this size and another.

#### `max`

```rust
pub fn max(self, other: Self) -> Self
```

Returns the maximum of each component of this size and another.

#### `expand`

```rust
pub fn expand(self, other: impl Into<Size>) -> Self
```

Expands this [`Size`](struct.Size.md) by the given amount.

#### `rotate`

```rust
pub fn rotate(self, rotation: Radians) -> Size
```

Rotates this [`Size`](struct.Size.md) and returns the minimum [`Size`](struct.Size.md) containing it.

#### `ratio`

```rust
pub const fn ratio(self, aspect_ratio: f32) -> Size
```

Applies an aspect ratio to this [`Size`](struct.Size.md) without exceeding its bounds.

#### `align`

```rust
pub fn align( self, container: Size, align_x: Alignment, align_y: Alignment, ) -> Vector
```

Returns the offset needed to align this [`Size`](struct.Size.md) within the given `container` using the provided alignments.

## Trait Implementations

### `impl<T> Add for Size<T> where T: Add<Output = T>,`

#### `Output`

```rust
type Output = Size<T>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, rhs: Self) -> Self::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T: Clone> Clone for Size<T>`

#### `clone`

```rust
fn clone(&self) -> Size<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T: Debug> Debug for Size<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T: Default> Default for Size<T>`

#### `default`

```rust
fn default() -> Size<T>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T> Div<T> for Size<T> where T: Div<Output = T> + Copy,`

#### `Output`

```rust
type Output = Size<T>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, rhs: T) -> Self::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T> From<[T; 2]> for Size<T>`

#### `from`

```rust
fn from([width, height]: [T; 2]) -> Self
```

Converts to this type from the input type.

### `impl<T> From<(T, T)> for Size<T>`

#### `from`

```rust
fn from((width, height): (T, T)) -> Self
```

Converts to this type from the input type.

### `impl From<(u32, u32)> for Size`

#### `from`

```rust
fn from((width, height): (u32, u32)) -> Self
```

Converts to this type from the input type.

### `impl From<Padding> for Size`

#### `from`

```rust
fn from(padding: Padding) -> Self
```

Converts to this type from the input type.

### `impl<T> From<Rectangle<T>> for Size<T>`

#### `from`

```rust
fn from(rectangle: Rectangle<T>) -> Size<T>
```

Converts to this type from the input type.

### `impl<T> From<Size<T>> for [T; 2]`

#### `from`

```rust
fn from(size: Size<T>) -> Self
```

Converts to this type from the input type.

### `impl<T> From<Size<T>> for Vector<T>`

#### `from`

```rust
fn from(size: Size<T>) -> Self
```

Converts to this type from the input type.

### `impl<T> From<Vector<T>> for Size<T>`

#### `from`

```rust
fn from(vector: Vector<T>) -> Self
```

Converts to this type from the input type.

### `impl<T: Hash> Hash for Size<T>`

#### `hash`

```rust
fn hash<__H: Hasher>(&self, state: &mut __H)
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl<T> Mul<T> for Size<T> where T: Mul<Output = T> + Copy,`

#### `Output`

```rust
type Output = Size<T>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, rhs: T) -> Self::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl Mul<Transformation> for Size`

#### `Output`

```rust
type Output = Size
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, transformation: Transformation) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T> Mul<Vector<T>> for Size<T> where T: Mul<Output = T> + Copy,`

#### `Output`

```rust
type Output = Size<T>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: Vector<T>) -> Self::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T: PartialEq> PartialEq for Size<T>`

#### `eq`

```rust
fn eq(&self, other: &Size<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T> Sub for Size<T> where T: Sub<Output = T>,`

#### `Output`

```rust
type Output = Size<T>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, rhs: Self) -> Self::Output
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T: Copy> Copy for Size<T>`

### `impl<T: Eq> Eq for Size<T>`

### `impl<T> StructuralPartialEq for Size<T>`

## Auto Trait Implementations

- `impl<T> Freeze for Size<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for Size<T> where T: RefUnwindSafe,`
- `impl<T> Send for Size<T> where T: Send,`
- `impl<T> Sync for Size<T> where T: Sync,`
- `impl<T> Unpin for Size<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Size<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for Size<T> where T: UnwindSafe,`
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
