---
title: "Settings"
kind: struct
path: "iced_core::backend::Settings"
crate: "iced_core"
module: "iced_core::backend"
file: "iced_core/backend/struct.Settings.md"
docs_url: "https://docs.iced.rs/iced_core/backend/struct.Settings.html"
source_file: "src/iced_core/backend.rs"
source_lines: "141-165"
---

# `iced_core::backend::Settings`

```rust
pub struct Settings {
    pub backend: Backend,
    pub power_preference: PowerPreference,
    pub antialiasing: bool,
    pub vsync: bool,
}
```

The settings usted to configure a [`Backend`](enum.Backend.md).

**Source:** [backend.rs](../../src/iced_core/backend.rs.md) (lines 141-165)

## Fields

### `backend: Backend`

The graphical backend to use.

By default, it is [`Backend::Best`](enum.Backend.md).

### `power_preference: PowerPreference`

The [`PowerPreference`](enum.PowerPreference.md) of the backend.

By default, it is [`PowerPreference::None`](enum.PowerPreference.md).

### `antialiasing: bool`

If set to true, the renderer will try to perform antialiasing for some primitives.

Enabling it can produce a smoother result in some widgets, like the `Canvas`, at a performance cost.

By default, it is `true`.

### `vsync: bool`

Whether or not to synchronize frames.

By default, it is `true`.

## Trait Implementations

### `impl Clone for Settings`

#### `clone`

```rust
fn clone(&self) -> Settings
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Settings`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Settings`

#### `default`

```rust
fn default() -> Settings
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl From<&Settings> for Settings`

#### `from`

```rust
fn from(settings: &Settings) -> Self
```

Converts to this type from the input type.

### `impl PartialEq for Settings`

#### `eq`

```rust
fn eq(&self, other: &Settings) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl StructuralPartialEq for Settings`

## Auto Trait Implementations

- `impl Freeze for Settings`
- `impl RefUnwindSafe for Settings`
- `impl Send for Settings`
- `impl Sync for Settings`
- `impl Unpin for Settings`
- `impl UnsafeUnpin for Settings`
- `impl UnwindSafe for Settings`
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
