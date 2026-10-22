---
title: "Gradient"
kind: enum
path: "iced_core::gradient::Gradient"
crate: "iced_core"
module: "iced_core::gradient"
file: "iced_core/gradient/enum.Gradient.md"
docs_url: "https://docs.iced.rs/iced_core/gradient/enum.Gradient.html"
source_file: "src/iced_core/gradient.rs"
source_lines: "9-12"
---

# `iced_core::gradient::Gradient`

```rust
pub enum Gradient {
    Linear(Linear),
}
```

A fill which transitions colors progressively along a direction, either linearly, radially (TBD), or conically (TBD).

**Source:** [gradient.rs](../../src/iced_core/gradient.rs.md) (lines 9-12)

## Variants

### `Linear(Linear)`

A linear gradient interpolates colors along a direction at a specific angle.

## Methods

### `impl Gradient`

#### `scale_alpha`

```rust
pub fn scale_alpha(self, factor: f32) -> Self
```

Scales the alpha channel of the [`Gradient`](enum.Gradient.md) by the given factor.

## Trait Implementations

### `impl Clone for Gradient`

#### `clone`

```rust
fn clone(&self) -> Gradient
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Gradient`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<Gradient> for Background`

#### `from`

```rust
fn from(gradient: Gradient) -> Self
```

Converts to this type from the input type.

### `impl From<Linear> for Gradient`

#### `from`

```rust
fn from(gradient: Linear) -> Self
```

Converts to this type from the input type.

### `impl PartialEq for Gradient`

#### `eq`

```rust
fn eq(&self, other: &Gradient) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Gradient`

### `impl StructuralPartialEq for Gradient`

## Auto Trait Implementations

- `impl Freeze for Gradient`
- `impl RefUnwindSafe for Gradient`
- `impl Send for Gradient`
- `impl Sync for Gradient`
- `impl Unpin for Gradient`
- `impl UnsafeUnpin for Gradient`
- `impl UnwindSafe for Gradient`
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
