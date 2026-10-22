---
title: "Rectangle"
kind: struct
path: "iced_core::Rectangle"
crate: "iced_core"
module: "iced_core"
file: "iced_core/struct.Rectangle.md"
docs_url: "https://docs.iced.rs/iced_core/struct.Rectangle.html"
source_file: "src/iced_core/rectangle.rs"
source_lines: "6-18"
---

# `iced_core::Rectangle`

```rust
pub struct Rectangle<T = f32> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}
```

An axis-aligned rectangle.

**Source:** [rectangle.rs](../src/iced_core/rectangle.rs.md) (lines 6-18)

## Fields

### `x: T`

X coordinate of the top-left corner.

### `y: T`

Y coordinate of the top-left corner.

### `width: T`

Width of the rectangle.

### `height: T`

Height of the rectangle.

## Methods

### `impl<T> Rectangle<T> where T: Default + Copy,`

#### `with_size`

```rust
pub fn with_size(size: Size<T>) -> Self
```

Creates a new [`Rectangle`](struct.Rectangle.md) with its top-left corner at the origin and with the provided [`Size`](struct.Size.md).

#### `size`

```rust
pub const fn size(&self) -> Size<T>
```

Returns the [`Size`](struct.Size.md) of the [`Rectangle`](struct.Rectangle.md).

### `impl Rectangle<f32>`

#### `INFINITE`

```rust
pub const INFINITE: Self
```

A rectangle starting at negative infinity and with infinite width and height.

#### `new`

```rust
pub const fn new(top_left: Point, size: Size) -> Self
```

Creates a new [`Rectangle`](struct.Rectangle.md) with its top-left corner in the given [`Point`](struct.Point.md) and with the provided [`Size`](struct.Size.md).

#### `with_radius`

```rust
pub fn with_radius(radius: f32) -> Self
```

Creates a new square [`Rectangle`](struct.Rectangle.md) with the center at the origin and with the given radius.

#### `with_vertices`

```rust
pub fn with_vertices( top_left: Point, top_right: Point, bottom_left: Point, ) -> (Rectangle, Radians)
```

Creates a new axis-aligned [`Rectangle`](struct.Rectangle.md) from the given vertices; returning the rotation in [`Radians`](struct.Radians.md) that must be applied to the axis-aligned [`Rectangle`](struct.Rectangle.md) to obtain the desired result.

#### `center`

```rust
pub fn center(&self) -> Point
```

Returns the [`Point`](struct.Point.md) at the center of the [`Rectangle`](struct.Rectangle.md).

#### `center_x`

```rust
pub fn center_x(&self) -> f32
```

Returns the X coordinate of the [`Point`](struct.Point.md) at the center of the [`Rectangle`](struct.Rectangle.md).

#### `center_y`

```rust
pub fn center_y(&self) -> f32
```

Returns the Y coordinate of the [`Point`](struct.Point.md) at the center of the [`Rectangle`](struct.Rectangle.md).

#### `position`

```rust
pub fn position(&self) -> Point
```

Returns the position of the top left corner of the [`Rectangle`](struct.Rectangle.md).

#### `area`

```rust
pub fn area(&self) -> f32
```

Returns the area of the [`Rectangle`](struct.Rectangle.md).

#### `contains`

```rust
pub fn contains(&self, point: Point) -> bool
```

Returns true if the given [`Point`](struct.Point.md) is contained in the [`Rectangle`](struct.Rectangle.md). Excludes the right and bottom edges.

#### `distance`

```rust
pub fn distance(&self, point: Point) -> f32
```

Returns the minimum distance from the given [`Point`](struct.Point.md) to any of the edges of the [`Rectangle`](struct.Rectangle.md).

#### `offset`

```rust
pub fn offset(&self, container: &Rectangle) -> Vector
```

Computes the offset that must be applied to the [`Rectangle`](struct.Rectangle.md) to be placed inside the given `container`.

#### `is_within`

```rust
pub fn is_within(&self, container: &Rectangle) -> bool
```

Returns true if the current [`Rectangle`](struct.Rectangle.md) is within the given `container`. Includes the right and bottom edges.

#### `intersection`

```rust
pub fn intersection(&self, other: &Rectangle<f32>) -> Option<Rectangle<f32>>
```

Computes the intersection with the given [`Rectangle`](struct.Rectangle.md).

#### `intersects`

```rust
pub fn intersects(&self, other: &Self) -> bool
```

Returns whether the [`Rectangle`](struct.Rectangle.md) intersects with the given one.

#### `union`

```rust
pub fn union(&self, other: &Self) -> Self
```

Computes the union with the given [`Rectangle`](struct.Rectangle.md).

#### `round`

```rust
pub fn round(self) -> Self
```

Rounds the [`Rectangle`](struct.Rectangle.md) coordinates.

#### `snap`

```rust
pub fn snap(self) -> Option<Rectangle<u32>>
```

Snaps the [`Rectangle`](struct.Rectangle.md) to **unsigned** integer coordinates.

#### `hint`

```rust
pub fn hint(self, scale_factor: f32) -> Self
```

Computes the logical [`Rectangle`](struct.Rectangle.md) that ends up rounded after the given `scale_factor` is applied to it.

#### `expand`

```rust
pub fn expand(self, padding: impl Into<Padding>) -> Self
```

Expands the [`Rectangle`](struct.Rectangle.md) a given amount.

#### `shrink`

```rust
pub fn shrink(self, padding: impl Into<Padding>) -> Self
```

Shrinks the [`Rectangle`](struct.Rectangle.md) a given amount.

#### `rotate`

```rust
pub fn rotate(self, rotation: Radians) -> Self
```

Rotates the [`Rectangle`](struct.Rectangle.md) and returns the smallest [`Rectangle`](struct.Rectangle.md) containing it.

#### `zoom`

```rust
pub fn zoom(self, zoom: f32) -> Self
```

Scales the [`Rectangle`](struct.Rectangle.md) without changing its position, effectively “zooming” it.

#### `anchor`

```rust
pub fn anchor( &self, size: Size, align_x: impl Into<Horizontal>, align_y: impl Into<Vertical>, ) -> Point
```

Returns the top-left position to render an object of the given [`Size`](struct.Size.md). inside the [`Rectangle`](struct.Rectangle.md) that is anchored to the edge or corner defined by the alignment arguments.

#### `chord`

```rust
pub fn chord(&self, angle: impl Into<Radians>) -> (Point, Point)
```

Returns the two endpoints of a chord through the rectangle’s center at the given `angle`.

The angle is measured clockwise from the negative y-axis, so `0` produces a vertical chord and `π/2` a horizontal one. The returned points are `(start, end)`, symmetric about [`Rectangle::center`](struct.Rectangle.md).

## Trait Implementations

### `impl<T> Add<Vector<T>> for Rectangle<T> where T: Add<Output = T>,`

#### `Output`

```rust
type Output = Rectangle<T>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, translation: Vector<T>) -> Self
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T> AddAssign<Vector<T>> for Rectangle<T> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, translation: Vector<T>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T: Clone> Clone for Rectangle<T>`

#### `clone`

```rust
fn clone(&self) -> Rectangle<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T: Debug> Debug for Rectangle<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T: Default> Default for Rectangle<T>`

#### `default`

```rust
fn default() -> Rectangle<T>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Div<f32> for Rectangle<f32>`

#### `Output`

```rust
type Output = Rectangle
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, scale: f32) -> Self
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T> From<Rectangle<T>> for Size<T>`

#### `from`

```rust
fn from(rectangle: Rectangle<T>) -> Size<T>
```

Converts to this type from the input type.

### `impl From<Rectangle<u32>> for Rectangle<f32>`

#### `from`

```rust
fn from(rectangle: Rectangle<u32>) -> Rectangle<f32>
```

Converts to this type from the input type.

### `impl Mul<Transformation> for Rectangle`

#### `Output`

```rust
type Output = Rectangle
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, transformation: Transformation) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl Mul<f32> for Rectangle<f32>`

#### `Output`

```rust
type Output = Rectangle
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: f32) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T: PartialEq> PartialEq for Rectangle<T>`

#### `eq`

```rust
fn eq(&self, other: &Rectangle<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T> Sub<Vector<T>> for Rectangle<T> where T: Sub<Output = T>,`

#### `Output`

```rust
type Output = Rectangle<T>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, translation: Vector<T>) -> Self
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T> SubAssign<Vector<T>> for Rectangle<T> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, translation: Vector<T>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<T: Copy> Copy for Rectangle<T>`

### `impl<T: Eq> Eq for Rectangle<T>`

### `impl<T> StructuralPartialEq for Rectangle<T>`

## Auto Trait Implementations

- `impl<T> Freeze for Rectangle<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for Rectangle<T> where T: RefUnwindSafe,`
- `impl<T> Send for Rectangle<T> where T: Send,`
- `impl<T> Sync for Rectangle<T> where T: Sync,`
- `impl<T> Unpin for Rectangle<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Rectangle<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for Rectangle<T> where T: UnwindSafe,`
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
