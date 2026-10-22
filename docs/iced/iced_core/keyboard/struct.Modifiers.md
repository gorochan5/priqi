---
title: "Modifiers"
kind: struct
path: "iced_core::keyboard::Modifiers"
crate: "iced_core"
module: "iced_core::keyboard"
file: "iced_core/keyboard/struct.Modifiers.md"
docs_url: "https://docs.iced.rs/iced_core/keyboard/struct.Modifiers.html"
source_file: "src/iced_core/keyboard/modifiers.rs"
source_lines: "3-30"
---

# `iced_core::keyboard::Modifiers`

```rust
pub struct Modifiers(/* private fields */);
```

The current state of the keyboard modifiers.

**Source:** [modifiers.rs](../../src/iced_core/keyboard/modifiers.rs.md) (lines 3-30)

## Methods

### `impl Modifiers`

#### `SHIFT`

```rust
pub const SHIFT: Self
```

The “shift” key.

#### `CTRL`

```rust
pub const CTRL: Self
```

The “control” key.

#### `ALT`

```rust
pub const ALT: Self
```

The “alt” key.

#### `LOGO`

```rust
pub const LOGO: Self
```

The “windows” key on Windows, “command” key on Mac, and “super” key on Linux.

#### `NONE`

```rust
pub const NONE: Self
```

No modifiers

### `impl Modifiers`

#### `empty`

```rust
pub const fn empty() -> Self
```

Get a flags value with all bits unset.

#### `all`

```rust
pub const fn all() -> Self
```

Get a flags value with all known bits set.

#### `bits`

```rust
pub const fn bits(&self) -> u32
```

Get the underlying bits value.

The returned value is exactly the bits set in this flags value.

#### `from_bits`

```rust
pub const fn from_bits(bits: u32) -> Option<Self>
```

Convert from a bits value.

This method will return `None` if any unknown bits are set.

#### `from_bits_truncate`

```rust
pub const fn from_bits_truncate(bits: u32) -> Self
```

Convert from a bits value, unsetting any unknown bits.

#### `from_bits_retain`

```rust
pub const fn from_bits_retain(bits: u32) -> Self
```

Convert from a bits value exactly.

#### `from_name`

```rust
pub fn from_name(name: &str) -> Option<Self>
```

Get a flags value with the bits of a flag with the given name set.

This method will return `None` if `name` is empty or doesn’t correspond to any named flag.

#### `is_empty`

```rust
pub const fn is_empty(&self) -> bool
```

Whether all bits in `self` are unset.

#### `is_all`

```rust
pub const fn is_all(&self) -> bool
```

Whether all known bits in this flags value are set.

#### `intersects`

```rust
pub const fn intersects(&self, other: Self) -> bool
```

Whether any set bits in `other` are also set in `self`.

#### `contains`

```rust
pub const fn contains(&self, other: Self) -> bool
```

Whether all set bits in `other` are also set in `self`.

#### `insert`

```rust
pub fn insert(&mut self, other: Self)
```

The bitwise or (`|`) of the bits in `self` and `other`.

#### `remove`

```rust
pub fn remove(&mut self, other: Self)
```

The intersection of `self` with the complement of `other` (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `remove` won’t truncate `other`, but the `!` operator will.

#### `toggle`

```rust
pub fn toggle(&mut self, other: Self)
```

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

#### `set`

```rust
pub fn set(&mut self, other: Self, value: bool)
```

Call `insert` when `value` is `true` or `remove` when `value` is `false`.

#### `intersection`

```rust
pub const fn intersection(self, other: Self) -> Self
```

The bitwise and (`&`) of the bits in `self` and `other`.

#### `union`

```rust
pub const fn union(self, other: Self) -> Self
```

The bitwise or (`|`) of the bits in `self` and `other`.

#### `difference`

```rust
pub const fn difference(self, other: Self) -> Self
```

The intersection of `self` with the complement of `other` (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `difference` won’t truncate `other`, but the `!` operator will.

#### `symmetric_difference`

```rust
pub const fn symmetric_difference(self, other: Self) -> Self
```

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

#### `complement`

```rust
pub const fn complement(self) -> Self
```

The bitwise negation (`!`) of the bits in `self`, truncating the result.

### `impl Modifiers`

#### `iter`

```rust
pub const fn iter(&self) -> Iter<Modifiers>
```

Yield a set of contained flags values.

Each yielded flags value will correspond to a defined named flag. Any unknown bits will be yielded together as a final flags value.

#### `iter_names`

```rust
pub const fn iter_names(&self) -> IterNames<Modifiers>
```

Yield a set of contained named flags values.

This method is like `iter`, except only yields bits in contained named flags. Any unknown bits, or bits not corresponding to a contained flag will not be yielded.

### `impl Modifiers`

#### `COMMAND`

```rust
pub const COMMAND: Self
```

The “command” key.

This is normally the main modifier to be used for hotkeys.

On macOS, this is equivalent to `Self::LOGO`. Otherwise, this is equivalent to `Self::CTRL`.

#### `shift`

```rust
pub fn shift(self) -> bool
```

Returns true if the [`SHIFT`](struct.Modifiers.md) key is pressed in the [`Modifiers`](struct.Modifiers.md).

#### `control`

```rust
pub fn control(self) -> bool
```

Returns true if the [`CTRL`](struct.Modifiers.md) key is pressed in the [`Modifiers`](struct.Modifiers.md).

#### `alt`

```rust
pub fn alt(self) -> bool
```

Returns true if the [`ALT`](struct.Modifiers.md) key is pressed in the [`Modifiers`](struct.Modifiers.md).

#### `logo`

```rust
pub fn logo(self) -> bool
```

Returns true if the [`LOGO`](struct.Modifiers.md) key is pressed in the [`Modifiers`](struct.Modifiers.md).

#### `command`

```rust
pub fn command(self) -> bool
```

Returns true if a “command key” is pressed in the [`Modifiers`](struct.Modifiers.md).

The “command key” is the main modifier key used to issue commands in the current platform. Specifically:

- It is the `logo` or command key (⌘) on macOS
- It is the `control` key on other platforms

#### `jump`

```rust
pub fn jump(self) -> bool
```

Returns true if the “jump key” is pressed in the [`Modifiers`](struct.Modifiers.md).

The “jump key” is the modifier key used to widen text motions. It is the `Alt` key in macOS and the `Ctrl` key in other platforms.

#### `macos_command`

```rust
pub fn macos_command(self) -> bool
```

Returns true if the “command key” is pressed on a macOS device.

This is relevant for macOS-specific actions (e.g. `⌘ + ArrowLeft` moves the cursor to the beginning of the line).

## Trait Implementations

### `impl Binary for Modifiers`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html#tymethod.fmt)

### `impl BitAnd for Modifiers`

#### `bitand`

```rust
fn bitand(self, other: Self) -> Self
```

The bitwise and (`&`) of the bits in `self` and `other`.

#### `Output`

```rust
type Output = Modifiers
```

The resulting type after applying the `&` operator.

### `impl BitAndAssign for Modifiers`

#### `bitand_assign`

```rust
fn bitand_assign(&mut self, other: Self)
```

The bitwise and (`&`) of the bits in `self` and `other`.

### `impl BitOr for Modifiers`

#### `bitor`

```rust
fn bitor(self, other: Modifiers) -> Self
```

The bitwise or (`|`) of the bits in `self` and `other`.

#### `Output`

```rust
type Output = Modifiers
```

The resulting type after applying the `|` operator.

### `impl BitOrAssign for Modifiers`

#### `bitor_assign`

```rust
fn bitor_assign(&mut self, other: Self)
```

The bitwise or (`|`) of the bits in `self` and `other`.

### `impl BitXor for Modifiers`

#### `bitxor`

```rust
fn bitxor(self, other: Self) -> Self
```

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

#### `Output`

```rust
type Output = Modifiers
```

The resulting type after applying the `^` operator.

### `impl BitXorAssign for Modifiers`

#### `bitxor_assign`

```rust
fn bitxor_assign(&mut self, other: Self)
```

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

### `impl Clone for Modifiers`

#### `clone`

```rust
fn clone(&self) -> Modifiers
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Modifiers`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Modifiers`

#### `default`

```rust
fn default() -> Modifiers
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Extend<Modifiers> for Modifiers`

#### `extend`

```rust
fn extend<T: IntoIterator<Item = Self>>(&mut self, iterator: T)
```

The bitwise or (`|`) of the bits in each flags value.

#### `extend_one`

```rust
fn extend_one(&mut self, item: A)
```

*🔬This is a nightly-only experimental API. (extend_one)*

Extends a collection with exactly one element.

#### `extend_reserve`

```rust
fn extend_reserve(&mut self, additional: usize)
```

*🔬This is a nightly-only experimental API. (extend_one)*

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

### `impl Flags for Modifiers`

#### `FLAGS`

```rust
const FLAGS: &'static [Flag<Modifiers>]
```

The set of defined flags.

#### `Bits`

```rust
type Bits = u32
```

The underlying bits type.

#### `bits`

```rust
fn bits(&self) -> u32
```

Get the underlying bits value. Read more

#### `from_bits_retain`

```rust
fn from_bits_retain(bits: u32) -> Modifiers
```

Convert from a bits value exactly.

#### `all_named`

```rust
fn all_named() -> Modifiers
```

Get a flags value with all bits from named flags set. Read more

#### `empty`

```rust
fn empty() -> Self
```

Get a flags value with all bits unset.

#### `all`

```rust
fn all() -> Self
```

Get a flags value with all known bits set.

#### `known_bits`

```rust
fn known_bits(&self) -> Self::Bits
```

Get the known bits from a flags value.

#### `unknown_bits`

```rust
fn unknown_bits(&self) -> Self::Bits
```

Get the unknown bits from a flags value.

#### `contains_unknown_bits`

```rust
fn contains_unknown_bits(&self) -> bool
```

This method will return `true` if any unknown bits are set.

#### `from_bits`

```rust
fn from_bits(bits: Self::Bits) -> Option<Self>
```

Convert from a bits value. Read more

#### `from_bits_truncate`

```rust
fn from_bits_truncate(bits: Self::Bits) -> Self
```

Convert from a bits value, unsetting any unknown bits.

#### `from_name`

```rust
fn from_name(name: &str) -> Option<Self>
```

Get a flags value with the bits of a flag with the given name set. Read more

#### `iter`

```rust
fn iter(&self) -> Iter<Self>
```

Yield a set of contained flags values. Read more

#### `iter_names`

```rust
fn iter_names(&self) -> IterNames<Self>
```

Yield a set of contained named flags values. Read more

#### `iter_defined_names`

```rust
fn iter_defined_names() -> IterDefinedNames<Self>
```

Yield a set of all named flags defined by [`Self::FLAGS`].

#### `iter_equal_names`

```rust
fn iter_equal_names(&self) -> IterEqualNames<Self>
```

Get an iterator over all defined names for this flags value. Read more

#### `is_empty`

```rust
fn is_empty(&self) -> bool
```

Whether all bits in this flags value are unset.

#### `is_all`

```rust
fn is_all(&self) -> bool
```

Whether all known bits in this flags value are set.

#### `intersects`

```rust
fn intersects(&self, other: Self) -> bool where Self: Sized,
```

Whether any set bits in `other` are also set in `self`.

#### `contains`

```rust
fn contains(&self, other: Self) -> bool where Self: Sized,
```

Whether all set bits in `other` are also set in `self`.

#### `truncate`

```rust
fn truncate(&mut self) where Self: Sized,
```

Remove any unknown bits from the flags.

#### `insert`

```rust
fn insert(&mut self, other: Self) where Self: Sized,
```

The bitwise or (`|`) of the bits in `self` and `other`.

#### `remove`

```rust
fn remove(&mut self, other: Self) where Self: Sized,
```

The intersection of `self` with the complement of `other` (`&!`). Read more

#### `toggle`

```rust
fn toggle(&mut self, other: Self) where Self: Sized,
```

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

#### `set`

```rust
fn set(&mut self, other: Self, value: bool) where Self: Sized,
```

Call [`Flags::insert`] when `value` is `true` or [`Flags::remove`] when `value` is `false`.

#### `clear`

```rust
fn clear(&mut self) where Self: Sized,
```

Unsets all bits in the flags.

#### `intersection`

```rust
fn intersection(self, other: Self) -> Self
```

The bitwise and (`&`) of the bits in `self` and `other`.

#### `union`

```rust
fn union(self, other: Self) -> Self
```

The bitwise or (`|`) of the bits in `self` and `other`.

#### `difference`

```rust
fn difference(self, other: Self) -> Self
```

The intersection of `self` with the complement of `other` (`&!`). Read more

#### `symmetric_difference`

```rust
fn symmetric_difference(self, other: Self) -> Self
```

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

#### `complement`

```rust
fn complement(self) -> Self
```

The bitwise negation (`!`) of the bits in `self`, truncating the result.

### `impl FromIterator<Modifiers> for Modifiers`

#### `from_iter`

```rust
fn from_iter<T: IntoIterator<Item = Self>>(iterator: T) -> Self
```

The bitwise or (`|`) of the bits in each flags value.

### `impl Hash for Modifiers`

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

### `impl IntoIterator for Modifiers`

#### `Item`

```rust
type Item = Modifiers
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = Iter<Modifiers>
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> Self::IntoIter
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

### `impl LowerHex for Modifiers`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html#tymethod.fmt)

### `impl Not for Modifiers`

#### `not`

```rust
fn not(self) -> Self
```

The bitwise negation (`!`) of the bits in `self`, truncating the result.

#### `Output`

```rust
type Output = Modifiers
```

The resulting type after applying the `!` operator.

### `impl Octal for Modifiers`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html#tymethod.fmt)

### `impl Ord for Modifiers`

#### `cmp`

```rust
fn cmp(&self, other: &Modifiers) -> Ordering
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

### `impl PartialEq for Modifiers`

#### `eq`

```rust
fn eq(&self, other: &Modifiers) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd for Modifiers`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Modifiers) -> Option<Ordering>
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

### `impl PublicFlags for Modifiers`

#### `Primitive`

```rust
type Primitive = u32
```

The type of the underlying storage.

#### `Internal`

```rust
type Internal = InternalBitFlags
```

The type of the internal field on the generated flags type.

### `impl Sub for Modifiers`

#### `sub`

```rust
fn sub(self, other: Self) -> Self
```

The intersection of `self` with the complement of `other` (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `difference` won’t truncate `other`, but the `!` operator will.

#### `Output`

```rust
type Output = Modifiers
```

The resulting type after applying the `-` operator.

### `impl SubAssign for Modifiers`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Self)
```

The intersection of `self` with the complement of `other` (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `difference` won’t truncate `other`, but the `!` operator will.

### `impl UpperHex for Modifiers`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html#tymethod.fmt)

### `impl Copy for Modifiers`

### `impl Eq for Modifiers`

### `impl StructuralPartialEq for Modifiers`

## Auto Trait Implementations

- `impl Freeze for Modifiers`
- `impl RefUnwindSafe for Modifiers`
- `impl Send for Modifiers`
- `impl Sync for Modifiers`
- `impl Unpin for Modifiers`
- `impl UnsafeUnpin for Modifiers`
- `impl UnwindSafe for Modifiers`
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
