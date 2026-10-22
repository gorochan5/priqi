---
title: "Point"
kind: struct
path: "iced_core::Point"
crate: "iced_core"
module: "iced_core"
file: "iced_core/struct.Point.md"
docs_url: "https://docs.iced.rs/iced_core/struct.Point.html"
source_file: "src/iced_core/point.rs"
source_lines: "8-14"
---

# `iced_core::Point`

```rust
pub struct Point<T = f32> {
    pub x: T,
    pub y: T,
}
```

A 2D point.

**Source:** [point.rs](../src/iced_core/point.rs.md) (lines 8-14)

## Fields

### `x: T`

The X coordinate.

### `y: T`

The Y coordinate.

## Methods

### `impl Point`

#### `ORIGIN`

```rust
pub const ORIGIN: Self
```

The origin (i.e. a [`Point`](struct.Point.md) at (0, 0)).

### `impl<T> Point<T>`

#### `new`

```rust
pub const fn new(x: T, y: T) -> Self
```

Creates a new [`Point`](struct.Point.md) with the given coordinates.

#### `distance`

```rust
pub fn distance(&self, to: Self) -> T where T: Float,
```

Computes the distance to another [`Point`](struct.Point.md).

### `impl Point<f32>`

#### `round`

```rust
pub fn round(self) -> Self
```

Rounds the [`Point`](struct.Point.md) coordinates.

#### `snap`

```rust
pub fn snap(self) -> Point<u32>
```

Snaps the [`Point`](struct.Point.md) to **unsigned** integer coordinates.

## Trait Implementations

### `impl<T> Add<Vector<T>> for Point<T> where T: Add<Output = T>,`

#### `Output`

```rust
type Output = Point<T>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, vector: Vector<T>) -> Self
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T> AddAssign<Vector<T>> for Point<T> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, vector: Vector<T>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T: Clone> Clone for Point<T>`

#### `clone`

```rust
fn clone(&self) -> Point<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T: Debug> Debug for Point<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T: Default> Default for Point<T>`

#### `default`

```rust
fn default() -> Point<T>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T> Display for Point<T> where T: Display,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl<T> From<[T; 2]> for Point<T> where T: Num,`

#### `from`

```rust
fn from([x, y]: [T; 2]) -> Self
```

Converts to this type from the input type.

### `impl<T> From<(T, T)> for Point<T> where T: Num,`

#### `from`

```rust
fn from((x, y): (T, T)) -> Self
```

Converts to this type from the input type.

### `impl<T> From<Point<T>> for [T; 2]`

#### `from`

```rust
fn from(point: Point<T>) -> [T; 2]
```

Converts to this type from the input type.

### `impl Mul<Transformation> for Point`

#### `Output`

```rust
type Output = Point
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, transformation: Transformation) -> Self
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T: PartialEq> PartialEq for Point<T>`

#### `eq`

```rust
fn eq(&self, other: &Point<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T> Sub<Vector<T>> for Point<T> where T: Sub<Output = T>,`

#### `Output`

```rust
type Output = Point<T>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, vector: Vector<T>) -> Self
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T> Sub for Point<T> where T: Sub<Output = T>,`

#### `Output`

```rust
type Output = Vector<T>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, point: Self) -> Vector<T>
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T> SubAssign<Vector<T>> for Point<T> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, vector: Vector<T>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<T: Copy> Copy for Point<T>`

### `impl<T: Eq> Eq for Point<T>`

### `impl<T> StructuralPartialEq for Point<T>`

## Auto Trait Implementations

- `impl<T> Freeze for Point<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for Point<T> where T: RefUnwindSafe,`
- `impl<T> Send for Point<T> where T: Send,`
- `impl<T> Sync for Point<T> where T: Sync,`
- `impl<T> Unpin for Point<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Point<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for Point<T> where T: UnwindSafe,`
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

### `impl<T> ToSmolStr for T where T: Display + ?Sized,`

#### `to_smolstr`

```rust
fn to_smolstr(&self) -> SmolStr
```

### `impl<T> ToString for T where T: Display + ?Sized,`

#### `to_string`

```rust
fn to_string(&self) -> String
```

Converts the given value to a `String`. [Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)

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
