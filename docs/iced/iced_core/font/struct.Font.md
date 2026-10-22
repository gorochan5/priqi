---
title: "Font"
kind: struct
path: "iced_core::font::Font"
crate: "iced_core"
module: "iced_core::font"
file: "iced_core/font/struct.Font.md"
docs_url: "https://docs.iced.rs/iced_core/font/struct.Font.html"
source_file: "src/iced_core/font.rs"
source_lines: "6-15"
---

# `iced_core::font::Font`

```rust
pub struct Font {
    pub family: Family,
    pub weight: Weight,
    pub stretch: Stretch,
    pub style: Style,
}
```

A font.

**Source:** [font.rs](../../src/iced_core/font.rs.md) (lines 6-15)

## Fields

### `family: Family`

The [`Family`](enum.Family.md) of the [`Font`](struct.Font.md).

### `weight: Weight`

The [`Weight`](enum.Weight.md) of the [`Font`](struct.Font.md).

### `stretch: Stretch`

The [`Stretch`](enum.Stretch.md) of the [`Font`](struct.Font.md).

### `style: Style`

The [`Style`](enum.Style.md) of the [`Font`](struct.Font.md).

## Methods

### `impl Font`

#### `DEFAULT`

```rust
pub const DEFAULT: Font
```

A non-monospaced sans-serif font with normal [`Weight`](enum.Weight.md).

#### `MONOSPACE`

```rust
pub const MONOSPACE: Font
```

A monospaced font with normal [`Weight`](enum.Weight.md).

#### `new`

```rust
pub const fn new(name: &'static str) -> Self
```

Creates a [`Font`](struct.Font.md) with the given [`Family::Name`](enum.Family.md) and default attributes.

#### `with_family`

```rust
pub fn with_family(family: impl Into<Family>) -> Self
```

Creates a [`Font`](struct.Font.md) with the given [`Family`](enum.Family.md) and default attributes.

#### `weight`

```rust
pub const fn weight(self, weight: Weight) -> Self
```

Sets the [`Weight`](enum.Weight.md) of the [`Font`](struct.Font.md).

#### `stretch`

```rust
pub const fn stretch(self, stretch: Stretch) -> Self
```

Sets the [`Stretch`](enum.Stretch.md) of the [`Font`](struct.Font.md).

#### `style`

```rust
pub const fn style(self, style: Style) -> Self
```

Sets the [`Style`](enum.Style.md) of the [`Font`](struct.Font.md).

## Trait Implementations

### `impl Clone for Font`

#### `clone`

```rust
fn clone(&self) -> Font
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Font`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Font`

#### `default`

```rust
fn default() -> Font
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl From<&'static str> for Font`

#### `from`

```rust
fn from(name: &'static str) -> Self
```

Converts to this type from the input type.

### `impl From<Family> for Font`

#### `from`

```rust
fn from(family: Family) -> Self
```

Converts to this type from the input type.

### `impl Hash for Font`

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

### `impl PartialEq for Font`

#### `eq`

```rust
fn eq(&self, other: &Font) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Font`

### `impl Eq for Font`

### `impl StructuralPartialEq for Font`

## Auto Trait Implementations

- `impl Freeze for Font`
- `impl RefUnwindSafe for Font`
- `impl Send for Font`
- `impl Sync for Font`
- `impl Unpin for Font`
- `impl UnsafeUnpin for Font`
- `impl UnwindSafe for Font`
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
