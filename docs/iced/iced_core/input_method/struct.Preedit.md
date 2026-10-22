---
title: "Preedit"
kind: struct
path: "iced_core::input_method::Preedit"
crate: "iced_core"
module: "iced_core::input_method"
file: "iced_core/input_method/struct.Preedit.md"
docs_url: "https://docs.iced.rs/iced_core/input_method/struct.Preedit.html"
source_file: "src/iced_core/input_method.rs"
source_lines: "30-37"
---

# `iced_core::input_method::Preedit`

```rust
pub struct Preedit<T = String> {
    pub content: T,
    pub selection: Option<Range<usize>>,
    pub text_size: Option<Pixels>,
}
```

The pre-edit of an [`InputMethod`](enum.InputMethod.md).

**Source:** [input_method.rs](../../src/iced_core/input_method.rs.md) (lines 30-37)

## Fields

### `content: T`

The current content.

### `selection: Option<Range<usize>>`

The selected range of the content.

### `text_size: Option<Pixels>`

The text size of the content.

## Methods

### `impl<T> Preedit<T>`

#### `new`

```rust
pub fn new() -> Self where T: Default,
```

Creates a new empty [`Preedit`](struct.Preedit.md).

#### `to_owned`

```rust
pub fn to_owned(&self) -> Preedit where T: AsRef<str>,
```

Turns a [`Preedit`](struct.Preedit.md) into its owned version.

### `impl Preedit`

#### `as_ref`

```rust
pub fn as_ref(&self) -> Preedit<&str>
```

Borrows the contents of a [`Preedit`](struct.Preedit.md).

## Trait Implementations

### `impl<T: Clone> Clone for Preedit<T>`

#### `clone`

```rust
fn clone(&self) -> Preedit<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T: Debug> Debug for Preedit<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T: Default> Default for Preedit<T>`

#### `default`

```rust
fn default() -> Preedit<T>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T: PartialEq> PartialEq for Preedit<T>`

#### `eq`

```rust
fn eq(&self, other: &Preedit<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T> StructuralPartialEq for Preedit<T>`

## Auto Trait Implementations

- `impl<T> Freeze for Preedit<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for Preedit<T> where T: RefUnwindSafe,`
- `impl<T> Send for Preedit<T> where T: Send,`
- `impl<T> Sync for Preedit<T> where T: Sync,`
- `impl<T> Unpin for Preedit<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Preedit<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for Preedit<T> where T: UnwindSafe,`
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
