---
title: "Bytes"
kind: struct
path: "iced_core::Bytes"
crate: "iced_core"
module: "iced_core"
file: "iced_core/struct.Bytes.md"
docs_url: "https://docs.iced.rs/iced_core/struct.Bytes.html"
---

# `iced_core::Bytes`

```rust
pub struct Bytes { /* private fields */ }
```

A cheaply cloneable and sliceable chunk of contiguous memory.

`Bytes` is an efficient container for storing and operating on contiguous slices of memory. It is intended for use primarily in networking code, but could have applications elsewhere as well.

`Bytes` values facilitate zero-copy network programming by allowing multiple `Bytes` objects to point to the same underlying memory.

`Bytes` does not have a single implementation. It is an interface, whose exact behavior is implemented through dynamic dispatch in several underlying implementations of `Bytes`.

All `Bytes` implementations must fulfill the following requirements:

- They are cheaply cloneable and thereby shareable between an unlimited amount of components, for example by modifying a reference count.
- Instances can be sliced to refer to a subset of the original buffer.

```rust
use bytes::Bytes;

let mut mem = Bytes::from("Hello world");
let a = mem.slice(0..5);

assert_eq!(a, "Hello");

let b = mem.split_to(6);

assert_eq!(mem, "world");
assert_eq!(b, "Hello ");
```

## Memory layout

The `Bytes` struct itself is fairly small, limited to 4 `usize` fields used to track information about which segment of the underlying memory the `Bytes` handle has access to.

`Bytes` keeps both a pointer to the shared state containing the full memory slice and a pointer to the start of the region visible by the handle. `Bytes` also tracks the length of its view into the memory.

## Sharing

`Bytes` contains a vtable, which allows implementations of `Bytes` to define how sharing/cloning is implemented in detail. When `Bytes::clone()` is called, `Bytes` will call the vtable function for cloning the backing storage in order to share it behind multiple `Bytes` instances.

For `Bytes` implementations which refer to constant memory (e.g. created via `Bytes::from_static()`) the cloning implementation will be a no-op.

For `Bytes` implementations which point to a reference counted shared storage (e.g. an `Arc<[u8]>`), sharing will be implemented by increasing the reference count.

Due to this mechanism, multiple `Bytes` instances may point to the same shared memory region. Each `Bytes` instance can point to different sections within that memory region, and `Bytes` instances may or may not have overlapping views into the memory.

The following diagram visualizes a scenario where 2 `Bytes` instances make use of an `Arc`-based backing storage, and provide access to different views:

```
   Arc ptrs                   ┌─────────┐
   ________________________ / │ Bytes 2 │
  /                           └─────────┘
 /          ┌───────────┐     |         |
|_________/ │  Bytes 1  │     |         |
|           └───────────┘     |         |
|           |           | ___/ data     | tail
|      data |      tail |/              |
v           v           v               v
┌─────┬─────┬───────────┬───────────────┬─────┐
│ Arc │     │           │               │     │
└─────┴─────┴───────────┴───────────────┴─────┘
```

## Methods

### `impl Bytes`

#### `new`

```rust
pub const fn new() -> Bytes
```

Creates a new empty `Bytes`.

This will not allocate and the returned `Bytes` handle will be empty.

###### Examples

```rust
use bytes::Bytes;

let b = Bytes::new();
assert_eq!(&b[..], b"");
```

#### `from_static`

```rust
pub const fn from_static(bytes: &'static [u8]) -> Bytes
```

Creates a new `Bytes` from a static slice.

The returned `Bytes` will point directly to the static slice. There is no allocating or copying.

###### Examples

```rust
use bytes::Bytes;

let b = Bytes::from_static(b"hello");
assert_eq!(&b[..], b"hello");
```

#### `from_owner`

```rust
pub fn from_owner<T>(owner: T) -> Bytes where T: AsRef<[u8]> + Send + 'static,
```

Create [Bytes](struct.Bytes.md) with a buffer whose lifetime is controlled via an explicit owner.

A common use case is to zero-copy construct from mapped memory.

```rust
use bytes::Bytes;
use memmap2::Mmap;

let file = File::open("upload_bundle.tar.gz")?;
let mmap = unsafe { Mmap::map(&file) }?;
let b = Bytes::from_owner(mmap);
```

The `owner` will be transferred to the constructed [Bytes](struct.Bytes.md) object, which will ensure it is dropped once all remaining clones of the constructed object are dropped. The owner will then be responsible for dropping the specified region of memory as part of its [Drop](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html) implementation.

Note that converting [Bytes](struct.Bytes.md) constructed from an owner into a [BytesMut] will always create a deep copy of the buffer into newly allocated memory.

#### `len`

```rust
pub const fn len(&self) -> usize
```

Returns the number of bytes contained in this `Bytes`.

###### Examples

```rust
use bytes::Bytes;

let b = Bytes::from(&b"hello"[..]);
assert_eq!(b.len(), 5);
```

#### `is_empty`

```rust
pub const fn is_empty(&self) -> bool
```

Returns true if the `Bytes` has a length of 0.

###### Examples

```rust
use bytes::Bytes;

let b = Bytes::new();
assert!(b.is_empty());
```

#### `is_unique`

```rust
pub fn is_unique(&self) -> bool
```

Returns true if this is the only reference to the data and `Into<BytesMut>` would avoid cloning the underlying buffer.

Always returns false if the data is backed by a [static slice](struct.Bytes.md), or an [owner](struct.Bytes.md).

The result of this method may be invalidated immediately if another thread clones this value while this is being called. Ensure you have unique access to this value (`&mut Bytes`) first if you need to be certain the result is valid (i.e. for safety reasons).

###### Examples

```rust
use bytes::Bytes;

let a = Bytes::from(vec![1, 2, 3]);
assert!(a.is_unique());
let b = a.clone();
assert!(!a.is_unique());
```

#### `copy_from_slice`

```rust
pub fn copy_from_slice(data: &[u8]) -> Bytes
```

Creates `Bytes` instance from slice, by copying it.

#### `slice`

```rust
pub fn slice(&self, range: impl RangeBounds<usize>) -> Bytes
```

Returns a slice of self for the provided range.

This will increment the reference count for the underlying memory and return a new `Bytes` handle set to the slice.

This operation is `O(1)`.

###### Examples

```rust
use bytes::Bytes;

let a = Bytes::from(&b"hello world"[..]);
let b = a.slice(2..5);

assert_eq!(&b[..], b"llo");
```

###### Panics

Requires that `begin <= end` and `end <= self.len()`, otherwise slicing will panic.

#### `slice_ref`

```rust
pub fn slice_ref(&self, subset: &[u8]) -> Bytes
```

Returns a slice of self that is equivalent to the given `subset`.

When processing a `Bytes` buffer with other tools, one often gets a `&[u8]` which is in fact a slice of the `Bytes`, i.e. a subset of it. This function turns that `&[u8]` into another `Bytes`, as if one had called `self.slice()` with the offsets that correspond to `subset`.

This operation is `O(1)`.

###### Examples

```rust
use bytes::Bytes;

let bytes = Bytes::from(&b"012345678"[..]);
let as_slice = bytes.as_ref();
let subset = &as_slice[2..6];
let subslice = bytes.slice_ref(&subset);
assert_eq!(&subslice[..], b"2345");
```

###### Panics

Requires that the given `sub` slice is in fact contained within the `Bytes` buffer; otherwise this function will panic.

#### `split_off`

```rust
pub fn split_off(&mut self, at: usize) -> Bytes
```

Splits the bytes into two at the given index.

Afterwards `self` contains elements `[0, at)`, and the returned `Bytes` contains elements `[at, len)`. It’s guaranteed that the memory does not move, that is, the address of `self` does not change, and the address of the returned slice is `at` bytes after that.

This is an `O(1)` operation that just increases the reference count and sets a few indices.

###### Examples

```rust
use bytes::Bytes;

let mut a = Bytes::from(&b"hello world"[..]);
let b = a.split_off(5);

assert_eq!(&a[..], b"hello");
assert_eq!(&b[..], b" world");
```

###### Panics

Panics if `at > len`.

#### `split_to`

```rust
pub fn split_to(&mut self, at: usize) -> Bytes
```

Splits the bytes into two at the given index.

Afterwards `self` contains elements `[at, len)`, and the returned `Bytes` contains elements `[0, at)`.

This is an `O(1)` operation that just increases the reference count and sets a few indices.

###### Examples

```rust
use bytes::Bytes;

let mut a = Bytes::from(&b"hello world"[..]);
let b = a.split_to(5);

assert_eq!(&a[..], b" world");
assert_eq!(&b[..], b"hello");
```

###### Panics

Panics if `at > len`.

#### `truncate`

```rust
pub fn truncate(&mut self, len: usize)
```

Shortens the buffer, keeping the first `len` bytes and dropping the rest.

If `len` is greater than the buffer’s current length, this has no effect.

The [split_off](struct.Bytes.md) method can emulate `truncate`, but this causes the excess bytes to be returned instead of dropped.

###### Examples

```rust
use bytes::Bytes;

let mut buf = Bytes::from(&b"hello world"[..]);
buf.truncate(5);
assert_eq!(buf, b"hello"[..]);
```

#### `clear`

```rust
pub fn clear(&mut self)
```

Clears the buffer, removing all data.

###### Examples

```rust
use bytes::Bytes;

let mut buf = Bytes::from(&b"hello world"[..]);
buf.clear();
assert!(buf.is_empty());
```

#### `try_into_mut`

```rust
pub fn try_into_mut(self) -> Result<BytesMut, Bytes>
```

Try to convert self into `BytesMut`.

If `self` is unique for the entire original buffer, this will succeed and return a `BytesMut` with the contents of `self` without copying. If `self` is not unique for the entire original buffer, this will fail and return self.

This will also always fail if the buffer was constructed via either [from_owner](struct.Bytes.md) or [from_static](struct.Bytes.md).

###### Examples

```rust
use bytes::{Bytes, BytesMut};

let bytes = Bytes::from(b"hello".to_vec());
assert_eq!(bytes.try_into_mut(), Ok(BytesMut::from(&b"hello"[..])));
```

## Trait Implementations

### `impl AsRef<[u8]> for Bytes`

#### `as_ref`

```rust
fn as_ref(&self) -> &[u8] ⓘ
```

Converts this type into a shared reference of the (usually inferred) input type.

### `impl Borrow<[u8]> for Bytes`

#### `borrow`

```rust
fn borrow(&self) -> &[u8] ⓘ
```

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

### `impl Buf for Bytes`

#### `remaining`

```rust
fn remaining(&self) -> usize
```

Returns the number of bytes between the current position and the end of the buffer. Read more

#### `chunk`

```rust
fn chunk(&self) -> &[u8] ⓘ
```

Returns a slice starting at the current position and of length between 0 and `Buf::remaining()`. Note that this *can* return a shorter slice (this allows non-continuous internal representation). Read more

#### `advance`

```rust
fn advance(&mut self, cnt: usize)
```

Advance the internal cursor of the Buf Read more

#### `copy_to_bytes`

```rust
fn copy_to_bytes(&mut self, len: usize) -> Bytes
```

Consumes `len` bytes inside self and returns new instance of `Bytes` with this data. Read more

#### `chunks_vectored`

```rust
fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize
```

Fills `dst` with potentially multiple slices starting at `self`’s current position. Read more

#### `has_remaining`

```rust
fn has_remaining(&self) -> bool
```

Returns true if there are any more bytes to consume Read more

#### `copy_to_slice`

```rust
fn copy_to_slice(&mut self, dst: &mut [u8])
```

Copies bytes from `self` into `dst`. Read more

#### `get_u8`

```rust
fn get_u8(&mut self) -> u8
```

Gets an unsigned 8 bit integer from `self`. Read more

#### `get_i8`

```rust
fn get_i8(&mut self) -> i8
```

Gets a signed 8 bit integer from `self`. Read more

#### `get_u16`

```rust
fn get_u16(&mut self) -> u16
```

Gets an unsigned 16 bit integer from `self` in big-endian byte order. Read more

#### `get_u16_le`

```rust
fn get_u16_le(&mut self) -> u16
```

Gets an unsigned 16 bit integer from `self` in little-endian byte order. Read more

#### `get_u16_ne`

```rust
fn get_u16_ne(&mut self) -> u16
```

Gets an unsigned 16 bit integer from `self` in native-endian byte order. Read more

#### `get_i16`

```rust
fn get_i16(&mut self) -> i16
```

Gets a signed 16 bit integer from `self` in big-endian byte order. Read more

#### `get_i16_le`

```rust
fn get_i16_le(&mut self) -> i16
```

Gets a signed 16 bit integer from `self` in little-endian byte order. Read more

#### `get_i16_ne`

```rust
fn get_i16_ne(&mut self) -> i16
```

Gets a signed 16 bit integer from `self` in native-endian byte order. Read more

#### `get_u32`

```rust
fn get_u32(&mut self) -> u32
```

Gets an unsigned 32 bit integer from `self` in the big-endian byte order. Read more

#### `get_u32_le`

```rust
fn get_u32_le(&mut self) -> u32
```

Gets an unsigned 32 bit integer from `self` in the little-endian byte order. Read more

#### `get_u32_ne`

```rust
fn get_u32_ne(&mut self) -> u32
```

Gets an unsigned 32 bit integer from `self` in native-endian byte order. Read more

#### `get_i32`

```rust
fn get_i32(&mut self) -> i32
```

Gets a signed 32 bit integer from `self` in big-endian byte order. Read more

#### `get_i32_le`

```rust
fn get_i32_le(&mut self) -> i32
```

Gets a signed 32 bit integer from `self` in little-endian byte order. Read more

#### `get_i32_ne`

```rust
fn get_i32_ne(&mut self) -> i32
```

Gets a signed 32 bit integer from `self` in native-endian byte order. Read more

#### `get_u64`

```rust
fn get_u64(&mut self) -> u64
```

Gets an unsigned 64 bit integer from `self` in big-endian byte order. Read more

#### `get_u64_le`

```rust
fn get_u64_le(&mut self) -> u64
```

Gets an unsigned 64 bit integer from `self` in little-endian byte order. Read more

#### `get_u64_ne`

```rust
fn get_u64_ne(&mut self) -> u64
```

Gets an unsigned 64 bit integer from `self` in native-endian byte order. Read more

#### `get_i64`

```rust
fn get_i64(&mut self) -> i64
```

Gets a signed 64 bit integer from `self` in big-endian byte order. Read more

#### `get_i64_le`

```rust
fn get_i64_le(&mut self) -> i64
```

Gets a signed 64 bit integer from `self` in little-endian byte order. Read more

#### `get_i64_ne`

```rust
fn get_i64_ne(&mut self) -> i64
```

Gets a signed 64 bit integer from `self` in native-endian byte order. Read more

#### `get_u128`

```rust
fn get_u128(&mut self) -> u128
```

Gets an unsigned 128 bit integer from `self` in big-endian byte order. Read more

#### `get_u128_le`

```rust
fn get_u128_le(&mut self) -> u128
```

Gets an unsigned 128 bit integer from `self` in little-endian byte order. Read more

#### `get_u128_ne`

```rust
fn get_u128_ne(&mut self) -> u128
```

Gets an unsigned 128 bit integer from `self` in native-endian byte order. Read more

#### `get_i128`

```rust
fn get_i128(&mut self) -> i128
```

Gets a signed 128 bit integer from `self` in big-endian byte order. Read more

#### `get_i128_le`

```rust
fn get_i128_le(&mut self) -> i128
```

Gets a signed 128 bit integer from `self` in little-endian byte order. Read more

#### `get_i128_ne`

```rust
fn get_i128_ne(&mut self) -> i128
```

Gets a signed 128 bit integer from `self` in native-endian byte order. Read more

#### `get_uint`

```rust
fn get_uint(&mut self, nbytes: usize) -> u64
```

Gets an unsigned n-byte integer from `self` in big-endian byte order. Read more

#### `get_uint_le`

```rust
fn get_uint_le(&mut self, nbytes: usize) -> u64
```

Gets an unsigned n-byte integer from `self` in little-endian byte order. Read more

#### `get_uint_ne`

```rust
fn get_uint_ne(&mut self, nbytes: usize) -> u64
```

Gets an unsigned n-byte integer from `self` in native-endian byte order. Read more

#### `get_int`

```rust
fn get_int(&mut self, nbytes: usize) -> i64
```

Gets a signed n-byte integer from `self` in big-endian byte order. Read more

#### `get_int_le`

```rust
fn get_int_le(&mut self, nbytes: usize) -> i64
```

Gets a signed n-byte integer from `self` in little-endian byte order. Read more

#### `get_int_ne`

```rust
fn get_int_ne(&mut self, nbytes: usize) -> i64
```

Gets a signed n-byte integer from `self` in native-endian byte order. Read more

#### `get_f32`

```rust
fn get_f32(&mut self) -> f32
```

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in big-endian byte order. Read more

#### `get_f32_le`

```rust
fn get_f32_le(&mut self) -> f32
```

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in little-endian byte order. Read more

#### `get_f32_ne`

```rust
fn get_f32_ne(&mut self) -> f32
```

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in native-endian byte order. Read more

#### `get_f64`

```rust
fn get_f64(&mut self) -> f64
```

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in big-endian byte order. Read more

#### `get_f64_le`

```rust
fn get_f64_le(&mut self) -> f64
```

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in little-endian byte order. Read more

#### `get_f64_ne`

```rust
fn get_f64_ne(&mut self) -> f64
```

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in native-endian byte order. Read more

#### `try_copy_to_slice`

```rust
fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<(), TryGetError>
```

Copies bytes from `self` into `dst`. Read more

#### `try_get_u8`

```rust
fn try_get_u8(&mut self) -> Result<u8, TryGetError>
```

Gets an unsigned 8 bit integer from `self`. Read more

#### `try_get_i8`

```rust
fn try_get_i8(&mut self) -> Result<i8, TryGetError>
```

Gets a signed 8 bit integer from `self`. Read more

#### `try_get_u16`

```rust
fn try_get_u16(&mut self) -> Result<u16, TryGetError>
```

Gets an unsigned 16 bit integer from `self` in big-endian byte order. Read more

#### `try_get_u16_le`

```rust
fn try_get_u16_le(&mut self) -> Result<u16, TryGetError>
```

Gets an unsigned 16 bit integer from `self` in little-endian byte order. Read more

#### `try_get_u16_ne`

```rust
fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError>
```

Gets an unsigned 16 bit integer from `self` in native-endian byte order. Read more

#### `try_get_i16`

```rust
fn try_get_i16(&mut self) -> Result<i16, TryGetError>
```

Gets a signed 16 bit integer from `self` in big-endian byte order. Read more

#### `try_get_i16_le`

```rust
fn try_get_i16_le(&mut self) -> Result<i16, TryGetError>
```

Gets an signed 16 bit integer from `self` in little-endian byte order. Read more

#### `try_get_i16_ne`

```rust
fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError>
```

Gets a signed 16 bit integer from `self` in native-endian byte order. Read more

#### `try_get_u32`

```rust
fn try_get_u32(&mut self) -> Result<u32, TryGetError>
```

Gets an unsigned 32 bit integer from `self` in big-endian byte order. Read more

#### `try_get_u32_le`

```rust
fn try_get_u32_le(&mut self) -> Result<u32, TryGetError>
```

Gets an unsigned 32 bit integer from `self` in little-endian byte order. Read more

#### `try_get_u32_ne`

```rust
fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError>
```

Gets an unsigned 32 bit integer from `self` in native-endian byte order. Read more

#### `try_get_i32`

```rust
fn try_get_i32(&mut self) -> Result<i32, TryGetError>
```

Gets a signed 32 bit integer from `self` in big-endian byte order. Read more

#### `try_get_i32_le`

```rust
fn try_get_i32_le(&mut self) -> Result<i32, TryGetError>
```

Gets a signed 32 bit integer from `self` in little-endian byte order. Read more

#### `try_get_i32_ne`

```rust
fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError>
```

Gets a signed 32 bit integer from `self` in native-endian byte order. Read more

#### `try_get_u64`

```rust
fn try_get_u64(&mut self) -> Result<u64, TryGetError>
```

Gets an unsigned 64 bit integer from `self` in big-endian byte order. Read more

#### `try_get_u64_le`

```rust
fn try_get_u64_le(&mut self) -> Result<u64, TryGetError>
```

Gets an unsigned 64 bit integer from `self` in little-endian byte order. Read more

#### `try_get_u64_ne`

```rust
fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError>
```

Gets an unsigned 64 bit integer from `self` in native-endian byte order. Read more

#### `try_get_i64`

```rust
fn try_get_i64(&mut self) -> Result<i64, TryGetError>
```

Gets a signed 64 bit integer from `self` in big-endian byte order. Read more

#### `try_get_i64_le`

```rust
fn try_get_i64_le(&mut self) -> Result<i64, TryGetError>
```

Gets a signed 64 bit integer from `self` in little-endian byte order. Read more

#### `try_get_i64_ne`

```rust
fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError>
```

Gets a signed 64 bit integer from `self` in native-endian byte order. Read more

#### `try_get_u128`

```rust
fn try_get_u128(&mut self) -> Result<u128, TryGetError>
```

Gets an unsigned 128 bit integer from `self` in big-endian byte order. Read more

#### `try_get_u128_le`

```rust
fn try_get_u128_le(&mut self) -> Result<u128, TryGetError>
```

Gets an unsigned 128 bit integer from `self` in little-endian byte order. Read more

#### `try_get_u128_ne`

```rust
fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError>
```

Gets an unsigned 128 bit integer from `self` in native-endian byte order. Read more

#### `try_get_i128`

```rust
fn try_get_i128(&mut self) -> Result<i128, TryGetError>
```

Gets a signed 128 bit integer from `self` in big-endian byte order. Read more

#### `try_get_i128_le`

```rust
fn try_get_i128_le(&mut self) -> Result<i128, TryGetError>
```

Gets a signed 128 bit integer from `self` in little-endian byte order. Read more

#### `try_get_i128_ne`

```rust
fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError>
```

Gets a signed 128 bit integer from `self` in native-endian byte order. Read more

#### `try_get_uint`

```rust
fn try_get_uint(&mut self, nbytes: usize) -> Result<u64, TryGetError>
```

Gets an unsigned n-byte integer from `self` in big-endian byte order. Read more

#### `try_get_uint_le`

```rust
fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError>
```

Gets an unsigned n-byte integer from `self` in little-endian byte order. Read more

#### `try_get_uint_ne`

```rust
fn try_get_uint_ne(&mut self, nbytes: usize) -> Result<u64, TryGetError>
```

Gets an unsigned n-byte integer from `self` in native-endian byte order. Read more

#### `try_get_int`

```rust
fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError>
```

Gets a signed n-byte integer from `self` in big-endian byte order. Read more

#### `try_get_int_le`

```rust
fn try_get_int_le(&mut self, nbytes: usize) -> Result<i64, TryGetError>
```

Gets a signed n-byte integer from `self` in little-endian byte order. Read more

#### `try_get_int_ne`

```rust
fn try_get_int_ne(&mut self, nbytes: usize) -> Result<i64, TryGetError>
```

Gets a signed n-byte integer from `self` in native-endian byte order. Read more

#### `try_get_f32`

```rust
fn try_get_f32(&mut self) -> Result<f32, TryGetError>
```

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in big-endian byte order. Read more

#### `try_get_f32_le`

```rust
fn try_get_f32_le(&mut self) -> Result<f32, TryGetError>
```

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in little-endian byte order. Read more

#### `try_get_f32_ne`

```rust
fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError>
```

Gets an IEEE754 single-precision (4 bytes) floating point number from `self` in native-endian byte order. Read more

#### `try_get_f64`

```rust
fn try_get_f64(&mut self) -> Result<f64, TryGetError>
```

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in big-endian byte order. Read more

#### `try_get_f64_le`

```rust
fn try_get_f64_le(&mut self) -> Result<f64, TryGetError>
```

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in little-endian byte order. Read more

#### `try_get_f64_ne`

```rust
fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError>
```

Gets an IEEE754 double-precision (8 bytes) floating point number from `self` in native-endian byte order. Read more

#### `take`

```rust
fn take(self, limit: usize) -> Take<Self> where Self: Sized,
```

Creates an adaptor which will read at most `limit` bytes from `self`. Read more

#### `chain`

```rust
fn chain<U>(self, next: U) -> Chain<Self, U> where U: Buf, Self: Sized,
```

Creates an adaptor which will chain this buffer with another. Read more

#### `reader`

```rust
fn reader(self) -> Reader<Self> where Self: Sized,
```

Creates an adaptor which implements the `Read` trait for `self`. Read more

### `impl Clone for Bytes`

#### `clone`

```rust
fn clone(&self) -> Bytes
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Bytes`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Bytes`

#### `default`

```rust
fn default() -> Bytes
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Deref for Bytes`

#### `Target`

```rust
type Target = [u8]
```

The resulting type after dereferencing.

#### `deref`

```rust
fn deref(&self) -> &[u8] ⓘ
```

Dereferences the value.

### `impl Drop for Bytes`

#### `drop`

```rust
fn drop(&mut self)
```

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

### `impl From<&'static [u8]> for Bytes`

#### `from`

```rust
fn from(slice: &'static [u8]) -> Bytes
```

Converts to this type from the input type.

### `impl From<&'static str> for Bytes`

#### `from`

```rust
fn from(slice: &'static str) -> Bytes
```

Converts to this type from the input type.

### `impl From<Box<[u8]>> for Bytes`

#### `from`

```rust
fn from(slice: Box<[u8]>) -> Bytes
```

Converts to this type from the input type.

### `impl From<Bytes> for Vec<u8>`

#### `from`

```rust
fn from(bytes: Bytes) -> Vec<u8> ⓘ
```

Converts to this type from the input type.

### `impl From<BytesMut> for Bytes`

#### `from`

```rust
fn from(src: BytesMut) -> Bytes
```

Converts to this type from the input type.

### `impl From<Screenshot> for Bytes`

#### `from`

```rust
fn from(screenshot: Screenshot) -> Self
```

Converts to this type from the input type.

### `impl From<String> for Bytes`

#### `from`

```rust
fn from(s: String) -> Bytes
```

Converts to this type from the input type.

### `impl From<Vec<u8>> for Bytes`

#### `from`

```rust
fn from(vec: Vec<u8>) -> Bytes
```

Converts to this type from the input type.

### `impl FromIterator<u8> for Bytes`

#### `from_iter`

```rust
fn from_iter<T>(into_iter: T) -> Bytes where T: IntoIterator<Item = u8>,
```

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

### `impl Hash for Bytes`

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

### `impl<'a> IntoIterator for &'a Bytes`

#### `Item`

```rust
type Item = &'a u8
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = Iter<'a, u8>
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> <&'a Bytes as IntoIterator>::IntoIter
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

### `impl IntoIterator for Bytes`

#### `Item`

```rust
type Item = u8
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = IntoIter<Bytes>
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> <Bytes as IntoIterator>::IntoIter
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

### `impl LowerHex for Bytes`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html#tymethod.fmt)

### `impl Ord for Bytes`

#### `cmp`

```rust
fn cmp(&self, other: &Bytes) -> Ordering
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

### `impl<'a, T> PartialEq<&'a T> for Bytes where Bytes: PartialEq<T>, T: ?Sized,`

#### `eq`

```rust
fn eq(&self, other: &&'a T) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<[u8]> for Bytes`

#### `eq`

```rust
fn eq(&self, other: &[u8]) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<Bytes> for &[u8]`

#### `eq`

```rust
fn eq(&self, other: &Bytes) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<Bytes> for &str`

#### `eq`

```rust
fn eq(&self, other: &Bytes) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<Bytes> for [u8]`

#### `eq`

```rust
fn eq(&self, other: &Bytes) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<Bytes> for Vec<u8>`

#### `eq`

```rust
fn eq(&self, other: &Bytes) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<Bytes> for str`

#### `eq`

```rust
fn eq(&self, other: &Bytes) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<BytesMut> for Bytes`

#### `eq`

```rust
fn eq(&self, other: &BytesMut) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<String> for Bytes`

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

### `impl PartialEq<Vec<u8>> for Bytes`

#### `eq`

```rust
fn eq(&self, other: &Vec<u8>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<str> for Bytes`

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

### `impl PartialEq for Bytes`

#### `eq`

```rust
fn eq(&self, other: &Bytes) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<'a, T> PartialOrd<&'a T> for Bytes where Bytes: PartialOrd<T>, T: ?Sized,`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &&'a T) -> Option<Ordering>
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

### `impl PartialOrd<[u8]> for Bytes`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &[u8]) -> Option<Ordering>
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

### `impl PartialOrd<Bytes> for &[u8]`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Bytes) -> Option<Ordering>
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

### `impl PartialOrd<Bytes> for &str`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Bytes) -> Option<Ordering>
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

### `impl PartialOrd<Bytes> for [u8]`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Bytes) -> Option<Ordering>
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

### `impl PartialOrd<Bytes> for Vec<u8>`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Bytes) -> Option<Ordering>
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

### `impl PartialOrd<Bytes> for str`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Bytes) -> Option<Ordering>
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

### `impl PartialOrd<String> for Bytes`

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

### `impl PartialOrd<Vec<u8>> for Bytes`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Vec<u8>) -> Option<Ordering>
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

### `impl PartialOrd<str> for Bytes`

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

### `impl PartialOrd for Bytes`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Bytes) -> Option<Ordering>
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

### `impl UpperHex for Bytes`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html#tymethod.fmt)

### `impl Eq for Bytes`

### `impl Send for Bytes`

### `impl Sync for Bytes`

## Auto Trait Implementations

- `impl !Freeze for Bytes`
- `impl RefUnwindSafe for Bytes`
- `impl Unpin for Bytes`
- `impl UnsafeUnpin for Bytes`
- `impl UnwindSafe for Bytes`
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

## Methods from Deref<Target = [u8]>
