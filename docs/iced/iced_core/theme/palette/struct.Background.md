---
title: "Background"
kind: struct
path: "iced_core::theme::palette::Background"
crate: "iced_core"
module: "iced_core::theme::palette"
file: "iced_core/theme/palette/struct.Background.md"
docs_url: "https://docs.iced.rs/iced_core/theme/palette/struct.Background.html"
source_file: "src/iced_core/theme/palette.rs"
source_lines: "66-83"
---

# `iced_core::theme::palette::Background`

```rust
pub struct Background {
    pub base: Pair,
    pub weakest: Pair,
    pub weaker: Pair,
    pub weak: Pair,
    pub neutral: Pair,
    pub strong: Pair,
    pub stronger: Pair,
    pub strongest: Pair,
}
```

A set of background colors.

**Source:** [palette.rs](../../../src/iced_core/theme/palette.rs.md) (lines 66-83)

## Fields

### `base: Pair`

The base background color.

### `weakest: Pair`

The weakest version of the base background color.

### `weaker: Pair`

A weaker version of the base background color.

### `weak: Pair`

A weak version of the base background color.

### `neutral: Pair`

A neutral version of the base background color, between weak and strong.

### `strong: Pair`

A strong version of the base background color.

### `stronger: Pair`

A stronger version of the base background color.

### `strongest: Pair`

The strongest version of the base background color.

## Methods

### `impl Background`

#### `new`

```rust
pub fn new(base: Color, text: Color) -> Self
```

Generates a set of [`Background`](struct.Background.md) colors from the base and text colors.

## Trait Implementations

### `impl Clone for Background`

#### `clone`

```rust
fn clone(&self) -> Background
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Background`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl PartialEq for Background`

#### `eq`

```rust
fn eq(&self, other: &Background) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for Background`

### `impl StructuralPartialEq for Background`

## Auto Trait Implementations

- `impl Freeze for Background`
- `impl RefUnwindSafe for Background`
- `impl Send for Background`
- `impl Sync for Background`
- `impl Unpin for Background`
- `impl UnsafeUnpin for Background`
- `impl UnwindSafe for Background`
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
