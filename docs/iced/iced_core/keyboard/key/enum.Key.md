---
title: "Key"
kind: enum
path: "iced_core::keyboard::key::Key"
crate: "iced_core"
module: "iced_core::keyboard::key"
file: "iced_core/keyboard/key/enum.Key.md"
docs_url: "https://docs.iced.rs/iced_core/keyboard/key/enum.Key.html"
source_file: "src/iced_core/keyboard/key.rs"
source_lines: "10-21"
---

# `iced_core::keyboard::key::Key`

```rust
pub enum Key<C = SmolStr> {
    Named(Named),
    Character(C),
    Unidentified,
}
```

A key on the keyboard.

This is mostly the `Key` type found in [`winit`](https://docs.rs/winit/0.30/winit/keyboard/enum.Key.html).

**Source:** [key.rs](../../../src/iced_core/keyboard/key.rs.md) (lines 10-21)

## Variants

### `Named(Named)`

A key with an established name.

### `Character(C)`

A key string that corresponds to the character typed by the user, taking into account the user’s current locale setting, and any system-level keyboard mapping overrides that are in effect.

### `Unidentified`

An unidentified key.

## Methods

### `impl Key`

#### `as_ref`

```rust
pub fn as_ref(&self) -> Key<&str>
```

Convert `Key::Character(SmolStr)` to `Key::Character(&str)` so you can more easily match on `Key`. All other variants remain unchanged.

#### `to_latin`

```rust
pub fn to_latin(&self, physical_key: Physical) -> Option<char>
```

Tries to convert this logical [`Key`](enum.Key.md) into its latin character, using the [`Physical`](enum.Physical.md) key provided for translation if it isn’t already in latin.

Returns `None` if no latin variant could be found.

```rust
use iced_core::keyboard::key::{Key, Named, Physical, Code};

// Latin c
assert_eq!(
    Key::Character("c".into()).to_latin(Physical::Code(Code::KeyC)),
    Some('c'),
);

// Cyrillic с
assert_eq!(
    Key::Character("с".into()).to_latin(Physical::Code(Code::KeyC)),
    Some('c'),
);

// Arrow Left
assert_eq!(
    Key::Named(Named::ArrowLeft).to_latin(Physical::Code(Code::ArrowLeft)),
    None,
);
```

## Trait Implementations

### `impl<C: Clone> Clone for Key<C>`

#### `clone`

```rust
fn clone(&self) -> Key<C>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<C: Debug> Debug for Key<C>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<Named> for Key`

#### `from`

```rust
fn from(named: Named) -> Self
```

Converts to this type from the input type.

### `impl<C: Hash> Hash for Key<C>`

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

### `impl<C: Ord> Ord for Key<C>`

#### `cmp`

```rust
fn cmp(&self, other: &Key<C>) -> Ordering
```

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

#### `max`

```rust
fn max(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

#### `min`

```rust
fn min(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

#### `clamp`

```rust
fn clamp(self, min: Self, max: Self) -> Self where Self: Sized,
```

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

### `impl<C: PartialEq> PartialEq for Key<C>`

#### `eq`

```rust
fn eq(&self, other: &Key<C>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<C: PartialOrd> PartialOrd for Key<C>`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Key<C>) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, other: &Rhs) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, other: &Rhs) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `gt`

```rust
fn gt(&self, other: &Rhs) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

#### `ge`

```rust
fn ge(&self, other: &Rhs) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

### `impl<C: Eq> Eq for Key<C>`

### `impl<C> StructuralPartialEq for Key<C>`

## Auto Trait Implementations

- `impl<C> Freeze for Key<C> where C: Freeze,`
- `impl<C> RefUnwindSafe for Key<C> where C: RefUnwindSafe,`
- `impl<C> Send for Key<C> where C: Send,`
- `impl<C> Sync for Key<C> where C: Sync,`
- `impl<C> Unpin for Key<C> where C: Unpin,`
- `impl<C> UnsafeUnpin for Key<C> where C: UnsafeUnpin,`
- `impl<C> UnwindSafe for Key<C> where C: UnwindSafe,`
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
