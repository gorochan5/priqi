---
title: "Family"
kind: enum
path: "iced_core::font::Family"
crate: "iced_core"
module: "iced_core::font"
file: "iced_core/font/enum.Family.md"
docs_url: "https://docs.iced.rs/iced_core/font/enum.Family.html"
source_file: "src/iced_core/font.rs"
source_lines: "78-103"
---

# `iced_core::font::Family`

```rust
pub enum Family {
    Name(&'static str),
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}
```

A font family.

**Source:** [font.rs](../../src/iced_core/font.rs.md) (lines 78-103)

## Variants

### `Name(&'static str)`

The name of a font family of choice.

### `Serif`

Serif fonts represent the formal text style for a script.

### `SansSerif`

Glyphs in sans-serif fonts, as the term is used in CSS, are generally low contrast and have stroke endings that are plain — without any flaring, cross stroke, or other ornamentation.

### `Cursive`

Glyphs in cursive fonts generally use a more informal script style, and the result looks more like handwritten pen or brush writing than printed letterwork.

### `Fantasy`

Fantasy fonts are primarily decorative or expressive fonts that contain decorative or expressive representations of characters.

### `Monospace`

The sole criterion of a monospace font is that all glyphs have the same fixed width.

## Methods

### `impl Family`

#### `VARIANTS`

```rust
pub const VARIANTS: &[Self]
```

A list of all the different standalone family variants.

#### `name`

```rust
pub fn name(name: &str) -> Self
```

Creates a [`Family::Name`](enum.Family.md) from the given string.

The name is interned in a global cache and never freed.

## Trait Implementations

### `impl Clone for Family`

#### `clone`

```rust
fn clone(&self) -> Family
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Family`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Family`

#### `default`

```rust
fn default() -> Family
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Display for Family`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl From<&str> for Family`

#### `from`

```rust
fn from(name: &str) -> Self
```

Converts to this type from the input type.

### `impl From<Family> for Font`

#### `from`

```rust
fn from(family: Family) -> Self
```

Converts to this type from the input type.

### `impl Hash for Family`

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

### `impl PartialEq for Family`

#### `eq`

```rust
fn eq(&self, other: &Family) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Family`

### `impl Eq for Family`

### `impl StructuralPartialEq for Family`

## Auto Trait Implementations

- `impl Freeze for Family`
- `impl RefUnwindSafe for Family`
- `impl Send for Family`
- `impl Sync for Family`
- `impl Unpin for Family`
- `impl UnsafeUnpin for Family`
- `impl UnwindSafe for Family`
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
