---
title: "Length"
kind: enum
path: "iced_core::length::Length"
crate: "iced_core"
module: "iced_core::length"
file: "iced_core/length/enum.Length.md"
docs_url: "https://docs.iced.rs/iced_core/length/enum.Length.html"
source_file: "src/iced_core/length.rs"
source_lines: "6-42"
---

# `iced_core::length::Length`

```rust
pub enum Length {
    Fit,
    Fill,
    FillPortion(u16),
    Shrink,
    Fixed(f32),
    Bounded {
        bounds: Bounds,
        sizing: Sizing,
    },
    Fluid(Constraint),
}
```

The strategy used to fill space in a specific dimension.

**Source:** [length.rs](../../src/iced_core/length.rs.md) (lines 6-42)

## Variants

### `Fit`

Fill the minimum amount of space based on the intrinsic size of the element; normally defined by its contents.

This is the default sizing strategy of most widgets.

### `Fill`

Fill all the remaining space.

### `FillPortion(u16)`

Fill a portion of the remaining space relative to other elements.

Let’s say we have two elements: one with `FillPortion(2)` and one with `FillPortion(3)`. The first will get 2 portions of the available space, while the second one would get 3.

`Length::Fill` is equivalent to `Length::FillPortion(1)`.

### `Shrink`

Fill the least amount of space; compressing contents if possible.

### `Fixed(f32)`

Fill a fixed amount of space in pixels.

### `Bounded`

Fill a certain amount of space inside the given [`Bounds`](enum.Bounds.md) with some [`Sizing`](enum.Sizing.md) strategy.

**Fields**

- `bounds: Bounds` — The [`Bounds`](enum.Bounds.md) of space that can be filled.
- `sizing: Sizing` — The [`Sizing`](enum.Sizing.md) strategy with which the [`Bounds`](enum.Bounds.md) should be filled.

### `Fluid(Constraint)`

Fill the remaining space like [`Fill`](enum.Length.md), but subject to a single open-ended [`Constraint`](enum.Constraint.md).

## Methods

### `impl Length`

#### `min`

```rust
pub fn min(self, min: impl Into<Pixels>) -> Self
```

Returns a new [`Bounded`](enum.Length.md) length with the given minimum bounds.

#### `max`

```rust
pub fn max(self, max: impl Into<Pixels>) -> Self
```

Returns a new [`Bounded`](enum.Length.md) length with the given maximum bounds.

#### `fill_factor`

```rust
pub fn fill_factor(&self) -> u16
```

Returns the *fill factor* of the [`Length`](enum.Length.md).

The *fill factor* is a relative unit describing how much of the remaining space should be filled when compared to other elements. It is only meant to be used by layout engines.

#### `is_fill`

```rust
pub fn is_fill(&self) -> bool
```

Returns `true` if the [`Length`](enum.Length.md) is either [`Length::Fill`](enum.Length.md) or [`Length::FillPortion`](enum.Length.md).

#### `is_fit`

```rust
pub fn is_fit(&self) -> bool
```

Returns `true` if the [`Length`](enum.Length.md) is [`Fit`](enum.Length.md).

#### `stack`

```rust
pub fn stack(self, other: Length) -> Self
```

Stacks the constraints of the current [`Length`](enum.Length.md) with the given one, if applicable.

Specifically, minimum constraints will be *added* together and accumulated.

You should use this when a container lays out multiple elements along a given axis and need to inherit their constraints in the *main* axis.

#### `cross`

```rust
pub fn cross(self, other: Length) -> Self
```

Crosses the constraints of the current [`Length`](enum.Length.md) with the given one, if applicable.

Specifically, minimum constraints will be compared and the *maximum* will be returned.

You should use this when a container lays out multiple elements along a given axis and need to inherit their constraints in the *cross* axis.

## Trait Implementations

### `impl Clone for Length`

#### `clone`

```rust
fn clone(&self) -> Length
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Length`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<Pixels> for Length`

#### `from`

```rust
fn from(amount: Pixels) -> Self
```

Converts to this type from the input type.

### `impl From<f32> for Length`

#### `from`

```rust
fn from(amount: f32) -> Self
```

Converts to this type from the input type.

### `impl From<u32> for Length`

#### `from`

```rust
fn from(units: u32) -> Self
```

Converts to this type from the input type.

### `impl PartialEq for Length`

#### `eq`

```rust
fn eq(&self, other: &Length) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Length`

### `impl StructuralPartialEq for Length`

## Auto Trait Implementations

- `impl Freeze for Length`
- `impl RefUnwindSafe for Length`
- `impl Send for Length`
- `impl Sync for Length`
- `impl Unpin for Length`
- `impl UnsafeUnpin for Length`
- `impl UnwindSafe for Length`
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
