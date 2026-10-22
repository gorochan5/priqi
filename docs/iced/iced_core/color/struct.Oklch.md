---
title: "Oklch"
kind: struct
path: "iced_core::color::Oklch"
crate: "iced_core"
module: "iced_core::color"
file: "iced_core/color/struct.Oklch.md"
docs_url: "https://docs.iced.rs/iced_core/color/struct.Oklch.html"
source_file: "src/iced_core/color.rs"
source_lines: "385-394"
---

# `iced_core::color::Oklch`

```rust
pub struct Oklch {
    pub l: f32,
    pub c: f32,
    pub h: f32,
    pub a: f32,
}
```

A color in the [Oklab color space](https://en.wikipedia.org/wiki/Oklab_color_space), represented as Oklch.

**Source:** [color.rs](../../src/iced_core/color.rs.md) (lines 385-394)

## Fields

### `l: f32`

Perceptual lightness: 0 is pure black, 1 is pure white.

### `c: f32`

Chromatic intensity: 0 is achromatic, +0.5 is usually the upper limit.

### `h: f32`

Hue angle, in radians.

### `a: f32`

Alpha channel.

## Trait Implementations

### `impl From<Color> for Oklch`

#### `from`

```rust
fn from(color: Color) -> Self
```

Converts to this type from the input type.

### `impl From<Oklch> for Color`

#### `from`

```rust
fn from(oklch: Oklch) -> Self
```

Converts to this type from the input type.

## Auto Trait Implementations

- `impl Freeze for Oklch`
- `impl RefUnwindSafe for Oklch`
- `impl Send for Oklch`
- `impl Sync for Oklch`
- `impl Unpin for Oklch`
- `impl UnsafeUnpin for Oklch`
- `impl UnwindSafe for Oklch`
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
