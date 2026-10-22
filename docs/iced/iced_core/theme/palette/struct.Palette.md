---
title: "Palette"
kind: struct
path: "iced_core::theme::palette::Palette"
crate: "iced_core"
module: "iced_core::theme::palette"
file: "iced_core/theme/palette/struct.Palette.md"
docs_url: "https://docs.iced.rs/iced_core/theme/palette/struct.Palette.html"
source_file: "src/iced_core/theme/palette.rs"
source_lines: "8-23"
---

# `iced_core::theme::palette::Palette`

```rust
pub struct Palette {
    pub background: Background,
    pub primary: Swatch,
    pub secondary: Swatch,
    pub success: Swatch,
    pub warning: Swatch,
    pub danger: Swatch,
    pub is_dark: bool,
}
```

An extended set of colors generated from a [`Seed`](struct.Seed.md).

**Source:** [palette.rs](../../../src/iced_core/theme/palette.rs.md) (lines 8-23)

## Fields

### `background: Background`

The set of background colors.

### `primary: Swatch`

The set of primary colors.

### `secondary: Swatch`

The set of secondary colors.

### `success: Swatch`

The set of success colors.

### `warning: Swatch`

The set of warning colors.

### `danger: Swatch`

The set of danger colors.

### `is_dark: bool`

Whether the palette is dark or not.

## Methods

### `impl Palette`

#### `generate`

```rust
pub fn generate(palette: Seed) -> Self
```

Generates a [`Palette`](struct.Palette.md) from the given [`Seed`](struct.Seed.md).

## Trait Implementations

### `impl Clone for Palette`

#### `clone`

```rust
fn clone(&self) -> Palette
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Palette`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl PartialEq for Palette`

#### `eq`

```rust
fn eq(&self, other: &Palette) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Palette`

### `impl StructuralPartialEq for Palette`

## Auto Trait Implementations

- `impl Freeze for Palette`
- `impl RefUnwindSafe for Palette`
- `impl Send for Palette`
- `impl Sync for Palette`
- `impl Unpin for Palette`
- `impl UnsafeUnpin for Palette`
- `impl UnwindSafe for Palette`
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
