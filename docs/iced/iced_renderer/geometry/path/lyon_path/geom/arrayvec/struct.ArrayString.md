---
title: "ArrayString"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::arrayvec::ArrayString"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::arrayvec"
file: "iced_renderer/geometry/path/lyon_path/geom/arrayvec/struct.ArrayString.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/arrayvec/struct.ArrayString.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::arrayvec::ArrayString`

```rust
pub struct ArrayString<const CAP: usize> { /* private fields */ }
```

A string with a fixed capacity.

The `ArrayString` is a string backed by a fixed size array. It keeps track of its length, and is parameterized by `CAP` for the maximum capacity.

`CAP` is of type `usize` but is range limited to `u32::MAX` (or `u16` on 16-bit targets); attempting to create larger arrayvecs with larger capacity will panic.

The string is a contiguous value that you can store directly on the stack if needed.

**Source:** [array_string.rs.html#37](https://docs.rs/arrayvec/0.7/src/arrayvec/array_string.rs.html#37) (lines 37)

## Methods

### `impl<const CAP: usize> ArrayString<CAP>`

#### `new`

```rust
pub fn new() -> ArrayString<CAP>
```

Create a new empty `ArrayString`.

Capacity is inferred from the type parameter.

```rust
use arrayvec::ArrayString;

let mut string = ArrayString::<16>::new();
string.push_str("foo");
assert_eq!(&string[..], "foo");
assert_eq!(string.capacity(), 16);
```

#### `new_const`

```rust
pub const fn new_const() -> ArrayString<CAP>
```

Create a new empty `ArrayString` (const fn).

Capacity is inferred from the type parameter.

```rust
use arrayvec::ArrayString;

static ARRAY: ArrayString<1024> = ArrayString::new_const();
```

#### `len`

```rust
pub const fn len(&self) -> usize
```

Return the length of the string.

#### `is_empty`

```rust
pub const fn is_empty(&self) -> bool
```

Returns whether the string is empty.

#### `from`

```rust
pub fn from(s: &str) -> Result<ArrayString<CAP>, CapacityError<&str>>
```

Create a new `ArrayString` from a `str`.

Capacity is inferred from the type parameter.

**Errors** if the backing array is not large enough to fit the string.

```rust
use arrayvec::ArrayString;

let mut string = ArrayString::<3>::from("foo").unwrap();
assert_eq!(&string[..], "foo");
assert_eq!(string.len(), 3);
assert_eq!(string.capacity(), 3);
```

#### `from_byte_string`

```rust
pub fn from_byte_string(b: &[u8; CAP]) -> Result<ArrayString<CAP>, Utf8Error>
```

Create a new `ArrayString` from a byte string literal.

**Errors** if the byte string literal is not valid UTF-8.

```rust
use arrayvec::ArrayString;

let string = ArrayString::from_byte_string(b"hello world").unwrap();
```

#### `zero_filled`

```rust
pub fn zero_filled() -> ArrayString<CAP>
```

Create a new `ArrayString` value fully filled with ASCII NULL characters (`\0`). Useful to be used as a buffer to collect external data or as a buffer for intermediate processing.

```rust
use arrayvec::ArrayString;

let string = ArrayString::<16>::zero_filled();
assert_eq!(string.len(), 16);
```

#### `capacity`

```rust
pub const fn capacity(&self) -> usize
```

Return the capacity of the `ArrayString`.

```rust
use arrayvec::ArrayString;

let string = ArrayString::<3>::new();
assert_eq!(string.capacity(), 3);
```

#### `is_full`

```rust
pub const fn is_full(&self) -> bool
```

Return if the `ArrayString` is completely filled.

```rust
use arrayvec::ArrayString;

let mut string = ArrayString::<1>::new();
assert!(!string.is_full());
string.push_str("A");
assert!(string.is_full());
```

#### `remaining_capacity`

```rust
pub const fn remaining_capacity(&self) -> usize
```

Returns the capacity left in the `ArrayString`.

```rust
use arrayvec::ArrayString;

let mut string = ArrayString::<3>::from("abc").unwrap();
string.pop();
assert_eq!(string.remaining_capacity(), 1);
```

#### `push`

```rust
pub fn push(&mut self, c: char)
```

Adds the given char to the end of the string.

***Panics*** if the backing array is not large enough to fit the additional char.

```rust
use arrayvec::ArrayString;

let mut string = ArrayString::<2>::new();

string.push('a');
string.push('b');

assert_eq!(&string[..], "ab");
```

#### `try_push`

```rust
pub fn try_push(&mut self, c: char) -> Result<(), CapacityError<char>>
```

Adds the given char to the end of the string.

Returns `Ok` if the push succeeds.

**Errors** if the backing array is not large enough to fit the additional char.

```rust
use arrayvec::ArrayString;

let mut string = ArrayString::<2>::new();

string.try_push('a').unwrap();
string.try_push('b').unwrap();
let overflow = string.try_push('c');

assert_eq!(&string[..], "ab");
assert_eq!(overflow.unwrap_err().element(), 'c');
```

#### `push_str`

```rust
pub fn push_str(&mut self, s: &str)
```

Adds the given string slice to the end of the string.

***Panics*** if the backing array is not large enough to fit the string.

```rust
use arrayvec::ArrayString;

let mut string = ArrayString::<2>::new();

string.push_str("a");
string.push_str("d");

assert_eq!(&string[..], "ad");
```

#### `try_push_str`

```rust
pub fn try_push_str<'a>( &mut self, s: &'a str, ) -> Result<(), CapacityError<&'a str>>
```

Adds the given string slice to the end of the string.

Returns `Ok` if the push succeeds.

**Errors** if the backing array is not large enough to fit the string.

```rust
use arrayvec::ArrayString;

let mut string = ArrayString::<2>::new();

string.try_push_str("a").unwrap();
let overflow1 = string.try_push_str("bc");
string.try_push_str("d").unwrap();
let overflow2 = string.try_push_str("ef");

assert_eq!(&string[..], "ad");
assert_eq!(overflow1.unwrap_err().element(), "bc");
assert_eq!(overflow2.unwrap_err().element(), "ef");
```

#### `pop`

```rust
pub fn pop(&mut self) -> Option<char>
```

Removes the last character from the string and returns it.

Returns `None` if this `ArrayString` is empty.

```rust
use arrayvec::ArrayString;

let mut s = ArrayString::<3>::from("foo").unwrap();

assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('f'));

assert_eq!(s.pop(), None);
```

#### `truncate`

```rust
pub fn truncate(&mut self, new_len: usize)
```

Shortens this `ArrayString` to the specified length.

If `new_len` is greater than the string’s current length, this has no effect.

***Panics*** if `new_len` does not lie on a `char` boundary.

```rust
use arrayvec::ArrayString;

let mut string = ArrayString::<6>::from("foobar").unwrap();
string.truncate(3);
assert_eq!(&string[..], "foo");
string.truncate(4);
assert_eq!(&string[..], "foo");
```

#### `remove`

```rust
pub fn remove(&mut self, idx: usize) -> char
```

Removes a `char` from this `ArrayString` at a byte position and returns it.

This is an `O(n)` operation, as it requires copying every element in the array.

***Panics*** if `idx` is larger than or equal to the `ArrayString`’s length, or if it does not lie on a `char` boundary.

```rust
use arrayvec::ArrayString;

let mut s = ArrayString::<3>::from("foo").unwrap();

assert_eq!(s.remove(0), 'f');
assert_eq!(s.remove(1), 'o');
assert_eq!(s.remove(0), 'o');
```

#### `clear`

```rust
pub fn clear(&mut self)
```

Make the string empty.

#### `set_len`

```rust
pub unsafe fn set_len(&mut self, length: usize)
```

Set the strings’s length.

This function is `unsafe` because it changes the notion of the number of “valid” bytes in the string. Use with care.

This method uses *debug assertions* to check the validity of `length` and may use other debug assertions.

#### `as_str`

```rust
pub fn as_str(&self) -> &str
```

Return a string slice of the whole `ArrayString`.

#### `as_mut_str`

```rust
pub fn as_mut_str(&mut self) -> &mut str
```

Return a mutable string slice of the whole `ArrayString`.

#### `as_ptr`

```rust
pub fn as_ptr(&self) -> *const u8
```

Return a raw pointer to the string’s buffer.

#### `as_mut_ptr`

```rust
pub fn as_mut_ptr(&mut self) -> *mut u8
```

Return a raw mutable pointer to the string’s buffer.

## Trait Implementations

### `impl<const CAP: usize> AsRef<Path> for ArrayString<CAP>`

#### `as_ref`

```rust
fn as_ref(&self) -> &Path
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl<const CAP: usize> AsRef<str> for ArrayString<CAP>`

#### `as_ref`

```rust
fn as_ref(&self) -> &str
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl<const CAP: usize> Borrow<str> for ArrayString<CAP>`

#### `borrow`

```rust
fn borrow(&self) -> &str
```

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

### `impl<const CAP: usize> BorrowMut<str> for ArrayString<CAP>`

#### `borrow_mut`

```rust
fn borrow_mut(&mut self) -> &mut str
```

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

### `impl<const CAP: usize> Clone for ArrayString<CAP>`

#### `clone`

```rust
fn clone(&self) -> ArrayString<CAP>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, rhs: &ArrayString<CAP>)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<const CAP: usize> Debug for ArrayString<CAP>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<const CAP: usize> Default for ArrayString<CAP>`

#### `default`

```rust
fn default() -> ArrayString<CAP>
```

Return an empty `ArrayString`

### `impl<const CAP: usize> Deref for ArrayString<CAP>`

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

### `impl<const CAP: usize> DerefMut for ArrayString<CAP>`

#### `deref_mut`

```rust
fn deref_mut(&mut self) -> &mut str
```

Mutably dereferences the value.

### `impl<const CAP: usize> Display for ArrayString<CAP>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl<const CAP: usize> FromStr for ArrayString<CAP>`

#### `Err`

```rust
type Err = CapacityError
```

The associated error which can be returned from parsing.

#### `from_str`

```rust
fn from_str( s: &str, ) -> Result<ArrayString<CAP>, <ArrayString<CAP> as FromStr>::Err>
```

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

### `impl<const CAP: usize> Hash for ArrayString<CAP>`

#### `hash`

```rust
fn hash<H>(&self, h: &mut H) where H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl<const CAP: usize> Ord for ArrayString<CAP>`

#### `cmp`

```rust
fn cmp(&self, rhs: &ArrayString<CAP>) -> Ordering
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

### `impl<const CAP: usize> PartialEq<ArrayString<CAP>> for str`

#### `eq`

```rust
fn eq(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<const CAP: usize> PartialEq<str> for ArrayString<CAP>`

#### `eq`

```rust
fn eq(&self, rhs: &str) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<const CAP: usize> PartialEq for ArrayString<CAP>`

#### `eq`

```rust
fn eq(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<const CAP: usize> PartialOrd<ArrayString<CAP>> for str`

#### `partial_cmp`

```rust
fn partial_cmp(&self, rhs: &ArrayString<CAP>) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `gt`

```rust
fn gt(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

#### `ge`

```rust
fn ge(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

### `impl<const CAP: usize> PartialOrd<str> for ArrayString<CAP>`

#### `partial_cmp`

```rust
fn partial_cmp(&self, rhs: &str) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, rhs: &str) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, rhs: &str) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `gt`

```rust
fn gt(&self, rhs: &str) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

#### `ge`

```rust
fn ge(&self, rhs: &str) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

### `impl<const CAP: usize> PartialOrd for ArrayString<CAP>`

#### `partial_cmp`

```rust
fn partial_cmp(&self, rhs: &ArrayString<CAP>) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `gt`

```rust
fn gt(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

#### `ge`

```rust
fn ge(&self, rhs: &ArrayString<CAP>) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

### `impl<'a, const CAP: usize> TryFrom<&'a str> for ArrayString<CAP>`

#### `Error`

```rust
type Error = CapacityError<&'a str>
```

The type returned in the event of a conversion error.

#### `try_from`

```rust
fn try_from( f: &'a str, ) -> Result<ArrayString<CAP>, <ArrayString<CAP> as TryFrom<&'a str>>::Error>
```

Performs the conversion.

### `impl<'a, const CAP: usize> TryFrom<Arguments<'a>> for ArrayString<CAP>`

#### `Error`

```rust
type Error = CapacityError<Error>
```

The type returned in the event of a conversion error.

#### `try_from`

```rust
fn try_from( f: Arguments<'a>, ) -> Result<ArrayString<CAP>, <ArrayString<CAP> as TryFrom<Arguments<'a>>>::Error>
```

Performs the conversion.

### `impl<const CAP: usize> Write for ArrayString<CAP>`

#### `write_char`

```rust
fn write_char(&mut self, c: char) -> Result<(), Error>
```

Writes a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html) into this writer, returning whether the write succeeded. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_char)

#### `write_str`

```rust
fn write_str(&mut self, s: &str) -> Result<(), Error>
```

Writes a string slice into this writer, returning whether the write succeeded. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#tymethod.write_str)

#### `write_fmt`

```rust
fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), Error>
```

Glue for usage of the [`write!`](https://doc.rust-lang.org/nightly/core/macro.write.html) macro with implementors of this trait. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_fmt)

### `impl<const CAP: usize> Copy for ArrayString<CAP>`

### `impl<const CAP: usize> Eq for ArrayString<CAP>`

## Auto Trait Implementations

- `impl<const CAP: usize> Freeze for ArrayString<CAP>`
- `impl<const CAP: usize> RefUnwindSafe for ArrayString<CAP>`
- `impl<const CAP: usize> Send for ArrayString<CAP>`
- `impl<const CAP: usize> Sync for ArrayString<CAP>`
- `impl<const CAP: usize> Unpin for ArrayString<CAP>`
- `impl<const CAP: usize> UnsafeUnpin for ArrayString<CAP>`
- `impl<const CAP: usize> UnwindSafe for ArrayString<CAP>`
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

### `impl<Q, K> Comparable<K> for Q where Q: Ord + ?Sized, K: Borrow<Q> + ?Sized,`

#### `compare`

```rust
fn compare(&self, key: &K) -> Ordering
```

Compare self to `key` and return their ordering.

### `impl<T> Downcast<T> for T`

#### `downcast`

```rust
fn downcast(&self) -> &T
```

### `impl<T> Downcast for T where T: Any,`

#### `into_any`

```rust
fn into_any(self: Box<T>) -> Box<dyn Any>
```

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `into_any_rc`

```rust
fn into_any_rc(self: Rc<T>) -> Rc<dyn Any>
```

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `as_any`

```rust
fn as_any(&self) -> &(dyn Any + 'static)
```

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

#### `as_any_mut`

```rust
fn as_any_mut(&mut self) -> &mut (dyn Any + 'static)
```

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

### `impl<T> DowncastSync for T where T: Any + Send + Sync,`

#### `into_any_arc`

```rust
fn into_any_arc(self: Arc<T>) -> Arc<dyn Any + Sync + Send>
```

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Checks if this value is equivalent to the given key. Read more

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Compare self to `key` and return `true` if they are equal.

### `impl<T> From<T> for T`

#### `from`

```rust
fn from(t: T) -> T
```

Returns the argument unchanged.

### `impl<T> Instrument for T`

#### `instrument`

```rust
fn instrument(self, span: Span) -> Instrumented<Self>
```

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

#### `in_current_span`

```rust
fn in_current_span(self) -> Instrumented<Self>
```

Instruments this type with the current `Span`, returning an `Instrumented` wrapper. Read more

### `impl<T, U> Into<U> for T where U: From<T>,`

#### `into`

```rust
fn into(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

### `impl<T> IntoEither for T`

#### `into_either`

```rust
fn into_either(self, into_left: bool) -> Either<Self, Self>
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either)

#### `into_either_with`

```rust
fn into_either_with<F>(self, into_left: F) -> Either<Self, Self> where F: FnOnce(&Self) -> bool,
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either_with)

### `impl<T> Pointable for T`

#### `ALIGN`

```rust
const ALIGN: usize
```

The alignment of pointer.

#### `Init`

```rust
type Init = T
```

The type for initializers.

#### `init`

```rust
unsafe fn init(init: <T as Pointable>::Init) -> usize
```

Initializes a with the given initializer. Read more

#### `deref`

```rust
unsafe fn deref<'a>(ptr: usize) -> &'a T
```

Dereferences the given pointer. Read more

#### `deref_mut`

```rust
unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T
```

Mutably dereferences the given pointer. Read more

#### `drop`

```rust
unsafe fn drop(ptr: usize)
```

Drops the object pointed to by the given pointer. Read more

### `impl<R, P> ReadPrimitive<R> for P where R: Read + ReadEndian<P>, P: Default,`

#### `read_from_little_endian`

```rust
fn read_from_little_endian(read: &mut R) -> Result<Self, Error>
```

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

#### `read_from_big_endian`

```rust
fn read_from_big_endian(read: &mut R) -> Result<Self, Error>
```

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

#### `read_from_native_endian`

```rust
fn read_from_native_endian(read: &mut R) -> Result<Self, Error>
```

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

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

### `impl<T> Upcast<T> for T`

#### `upcast`

```rust
fn upcast(&self) -> Option<&T>
```

### `impl<T> WithSubscriber for T`

#### `with_subscriber`

```rust
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> where S: Into<Dispatch>,
```

Attaches the provided `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

#### `with_current_subscriber`

```rust
fn with_current_subscriber(self) -> WithDispatch<Self>
```

Attaches the current default `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`

## Methods from Deref<Target = str>
