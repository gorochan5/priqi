---
title: "ArrayVec"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::arrayvec::ArrayVec"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::arrayvec"
file: "iced_renderer/geometry/path/lyon_path/geom/arrayvec/struct.ArrayVec.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/arrayvec/struct.ArrayVec.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::arrayvec::ArrayVec`

```rust
pub struct ArrayVec<T, const CAP: usize> { /* private fields */ }
```

A vector with a fixed capacity.

The `ArrayVec` is a vector backed by a fixed size array. It keeps track of the number of initialized elements. The `ArrayVec<T, CAP>` is parameterized by `T` for the element type and `CAP` for the maximum capacity.

`CAP` is of type `usize` but is range limited to `u32::MAX` (or `u16::MAX` on 16-bit targets); attempting to create larger arrayvecs with larger capacity will panic.

The vector is a contiguous value (storing the elements inline) that you can store directly on the stack if needed.

It offers a simple API but also dereferences to a slice, so that the full slice API is available. The ArrayVec can be converted into a by value iterator.

**Source:** [arrayvec.rs.html#43](https://docs.rs/arrayvec/0.7/src/arrayvec/arrayvec.rs.html#43) (lines 43)

## Methods

### `impl<T, const CAP: usize> ArrayVec<T, CAP>`

#### `new`

```rust
pub fn new() -> ArrayVec<T, CAP>
```

Create a new empty `ArrayVec`.

The maximum capacity is given by the generic parameter `CAP`.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::<_, 16>::new();
array.push(1);
array.push(2);
assert_eq!(&array[..], &[1, 2]);
assert_eq!(array.capacity(), 16);
```

#### `new_const`

```rust
pub const fn new_const() -> ArrayVec<T, CAP>
```

Create a new empty `ArrayVec` (const fn).

The maximum capacity is given by the generic parameter `CAP`.

```rust
use arrayvec::ArrayVec;

static ARRAY: ArrayVec<u8, 1024> = ArrayVec::new_const();
```

#### `len`

```rust
pub const fn len(&self) -> usize
```

Return the number of elements in the `ArrayVec`.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::from([1, 2, 3]);
array.pop();
assert_eq!(array.len(), 2);
```

#### `is_empty`

```rust
pub const fn is_empty(&self) -> bool
```

Returns whether the `ArrayVec` is empty.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::from([1]);
array.pop();
assert_eq!(array.is_empty(), true);
```

#### `capacity`

```rust
pub const fn capacity(&self) -> usize
```

Return the capacity of the `ArrayVec`.

```rust
use arrayvec::ArrayVec;

let array = ArrayVec::from([1, 2, 3]);
assert_eq!(array.capacity(), 3);
```

#### `is_full`

```rust
pub const fn is_full(&self) -> bool
```

Return true if the `ArrayVec` is completely filled to its capacity, false otherwise.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::<_, 1>::new();
assert!(!array.is_full());
array.push(1);
assert!(array.is_full());
```

#### `remaining_capacity`

```rust
pub const fn remaining_capacity(&self) -> usize
```

Returns the capacity left in the `ArrayVec`.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::from([1, 2, 3]);
array.pop();
assert_eq!(array.remaining_capacity(), 1);
```

#### `push`

```rust
pub fn push(&mut self, element: T)
```

Push `element` to the end of the vector.

***Panics*** if the vector is already full.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::<_, 2>::new();

array.push(1);
array.push(2);

assert_eq!(&array[..], &[1, 2]);
```

#### `try_push`

```rust
pub fn try_push(&mut self, element: T) -> Result<(), CapacityError<T>>
```

Push `element` to the end of the vector.

Return `Ok` if the push succeeds, or return an error if the vector is already full.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::<_, 2>::new();

let push1 = array.try_push(1);
let push2 = array.try_push(2);

assert!(push1.is_ok());
assert!(push2.is_ok());

assert_eq!(&array[..], &[1, 2]);

let overflow = array.try_push(3);

assert!(overflow.is_err());
```

#### `push_unchecked`

```rust
pub unsafe fn push_unchecked(&mut self, element: T)
```

Push `element` to the end of the vector without checking the capacity.

It is up to the caller to ensure the capacity of the vector is sufficiently large.

This method uses *debug assertions* to check that the arrayvec is not full.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::<_, 2>::new();

if array.len() + 2 <= array.capacity() {
    unsafe {
        array.push_unchecked(1);
        array.push_unchecked(2);
    }
}

assert_eq!(&array[..], &[1, 2]);
```

#### `truncate`

```rust
pub fn truncate(&mut self, new_len: usize)
```

Shortens the vector, keeping the first `len` elements and dropping the rest.

If `len` is greater than the vector’s current length this has no effect.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::from([1, 2, 3, 4, 5]);
array.truncate(3);
assert_eq!(&array[..], &[1, 2, 3]);
array.truncate(4);
assert_eq!(&array[..], &[1, 2, 3]);
```

#### `clear`

```rust
pub fn clear(&mut self)
```

Remove all elements in the vector.

#### `insert`

```rust
pub fn insert(&mut self, index: usize, element: T)
```

Insert `element` at position `index`.

Shift up all elements after `index`.

It is an error if the index is greater than the length or if the arrayvec is full.

***Panics*** if the array is full or the `index` is out of bounds. See `try_insert` for fallible version.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::<_, 2>::new();

array.insert(0, "x");
array.insert(0, "y");
assert_eq!(&array[..], &["y", "x"]);
```

#### `try_insert`

```rust
pub fn try_insert( &mut self, index: usize, element: T, ) -> Result<(), CapacityError<T>>
```

Insert `element` at position `index`.

Shift up all elements after `index`; the `index` must be less than or equal to the length.

Returns an error if vector is already at full capacity.

***Panics*** `index` is out of bounds.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::<_, 2>::new();

assert!(array.try_insert(0, "x").is_ok());
assert!(array.try_insert(0, "y").is_ok());
assert!(array.try_insert(0, "z").is_err());
assert_eq!(&array[..], &["y", "x"]);
```

#### `pop`

```rust
pub fn pop(&mut self) -> Option<T>
```

Remove the last element in the vector and return it.

Return `Some(` *element* `)` if the vector is non-empty, else `None`.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::<_, 2>::new();

array.push(1);

assert_eq!(array.pop(), Some(1));
assert_eq!(array.pop(), None);
```

#### `swap_remove`

```rust
pub fn swap_remove(&mut self, index: usize) -> T
```

Remove the element at `index` and swap the last element into its place.

This operation is O(1).

Return the *element* if the index is in bounds, else panic.

***Panics*** if the `index` is out of bounds.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::from([1, 2, 3]);

assert_eq!(array.swap_remove(0), 1);
assert_eq!(&array[..], &[3, 2]);

assert_eq!(array.swap_remove(1), 2);
assert_eq!(&array[..], &[3]);
```

#### `swap_pop`

```rust
pub fn swap_pop(&mut self, index: usize) -> Option<T>
```

Remove the element at `index` and swap the last element into its place.

This is a checked version of `.swap_remove`.
This operation is O(1).

Return `Some(` *element* `)` if the index is in bounds, else `None`.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::from([1, 2, 3]);

assert_eq!(array.swap_pop(0), Some(1));
assert_eq!(&array[..], &[3, 2]);

assert_eq!(array.swap_pop(10), None);
```

#### `remove`

```rust
pub fn remove(&mut self, index: usize) -> T
```

Remove the element at `index` and shift down the following elements.

The `index` must be strictly less than the length of the vector.

***Panics*** if the `index` is out of bounds.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::from([1, 2, 3]);

let removed_elt = array.remove(0);
assert_eq!(removed_elt, 1);
assert_eq!(&array[..], &[2, 3]);
```

#### `pop_at`

```rust
pub fn pop_at(&mut self, index: usize) -> Option<T>
```

Remove the element at `index` and shift down the following elements.

This is a checked version of `.remove(index)`. Returns `None` if there is no element at `index`. Otherwise, return the element inside `Some`.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::from([1, 2, 3]);

assert!(array.pop_at(0).is_some());
assert_eq!(&array[..], &[2, 3]);

assert!(array.pop_at(2).is_none());
assert!(array.pop_at(10).is_none());
```

#### `retain`

```rust
pub fn retain<F>(&mut self, f: F) where F: FnMut(&mut T) -> bool,
```

Retains only the elements specified by the predicate.

In other words, remove all elements `e` such that `f(&mut e)` returns false. This method operates in place and preserves the order of the retained elements.

```rust
use arrayvec::ArrayVec;

let mut array = ArrayVec::from([1, 2, 3, 4]);
array.retain(|x| *x & 1 != 0 );
assert_eq!(&array[..], &[1, 3]);
```

#### `spare_capacity_mut`

```rust
pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>]
```

Returns the remaining spare capacity of the vector as a slice of `MaybeUninit<T>`.

The returned slice can be used to fill the vector with data (e.g. by reading from a file) before marking the data as initialized using the [`set_len`](struct.ArrayVec.md) method.

###### Examples

```rust
use arrayvec::ArrayVec;

// Allocate vector big enough for 10 elements.
let mut v: ArrayVec<i32, 10> = ArrayVec::new();

// Fill in the first 3 elements.
let uninit = v.spare_capacity_mut();
uninit[0].write(0);
uninit[1].write(1);
uninit[2].write(2);

// Mark the first 3 elements of the vector as being initialized.
unsafe {
    v.set_len(3);
}

assert_eq!(&v[..], &[0, 1, 2]);
```

#### `set_len`

```rust
pub unsafe fn set_len(&mut self, length: usize)
```

Set the vector’s length without dropping or moving out elements

This method is `unsafe` because it changes the notion of the number of “valid” elements in the vector. Use with care.

This method uses *debug assertions* to check that `length` is not greater than the capacity.

#### `try_extend_from_slice`

```rust
pub fn try_extend_from_slice( &mut self, other: &[T], ) -> Result<(), CapacityError> where T: Copy,
```

Copy all elements from the slice and append to the `ArrayVec`.

```rust
use arrayvec::ArrayVec;

let mut vec: ArrayVec<usize, 10> = ArrayVec::new();
vec.push(1);
vec.try_extend_from_slice(&[2, 3]).unwrap();
assert_eq!(&vec[..], &[1, 2, 3]);
```

###### Errors

This method will return an error if the capacity left (see `remaining_capacity`) is smaller then the length of the provided slice.

#### `drain`

```rust
pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, CAP> ⓘ where R: RangeBounds<usize>,
```

Create a draining iterator that removes the specified range in the vector and yields the removed items from start to end. The element range is removed even if the iterator is not consumed until the end.

Note: It is unspecified how many elements are removed from the vector, if the `Drain` value is leaked.

**Panics** if the starting point is greater than the end point or if the end point is greater than the length of the vector.

```rust
use arrayvec::ArrayVec;

let mut v1 = ArrayVec::from([1, 2, 3]);
let v2: ArrayVec<_, 3> = v1.drain(0..2).collect();
assert_eq!(&v1[..], &[3]);
assert_eq!(&v2[..], &[1, 2]);
```

#### `into_inner`

```rust
pub fn into_inner(self) -> Result<[T; CAP], ArrayVec<T, CAP>>
```

Return the inner fixed size array, if it is full to its capacity.

Return an `Ok` value with the array if length equals capacity, return an `Err` with self otherwise.

#### `into_inner_unchecked`

```rust
pub unsafe fn into_inner_unchecked(self) -> [T; CAP]
```

Return the inner fixed size array.

Safety: This operation is safe if and only if length equals capacity.

#### `take`

```rust
pub fn take(&mut self) -> ArrayVec<T, CAP>
```

Returns the ArrayVec, replacing the original with a new empty ArrayVec.

```rust
use arrayvec::ArrayVec;

let mut v = ArrayVec::from([0, 1, 2, 3]);
assert_eq!([0, 1, 2, 3], v.take().into_inner().unwrap());
assert!(v.is_empty());
```

#### `as_slice`

```rust
pub fn as_slice(&self) -> &[T]
```

Return a slice containing all elements of the vector.

#### `as_mut_slice`

```rust
pub fn as_mut_slice(&mut self) -> &mut [T]
```

Return a mutable slice containing all elements of the vector.

#### `as_ptr`

```rust
pub fn as_ptr(&self) -> *const T
```

Return a raw pointer to the vector’s buffer.

#### `as_mut_ptr`

```rust
pub fn as_mut_ptr(&mut self) -> *mut T
```

Return a raw mutable pointer to the vector’s buffer.

## Trait Implementations

### `impl<T, const CAP: usize> AsMut<[T]> for ArrayVec<T, CAP>`

#### `as_mut`

```rust
fn as_mut(&mut self) -> &mut [T]
```

Converts this type into a mutable reference of the (usually inferred) input type.

### `impl<T, const CAP: usize> AsRef<[T]> for ArrayVec<T, CAP>`

#### `as_ref`

```rust
fn as_ref(&self) -> &[T]
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl<T, const CAP: usize> Borrow<[T]> for ArrayVec<T, CAP>`

#### `borrow`

```rust
fn borrow(&self) -> &[T]
```

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

### `impl<T, const CAP: usize> BorrowMut<[T]> for ArrayVec<T, CAP>`

#### `borrow_mut`

```rust
fn borrow_mut(&mut self) -> &mut [T]
```

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

### `impl<T, const CAP: usize> Clone for ArrayVec<T, CAP> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> ArrayVec<T, CAP>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, rhs: &ArrayVec<T, CAP>)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, const CAP: usize> Debug for ArrayVec<T, CAP> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, const CAP: usize> Default for ArrayVec<T, CAP>`

#### `default`

```rust
fn default() -> ArrayVec<T, CAP>
```

Return an empty array

### `impl<T, const CAP: usize> Deref for ArrayVec<T, CAP>`

#### `Target`

```rust
type Target = [T]
```

The resulting type after dereferencing.

#### `deref`

```rust
fn deref(&self) -> &<ArrayVec<T, CAP> as Deref>::Target ⓘ
```

Dereferences the value.

### `impl<T, const CAP: usize> DerefMut for ArrayVec<T, CAP>`

#### `deref_mut`

```rust
fn deref_mut(&mut self) -> &mut <ArrayVec<T, CAP> as Deref>::Target ⓘ
```

Mutably dereferences the value.

### `impl<T, const CAP: usize> Drop for ArrayVec<T, CAP>`

#### `drop`

```rust
fn drop(&mut self)
```

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

### `impl<T, const CAP: usize> Extend<T> for ArrayVec<T, CAP>`

#### `extend`

```rust
fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item = T>,
```

Extend the `ArrayVec` with an iterator.

***Panics*** if extending the vector exceeds its capacity.

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

### `impl<T, const CAP: usize> From<[T; CAP]> for ArrayVec<T, CAP>`

#### `from`

```rust
fn from(array: [T; CAP]) -> ArrayVec<T, CAP>
```

Converts to this type from the input type.

### `impl<T, const CAP: usize> FromIterator<T> for ArrayVec<T, CAP>`

#### `from_iter`

```rust
fn from_iter<I>(iter: I) -> ArrayVec<T, CAP> where I: IntoIterator<Item = T>,
```

Create an `ArrayVec` from an iterator.

***Panics*** if the number of elements in the iterator exceeds the arrayvec’s capacity.

### `impl<T, const CAP: usize> Hash for ArrayVec<T, CAP> where T: Hash,`

#### `hash`

```rust
fn hash<H>(&self, state: &mut H) where H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl<'a, T, const CAP: usize> IntoIterator for &'a ArrayVec<T, CAP> where T: 'a,`

#### `Item`

```rust
type Item = &'a T
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = Iter<'a, T>
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> <&'a ArrayVec<T, CAP> as IntoIterator>::IntoIter ⓘ
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

### `impl<'a, T, const CAP: usize> IntoIterator for &'a mut ArrayVec<T, CAP> where T: 'a,`

#### `Item`

```rust
type Item = &'a mut T
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = IterMut<'a, T>
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> <&'a mut ArrayVec<T, CAP> as IntoIterator>::IntoIter ⓘ
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

### `impl<T, const CAP: usize> IntoIterator for ArrayVec<T, CAP>`

#### `Item`

```rust
type Item = T
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = IntoIter<T, CAP>
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> IntoIter<T, CAP> ⓘ
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

### `impl<T, const CAP: usize> Ord for ArrayVec<T, CAP> where T: Ord,`

#### `cmp`

```rust
fn cmp(&self, other: &ArrayVec<T, CAP>) -> Ordering
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

### `impl<T, const CAP: usize> PartialEq<[T]> for ArrayVec<T, CAP> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &[T]) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, const CAP: usize> PartialEq for ArrayVec<T, CAP> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &ArrayVec<T, CAP>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, const CAP: usize> PartialOrd for ArrayVec<T, CAP> where T: PartialOrd,`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &ArrayVec<T, CAP>) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, other: &ArrayVec<T, CAP>) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, other: &ArrayVec<T, CAP>) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `ge`

```rust
fn ge(&self, other: &ArrayVec<T, CAP>) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

#### `gt`

```rust
fn gt(&self, other: &ArrayVec<T, CAP>) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

### `impl<T, const CAP: usize> TryFrom<&[T]> for ArrayVec<T, CAP> where T: Clone,`

#### `Error`

```rust
type Error = CapacityError
```

The type returned in the event of a conversion error.

#### `try_from`

```rust
fn try_from( slice: &[T], ) -> Result<ArrayVec<T, CAP>, <ArrayVec<T, CAP> as TryFrom<&[T]>>::Error>
```

Performs the conversion.

### `impl<const CAP: usize> Write for ArrayVec<u8, CAP>`

#### `write`

```rust
fn write(&mut self, data: &[u8]) -> Result<usize, Error>
```

Writes a buffer into this writer, returning how many bytes were written. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write)

#### `flush`

```rust
fn flush(&mut self) -> Result<(), Error>
```

Flushes this output stream, ensuring that all intermediately buffered contents reach their destination. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush)

#### `write_vectored`

```rust
fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize, Error>
```

Like [`write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write), except that it writes from a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored)

#### `is_write_vectored`

```rust
fn is_write_vectored(&self) -> bool
```

*🔬This is a nightly-only experimental API. (can_vector)*

Determines if this `Write`r has an efficient [`write_vectored`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored) implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored)

#### `write_all`

```rust
fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>
```

Attempts to write an entire buffer into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all)

#### `write_all_vectored`

```rust
fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> Result<(), Error>
```

*🔬This is a nightly-only experimental API. (write_all_vectored)*

Attempts to write multiple buffers into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored)

#### `write_fmt`

```rust
fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), Error>
```

Writes a formatted string into this writer, returning any error encountered. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt)

#### `by_ref`

```rust
fn by_ref(&mut self) -> &mut Self where Self: Sized,
```

Creates a “by reference” adapter for this instance of `Write`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref)

### `impl<T, const CAP: usize> Eq for ArrayVec<T, CAP> where T: Eq,`

## Auto Trait Implementations

- `impl<T, const CAP: usize> Freeze for ArrayVec<T, CAP> where T: Freeze,`
- `impl<T, const CAP: usize> RefUnwindSafe for ArrayVec<T, CAP> where T: RefUnwindSafe,`
- `impl<T, const CAP: usize> Send for ArrayVec<T, CAP> where T: Send,`
- `impl<T, const CAP: usize> Sync for ArrayVec<T, CAP> where T: Sync,`
- `impl<T, const CAP: usize> Unpin for ArrayVec<T, CAP> where T: Unpin,`
- `impl<T, const CAP: usize> UnsafeUnpin for ArrayVec<T, CAP> where T: UnsafeUnpin,`
- `impl<T, const CAP: usize> UnwindSafe for ArrayVec<T, CAP> where T: UnwindSafe,`
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

### `impl<W> WriteBytesExt for W where W: Write + ?Sized,`

#### `write_u8`

```rust
fn write_u8(&mut self, n: u8) -> Result<(), Error>
```

Writes an unsigned 8 bit integer to the underlying writer. Read more

#### `write_i8`

```rust
fn write_i8(&mut self, n: i8) -> Result<(), Error>
```

Writes a signed 8 bit integer to the underlying writer. Read more

#### `write_u16`

```rust
fn write_u16<T>(&mut self, n: u16) -> Result<(), Error> where T: ByteOrder,
```

Writes an unsigned 16 bit integer to the underlying writer. Read more

#### `write_i16`

```rust
fn write_i16<T>(&mut self, n: i16) -> Result<(), Error> where T: ByteOrder,
```

Writes a signed 16 bit integer to the underlying writer. Read more

#### `write_u24`

```rust
fn write_u24<T>(&mut self, n: u32) -> Result<(), Error> where T: ByteOrder,
```

Writes an unsigned 24 bit integer to the underlying writer. Read more

#### `write_i24`

```rust
fn write_i24<T>(&mut self, n: i32) -> Result<(), Error> where T: ByteOrder,
```

Writes a signed 24 bit integer to the underlying writer. Read more

#### `write_u32`

```rust
fn write_u32<T>(&mut self, n: u32) -> Result<(), Error> where T: ByteOrder,
```

Writes an unsigned 32 bit integer to the underlying writer. Read more

#### `write_i32`

```rust
fn write_i32<T>(&mut self, n: i32) -> Result<(), Error> where T: ByteOrder,
```

Writes a signed 32 bit integer to the underlying writer. Read more

#### `write_u48`

```rust
fn write_u48<T>(&mut self, n: u64) -> Result<(), Error> where T: ByteOrder,
```

Writes an unsigned 48 bit integer to the underlying writer. Read more

#### `write_i48`

```rust
fn write_i48<T>(&mut self, n: i64) -> Result<(), Error> where T: ByteOrder,
```

Writes a signed 48 bit integer to the underlying writer. Read more

#### `write_u64`

```rust
fn write_u64<T>(&mut self, n: u64) -> Result<(), Error> where T: ByteOrder,
```

Writes an unsigned 64 bit integer to the underlying writer. Read more

#### `write_i64`

```rust
fn write_i64<T>(&mut self, n: i64) -> Result<(), Error> where T: ByteOrder,
```

Writes a signed 64 bit integer to the underlying writer. Read more

#### `write_u128`

```rust
fn write_u128<T>(&mut self, n: u128) -> Result<(), Error> where T: ByteOrder,
```

Writes an unsigned 128 bit integer to the underlying writer.

#### `write_i128`

```rust
fn write_i128<T>(&mut self, n: i128) -> Result<(), Error> where T: ByteOrder,
```

Writes a signed 128 bit integer to the underlying writer.

#### `write_uint`

```rust
fn write_uint<T>(&mut self, n: u64, nbytes: usize) -> Result<(), Error> where T: ByteOrder,
```

Writes an unsigned n-bytes integer to the underlying writer. Read more

#### `write_int`

```rust
fn write_int<T>(&mut self, n: i64, nbytes: usize) -> Result<(), Error> where T: ByteOrder,
```

Writes a signed n-bytes integer to the underlying writer. Read more

#### `write_uint128`

```rust
fn write_uint128<T>(&mut self, n: u128, nbytes: usize) -> Result<(), Error> where T: ByteOrder,
```

Writes an unsigned n-bytes integer to the underlying writer. Read more

#### `write_int128`

```rust
fn write_int128<T>(&mut self, n: i128, nbytes: usize) -> Result<(), Error> where T: ByteOrder,
```

Writes a signed n-bytes integer to the underlying writer. Read more

#### `write_f32`

```rust
fn write_f32<T>(&mut self, n: f32) -> Result<(), Error> where T: ByteOrder,
```

Writes a IEEE754 single-precision (4 bytes) floating point number to the underlying writer. Read more

#### `write_f64`

```rust
fn write_f64<T>(&mut self, n: f64) -> Result<(), Error> where T: ByteOrder,
```

Writes a IEEE754 double-precision (8 bytes) floating point number to the underlying writer. Read more

### `impl<W> WriteEndian<[f32]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[f32]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[f32]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[f64]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[f64]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[f64]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[i128]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[i128]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[i128]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[i16]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[i16]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[i16]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[i32]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[i32]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[i32]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[i64]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[i64]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[i64]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[i8]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[i8]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[i8]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[u128]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[u128]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[u128]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[u16]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[u16]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[u16]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[u32]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[u32]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[u32]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[u64]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[u64]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[u64]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<[u8]> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &[u8]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &[u8]) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<f32> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &f32) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &f32) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<f64> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &f64) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &f64) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<i128> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &i128) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &i128) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<i16> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &i16) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &i16) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<i32> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &i32) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &i32) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<i64> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &i64) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &i64) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<i8> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &i8) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &i8) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<u128> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &u128) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &u128) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<u16> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &u16) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &u16) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<u32> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &u32) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &u32) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<u64> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &u64) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &u64) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<W> WriteEndian<u8> for W where W: Write,`

#### `write_as_little_endian`

```rust
fn write_as_little_endian(&mut self, value: &u8) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to little endianness

#### `write_as_big_endian`

```rust
fn write_as_big_endian(&mut self, value: &u8) -> Result<(), Error>
```

Write the byte value of the specified reference, converting it to big endianness

#### `write_as_native_endian`

```rust
fn write_as_native_endian(&mut self, value: &T) -> Result<(), Error>
```

Write the byte value of the specified reference, not converting it

### `impl<T> ZByteWriterTrait for T where T: Write,`

#### `write_bytes`

```rust
fn write_bytes(&mut self, buf: &[u8]) -> Result<usize, ZByteIoError>
```

Write some bytes into the sink returning number of bytes written or an error if something bad happened Read more

#### `write_all_bytes`

```rust
fn write_all_bytes(&mut self, buf: &[u8]) -> Result<(), ZByteIoError>
```

Write all bytes to the buffer or return an error if something occurred Read more

#### `write_const_bytes`

```rust
fn write_const_bytes<const N: usize>( &mut self, buf: &[u8; N], ) -> Result<(), ZByteIoError>
```

Write a fixed number of bytes and error out if we can’t write the bytes Read more

#### `flush_bytes`

```rust
fn flush_bytes(&mut self) -> Result<(), ZByteIoError>
```

Ensure bytes are written to the sink. Read more

#### `reserve_capacity`

```rust
fn reserve_capacity(&mut self, _: usize) -> Result<(), ZByteIoError>
```

A hint to tell the implementation how big of a size we expect the image to be An implementation like in memory `Vec` can use this to reserve additional memory to prevent reallocation when encoding Read more

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> NumBytes for T where T: Debug + AsRef<[u8]> + AsMut<[u8]> + PartialEq + Eq + PartialOrd + Ord + Hash + Borrow<[u8]> + BorrowMut<[u8]> + ?Sized,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`

## Methods from Deref<Target = [T]>
