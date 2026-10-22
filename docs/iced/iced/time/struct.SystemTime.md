---
title: "SystemTime"
kind: struct
path: "iced::time::SystemTime"
crate: "iced"
module: "iced::time"
file: "iced/time/struct.SystemTime.md"
docs_url: "https://docs.iced.rs/iced/time/struct.SystemTime.html"
---

# `iced::time::SystemTime`

```rust
pub struct SystemTime(/* private fields */);
```

A measurement of the system clock, useful for talking to external entities like the file system or other processes.

Distinct from the [`Instant`](struct.Instant.md) type, this time measurement **is not monotonic**. This means that you can save a file to the file system, then save another file to the file system, **and the second file has a `SystemTime` measurement earlier than the first**. In other words, an operation that happens after another operation in real time may have an earlier `SystemTime`!

Consequently, comparing two `SystemTime` instances to learn about the duration between them returns a [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html) instead of an infallible [`Duration`](struct.Duration.md) to indicate that this sort of time drift may happen and needs to be handled.

Although a `SystemTime` cannot be directly inspected, the [`UNIX_EPOCH`](struct.SystemTime.md) constant is provided in this module as an anchor in time to learn information about a `SystemTime`. By calculating the duration from this fixed point in time, a `SystemTime` can be converted to a human-readable time, or perhaps some other string representation.

The size of a `SystemTime` struct may vary depending on the target operating system.

A `SystemTime` does not count leap seconds. `SystemTime::now()`’s behavior around a leap second is the same as the operating system’s wall clock. The precise behavior near a leap second (e.g. whether the clock appears to run slow or fast, or stop, or jump) depends on platform and configuration, so should not be relied on.

Example:

```rust
use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn main() {
   let now = SystemTime::now();

   // we sleep for 2 seconds
   sleep(Duration::new(2, 0));
   match now.elapsed() {
       Ok(elapsed) => {
           // it prints '2'
           println!("{}", elapsed.as_secs());
       }
       Err(e) => {
           // the system clock went backwards!
           println!("Great Scott! {e:?}");
       }
   }
}
```

## Platform-specific behavior

The precision of `SystemTime` can depend on the underlying OS-specific time format. For example, on Windows the time is represented in 100 nanosecond intervals whereas Linux can represent nanosecond intervals.

The following system calls are [currently](https://doc.rust-lang.org/nightly/std/io/index.html#platform-specific-behavior) being used by `now()` to find out the current time:

| Platform | System call |
| --- | --- |
| SGX | [`insecure_time` usercall](https://edp.fortanix.com/docs/api/fortanix_sgx_abi/struct.Usercalls.html#method.insecure_time). More information on [timekeeping in SGX](https://edp.fortanix.com/docs/concepts/rust-std/#codestdtimecode) |
| UNIX | [clock_gettime (Realtime Clock)](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) |
| Darwin | [clock_gettime (Realtime Clock)](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) |
| VXWorks | [clock_gettime (Realtime Clock)](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) |
| SOLID | `SOLID_RTC_ReadTime` |
| WASI | [__wasi_clock_time_get (Realtime Clock)](https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/docs.md#clock_time_get) |
| Windows | [GetSystemTimePreciseAsFileTime](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime) / [GetSystemTimeAsFileTime](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeasfiletime) |

**Disclaimer:** These system calls might change over time.

> Note: mathematical operations like [`add`](struct.SystemTime.md) may panic if the underlying structure cannot represent the new point in time.

**Source:** [time.rs.html#250](https://doc.rust-lang.org/nightly/src/std/time.rs.html#250) (lines 250)

## Methods

### `impl SystemTime`

#### `UNIX_EPOCH`

```rust
pub const UNIX_EPOCH: SystemTime = UNIX_EPOCH
```

An anchor in time which can be used to create new `SystemTime` instances or learn about where in time a `SystemTime` lies.

This constant is defined to be “1970-01-01 00:00:00 UTC” on all systems with respect to the system clock. Using `duration_since` on an existing `SystemTime` instance can tell how far away from this point in time a measurement lies, and using `UNIX_EPOCH + duration` can be used to create a `SystemTime` instance to represent another fixed point in time.

`duration_since(UNIX_EPOCH).unwrap().as_secs()` returns the number of non-leap seconds since the start of 1970 UTC. This is a POSIX `time_t` (as a `u64`), and is the same time representation as used in many Internet protocols.

###### Examples

```rust
use std::time::SystemTime;

match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
}
```

#### `MAX`

```rust
pub const MAX: SystemTime
```

*🔬This is a nightly-only experimental API. (time_systemtime_limits)*

Represents the maximum value representable by [`SystemTime`](struct.SystemTime.md) on this platform.

This value differs a lot between platforms, but it is always the case that any positive addition of a [`Duration`](struct.Duration.md), whose value is greater than or equal to the time precision of the operating system, to [`SystemTime::MAX`](struct.SystemTime.md) will fail.

###### Examples

```rust
#![feature(time_systemtime_limits)]
use std::time::{Duration, SystemTime};

// Adding zero will change nothing.
assert_eq!(SystemTime::MAX.checked_add(Duration::ZERO), Some(SystemTime::MAX));

// But adding just one second will already fail ...
//
// Keep in mind that this in fact may succeed, if the Duration is
// smaller than the time precision of the operating system, which
// happens to be 1ns on most operating systems, with Windows being the
// notable exception by using 100ns, hence why this example uses 1s.
assert_eq!(SystemTime::MAX.checked_add(Duration::new(1, 0)), None);

// Utilize this for saturating arithmetic to improve error handling.
// In this case, we will use a certificate with a timestamp in the
// future as a practical example.
let configured_offset = Duration::from_secs(60 * 60 * 24);
let valid_after =
    SystemTime::now()
        .checked_add(configured_offset)
        .unwrap_or(SystemTime::MAX);
```

#### `MIN`

```rust
pub const MIN: SystemTime
```

*🔬This is a nightly-only experimental API. (time_systemtime_limits)*

Represents the minimum value representable by [`SystemTime`](struct.SystemTime.md) on this platform.

This value differs a lot between platforms, but it is always the case that any positive subtraction of a [`Duration`](struct.Duration.md) from, whose value is greater than or equal to the time precision of the operating system, to [`SystemTime::MIN`](struct.SystemTime.md) will fail.

Depending on the platform, this may be either less than or equal to [`SystemTime::UNIX_EPOCH`](struct.SystemTime.md), depending on whether the operating system supports the representation of timestamps before the Unix epoch or not. However, it is always guaranteed that a [`SystemTime::UNIX_EPOCH`](struct.SystemTime.md) fits between a [`SystemTime::MIN`](struct.SystemTime.md) and [`SystemTime::MAX`](struct.SystemTime.md).

###### Examples

```rust
#![feature(time_systemtime_limits)]
use std::time::{Duration, SystemTime};

// Subtracting zero will change nothing.
assert_eq!(SystemTime::MIN.checked_sub(Duration::ZERO), Some(SystemTime::MIN));

// But subtracting just one second will already fail.
//
// Keep in mind that this in fact may succeed, if the Duration is
// smaller than the time precision of the operating system, which
// happens to be 1ns on most operating systems, with Windows being the
// notable exception by using 100ns, hence why this example uses 1s.
assert_eq!(SystemTime::MIN.checked_sub(Duration::new(1, 0)), None);

// Utilize this for saturating arithmetic to improve error handling.
// In this case, we will use a cache expiry as a practical example.
let configured_expiry = Duration::from_secs(60 * 3);
let expiry_threshold =
    SystemTime::now()
        .checked_sub(configured_expiry)
        .unwrap_or(SystemTime::MIN);
```

#### `now`

```rust
pub fn now() -> SystemTime
```

Returns the system time corresponding to “now”.

###### Examples

```rust
use std::time::SystemTime;

let sys_time = SystemTime::now();
```

#### `duration_since`

```rust
pub fn duration_since( &self, earlier: SystemTime, ) -> Result<Duration, SystemTimeError>
```

Returns the amount of time elapsed from an earlier point in time.

This function may fail because measurements taken earlier are not guaranteed to always be before later measurements (due to anomalies such as the system clock being adjusted either forwards or backwards). [`Instant`](struct.Instant.md) can be used to measure elapsed time without this risk of failure.

If successful, `Ok(Duration)` is returned where the duration represents the amount of time elapsed from the specified measurement to this one.

Returns an [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err) if `earlier` is later than `self`, and the error contains how far from `self` the time is.

###### Examples

```rust
use std::time::SystemTime;

let sys_time = SystemTime::now();
let new_sys_time = SystemTime::now();
let difference = new_sys_time.duration_since(sys_time)
    .expect("Clock may have gone backwards");
println!("{difference:?}");
```

#### `elapsed`

```rust
pub fn elapsed(&self) -> Result<Duration, SystemTimeError>
```

Returns the difference from this system time to the current clock time.

This function may fail as the underlying system clock is susceptible to drift and updates (e.g., the system clock could go backwards), so this function might not always succeed. If successful, `Ok(Duration)` is returned where the duration represents the amount of time elapsed from this time measurement to the current time.

To measure elapsed time reliably, use [`Instant`](struct.Instant.md) instead.

Returns an [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err) if `self` is later than the current system time, and the error contains how far from the current system time `self` is.

###### Examples

```rust
use std::thread::sleep;
use std::time::{Duration, SystemTime};

let sys_time = SystemTime::now();
let one_sec = Duration::from_secs(1);
sleep(one_sec);
assert!(sys_time.elapsed().unwrap() >= one_sec);
```

#### `checked_add`

```rust
pub fn checked_add(&self, duration: Duration) -> Option<SystemTime>
```

Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as `SystemTime` (which means it’s inside the bounds of the underlying data structure), `None` otherwise.

In the case that the `duration` is smaller than the time precision of the operating system, `Some(self)` will be returned.

#### `checked_sub`

```rust
pub fn checked_sub(&self, duration: Duration) -> Option<SystemTime>
```

Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as `SystemTime` (which means it’s inside the bounds of the underlying data structure), `None` otherwise.

In the case that the `duration` is smaller than the time precision of the operating system, `Some(self)` will be returned.

#### `saturating_add`

```rust
pub fn saturating_add(&self, duration: Duration) -> SystemTime
```

*🔬This is a nightly-only experimental API. (time_saturating_systemtime)*

Saturating [`SystemTime`](struct.SystemTime.md) addition, computing `self + duration`, returning [`SystemTime::MAX`](struct.SystemTime.md) if overflow occurred.

In the case that the `duration` is smaller than the time precision of the operating system, `self` will be returned.

#### `saturating_sub`

```rust
pub fn saturating_sub(&self, duration: Duration) -> SystemTime
```

*🔬This is a nightly-only experimental API. (time_saturating_systemtime)*

Saturating [`SystemTime`](struct.SystemTime.md) subtraction, computing `self - duration`, returning [`SystemTime::MIN`](struct.SystemTime.md) if overflow occurred.

In the case that the `duration` is smaller than the time precision of the operating system, `self` will be returned.

#### `saturating_duration_since`

```rust
pub fn saturating_duration_since(&self, earlier: SystemTime) -> Duration
```

*🔬This is a nightly-only experimental API. (time_saturating_systemtime)*

Saturating computation of time elapsed from an earlier point in time, returning [`Duration::ZERO`](struct.Duration.md) in the case that `earlier` is later or equal to `self`.

###### Examples

```rust
#![feature(time_saturating_systemtime)]
use std::time::{Duration, SystemTime};

let now = SystemTime::now();
let prev = now.saturating_sub(Duration::new(1, 0));

// now - prev should return non-zero.
assert_eq!(now.saturating_duration_since(prev), Duration::new(1, 0));
assert!(now.duration_since(prev).is_ok());

// prev - now should return zero (and fail with the non-saturating).
assert_eq!(prev.saturating_duration_since(now), Duration::ZERO);
assert!(prev.duration_since(now).is_err());

// now - now should return zero (and work with the non-saturating).
assert_eq!(now.saturating_duration_since(now), Duration::ZERO);
assert!(now.duration_since(now).is_ok());
```

## Trait Implementations

### `impl Add<Duration> for SystemTime`

#### `add`

```rust
fn add(self, dur: Duration) -> SystemTime
```

###### Panics

This function may panic if the resulting point in time cannot be represented by the underlying data structure. See [`SystemTime::checked_add`](struct.SystemTime.md) for a version without panic.

#### `Output`

```rust
type Output = SystemTime
```

The resulting type after applying the `+` operator.

### `impl AddAssign<Duration> for SystemTime`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Duration)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<'de, __Context> BorrowDecode<'de, __Context> for SystemTime`

#### `borrow_decode`

```rust
fn borrow_decode<D>(decoder: &mut D) -> Result<SystemTime, DecodeError> where D: BorrowDecoder<'de, Context = __Context>,
```

Attempt to decode this type with the given [BorrowDecode](https://docs.rs/bincode/2.0.1/bincode/de/trait.BorrowDecode.html).

### `impl Clone for SystemTime`

#### `clone`

```rust
fn clone(&self) -> SystemTime
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for SystemTime`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<Context> Decode<Context> for SystemTime`

#### `decode`

```rust
fn decode<D>(decoder: &mut D) -> Result<SystemTime, DecodeError> where D: Decoder<Context = Context>,
```

Attempt to decode this type with the given [Decode](https://docs.rs/bincode/2.0.1/bincode/de/trait.Decode.html).

### `impl<'de> Deserialize<'de> for SystemTime`

#### `deserialize`

```rust
fn deserialize<D>( deserializer: D, ) -> Result<SystemTime, <D as Deserializer<'de>>::Error> where D: Deserializer<'de>,
```

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

### `impl Encode for SystemTime`

#### `encode`

```rust
fn encode<E>(&self, encoder: &mut E) -> Result<(), EncodeError> where E: Encoder,
```

Encode a given type.

### `impl From<Timestamp> for SystemTime`

#### `from`

```rust
fn from(ts: Timestamp) -> SystemTime
```

Perform the conversion.

If the conversion would fail, an undefined `SystemTime` will be returned instead. This can happen if the `Timestamp` would overflow the max value allowed by `SystemTime` on the target platform. Use `TryFrom` to catch conversion failures and handle them explicitly.

### `impl Hash for SystemTime`

#### `hash`

```rust
fn hash<__H>(&self, state: &mut __H) where __H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl Ord for SystemTime`

#### `cmp`

```rust
fn cmp(&self, other: &SystemTime) -> Ordering
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

### `impl PartialEq for SystemTime`

#### `eq`

```rust
fn eq(&self, other: &SystemTime) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd for SystemTime`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &SystemTime) -> Option<Ordering>
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

### `impl Serialize for SystemTime`

#### `serialize`

```rust
fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer,
```

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/ser/trait.Serialize.html#tymethod.serialize)

### `impl Sub<Duration> for SystemTime`

#### `Output`

```rust
type Output = SystemTime
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, dur: Duration) -> SystemTime
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl SubAssign<Duration> for SystemTime`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Duration)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl Type for SystemTime`

#### `SIGNATURE`

```rust
const SIGNATURE: &'static Signature = <(u64, u32)>::SIGNATURE
```

The signature for the implementing type, in parsed format. Read more

### `impl Copy for SystemTime`

### `impl Eq for SystemTime`

### `impl StructuralPartialEq for SystemTime`

## Auto Trait Implementations

- `impl Freeze for SystemTime`
- `impl RefUnwindSafe for SystemTime`
- `impl Send for SystemTime`
- `impl Sync for SystemTime`
- `impl Unpin for SystemTime`
- `impl UnsafeUnpin for SystemTime`
- `impl UnwindSafe for SystemTime`
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

### `impl<'de, T> DynamicDeserialize<'de> for T where T: Type + Deserialize<'de>,`

#### `Deserializer`

```rust
type Deserializer = PhantomData<T>
```

A [DeserializeSeed](https://docs.rs/serde_core/1.0.229/serde_core/de/trait.DeserializeSeed.html) implementation for this type.

#### `deserializer_for_signature`

```rust
fn deserializer_for_signature( signature: &Signature, ) -> Result<<T as DynamicDeserialize<'de>>::Deserializer, Error>
```

Get a deserializer compatible with this parsed signature.

### `impl<T> DynamicType for T where T: Type + ?Sized,`

#### `signature`

```rust
fn signature(&self) -> Signature
```

The type signature for `self`. Read more

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

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Checks if this value is equivalent to the given key. Read more

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

### `impl<State, Message> IntoBoot<State, Message> for State`

#### `into_boot`

```rust
fn into_boot(self) -> (State, Task<Message>)
```

Turns some type into the initial state of some [`Application`](../application/struct.Application.md).

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

### `impl<T> Same for T`

#### `Output`

```rust
type Output = T
```

Should always be `Self`

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

### `impl<V, T> VZip<V> for T where V: MultiLane<T>,`

#### `vzip`

```rust
fn vzip(self) -> V
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

### `impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de>,`

### `impl<T> MaybeClone for T where T: Clone,`

### `impl<T> MaybeDebug for T where T: Debug,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
