---
title: "SmolStr"
kind: struct
path: "iced_core::SmolStr"
crate: "iced_core"
module: "iced_core"
file: "iced_core/struct.SmolStr.md"
docs_url: "https://docs.iced.rs/iced_core/struct.SmolStr.html"
---

# `iced_core::SmolStr`

```rust
pub struct SmolStr(/* private fields */);
```

A `SmolStr` is a string type that has the following properties:

- `size_of::<SmolStr>() == 24` (therefore `== size_of::<String>()` on 64 bit platforms)
- `Clone` is `O(1)` (no allocation)
- Strings are stack-allocated if they are:

  - Up to 23 bytes long
  - Longer than 23 bytes, but substrings of `WS` (see below). Such strings consist solely of consecutive newlines, followed by consecutive spaces
- If a string does not satisfy the aforementioned conditions, it is heap-allocated
- Additionally, a `SmolStr` can be explicitly created from a `&'static str` without allocation

Unlike `String`, however, `SmolStr` is immutable. The primary use case for `SmolStr` is a good enough default storage for tokens of typical programming languages. Strings consisting of a series of newlines, followed by a series of whitespace are a typical pattern in computer programs because of indentation. Note that a specialized interner might be a better solution for some use cases.

`WS`: A string of 32 newlines followed by 128 spaces.

## Methods

### `impl SmolStr`

#### `INLINE_CAP`

```rust
pub const INLINE_CAP: usize = INLINE_CAP
```

The maximum byte length of a string that can be stored inline without heap allocation.

#### `new_inline`

```rust
pub const fn new_inline(text: &str) -> SmolStr
```

Constructs an inline variant of `SmolStr`.

This never allocates.

###### Panics

Panics if `text.len() >`[`SmolStr::INLINE_CAP`](struct.SmolStr.md).

#### `new_static`

```rust
pub const fn new_static(text: &'static str) -> SmolStr
```

Constructs a `SmolStr` from a statically allocated string.

This never allocates.

#### `new`

```rust
pub fn new(text: impl AsRef<str>) -> SmolStr
```

Constructs a `SmolStr` from a `str`, heap-allocating if necessary.

#### `as_str`

```rust
pub fn as_str(&self) -> &str
```

Returns a `&str` slice of this `SmolStr`.

#### `len`

```rust
pub fn len(&self) -> usize
```

Returns the length of `self` in bytes.

#### `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

Returns `true` if `self` has a length of zero bytes.

#### `is_heap_allocated`

```rust
pub const fn is_heap_allocated(&self) -> bool
```

Returns `true` if `self` is heap-allocated.

#### `from_utf8`

```rust
pub fn from_utf8(bytes: &[u8]) -> Result<SmolStr, Utf8Error>
```

Constructs a `SmolStr` from a byte slice, returning an error if the slice is not valid UTF-8.

#### `from_utf8_unchecked`

```rust
pub unsafe fn from_utf8_unchecked(bytes: &[u8]) -> SmolStr
```

Constructs a `SmolStr` from a byte slice without checking that the bytes are valid UTF-8.

###### Safety

`bytes` must be valid UTF-8.

## Trait Implementations

### `impl AsRef<[u8]> for SmolStr`

#### `as_ref`

```rust
fn as_ref(&self) -> &[u8] ⓘ
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl AsRef<OsStr> for SmolStr`

#### `as_ref`

```rust
fn as_ref(&self) -> &OsStr
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl AsRef<Path> for SmolStr`

#### `as_ref`

```rust
fn as_ref(&self) -> &Path
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl AsRef<str> for SmolStr`

#### `as_ref`

```rust
fn as_ref(&self) -> &str
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl Borrow<str> for SmolStr`

#### `borrow`

```rust
fn borrow(&self) -> &str
```

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

### `impl Clone for SmolStr`

#### `clone`

```rust
fn clone(&self) -> SmolStr
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for SmolStr`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for SmolStr`

#### `default`

```rust
fn default() -> SmolStr
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Deref for SmolStr`

#### `Target`

```rust
type Target = str
```

The resulting type after dereferencing.

#### `deref`

```rust
fn deref(&self) -> &str
```

Dereferences the value.

### `impl Display for SmolStr`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl From<&String> for SmolStr`

#### `from`

```rust
fn from(s: &String) -> SmolStr
```

Converts to this type from the input type.

### `impl From<&mut str> for SmolStr`

#### `from`

```rust
fn from(s: &mut str) -> SmolStr
```

Converts to this type from the input type.

### `impl From<&str> for SmolStr`

#### `from`

```rust
fn from(s: &str) -> SmolStr
```

Converts to this type from the input type.

### `impl From<Arc<str>> for SmolStr`

#### `from`

```rust
fn from(s: Arc<str>) -> SmolStr
```

Converts to this type from the input type.

### `impl From<Box<str>> for SmolStr`

#### `from`

```rust
fn from(s: Box<str>) -> SmolStr
```

Converts to this type from the input type.

### `impl<'a> From<Cow<'a, str>> for SmolStr`

#### `from`

```rust
fn from(s: Cow<'a, str>) -> SmolStr
```

Converts to this type from the input type.

### `impl From<SmolStrBuilder> for SmolStr`

#### `from`

```rust
fn from(value: SmolStrBuilder) -> SmolStr
```

Converts to this type from the input type.

### `impl From<String> for SmolStr`

#### `from`

```rust
fn from(text: String) -> SmolStr
```

Converts to this type from the input type.

### `impl From<char> for SmolStr`

#### `from`

```rust
fn from(c: char) -> SmolStr
```

Converts to this type from the input type.

### `impl<'a> FromIterator<&'a String> for SmolStr`

#### `from_iter`

```rust
fn from_iter<I>(iter: I) -> SmolStr where I: IntoIterator<Item = &'a String>,
```

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

### `impl<'a> FromIterator<&'a str> for SmolStr`

#### `from_iter`

```rust
fn from_iter<I>(iter: I) -> SmolStr where I: IntoIterator<Item = &'a str>,
```

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

### `impl FromIterator<String> for SmolStr`

#### `from_iter`

```rust
fn from_iter<I>(iter: I) -> SmolStr where I: IntoIterator<Item = String>,
```

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

### `impl FromIterator<char> for SmolStr`

#### `from_iter`

```rust
fn from_iter<I>(iter: I) -> SmolStr where I: IntoIterator<Item = char>,
```

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

### `impl FromStr for SmolStr`

#### `Err`

```rust
type Err = Infallible
```

The associated error which can be returned from parsing.

#### `from_str`

```rust
fn from_str(s: &str) -> Result<SmolStr, <SmolStr as FromStr>::Err>
```

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

### `impl Hash for SmolStr`

#### `hash`

```rust
fn hash<H>(&self, hasher: &mut H) where H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl Ord for SmolStr`

#### `cmp`

```rust
fn cmp(&self, other: &SmolStr) -> Ordering
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

### `impl<'a> PartialEq<&'a String> for SmolStr`

#### `eq`

```rust
fn eq(&self, other: &&'a String) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<'a> PartialEq<&'a str> for SmolStr`

#### `eq`

```rust
fn eq(&self, other: &&'a str) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<SmolStr> for &str`

#### `eq`

```rust
fn eq(&self, other: &SmolStr) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<SmolStr> for str`

#### `eq`

```rust
fn eq(&self, other: &SmolStr) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<String> for SmolStr`

#### `eq`

```rust
fn eq(&self, other: &String) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<str> for SmolStr`

#### `eq`

```rust
fn eq(&self, other: &str) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq for SmolStr`

#### `eq`

```rust
fn eq(&self, other: &SmolStr) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<'a> PartialOrd<&'a String> for SmolStr`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &&'a String) -> Option<Ordering>
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

### `impl<'a> PartialOrd<&'a str> for SmolStr`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &&'a str) -> Option<Ordering>
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

### `impl PartialOrd<SmolStr> for &str`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &SmolStr) -> Option<Ordering>
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

### `impl PartialOrd<String> for SmolStr`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &String) -> Option<Ordering>
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

### `impl PartialOrd<str> for SmolStr`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &str) -> Option<Ordering>
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

### `impl PartialOrd for SmolStr`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &SmolStr) -> Option<Ordering>
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

### `impl Eq for SmolStr`

## Auto Trait Implementations

- `impl Freeze for SmolStr`
- `impl RefUnwindSafe for SmolStr`
- `impl Send for SmolStr`
- `impl Sync for SmolStr`
- `impl Unpin for SmolStr`
- `impl UnsafeUnpin for SmolStr`
- `impl UnwindSafe for SmolStr`
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

### `impl<P, T> Receiver for P where P: Deref<Target = T> + ?Sized, T: ?Sized,`

#### `Target`

```rust
type Target = T
```

*🔬This is a nightly-only experimental API. (arbitrary_self_types)*

The target type on which the method may be called.

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

## Methods from Deref<Target = str>
