---
title: "Instant"
kind: struct
path: "iced::time::Instant"
crate: "iced"
module: "iced::time"
file: "iced/time/struct.Instant.md"
docs_url: "https://docs.iced.rs/iced/time/struct.Instant.html"
---

# `iced::time::Instant`

```rust
pub struct Instant(/* private fields */);
```

A measurement of a monotonically nondecreasing clock. Opaque and useful only with [`Duration`](struct.Duration.md).

Instants are always guaranteed, barring [platform bugs](struct.Instant.md), to be no less than any previously measured instant when created, and are often useful for tasks such as measuring benchmarks or timing how long an operation takes.

Note, however, that instants are **not** guaranteed to be **steady**. In other words, each tick of the underlying clock might not be the same length (e.g. some seconds may be longer than others). An instant may jump forwards or experience time dilation (slow down or speed up), but it will never go backwards. As part of this non-guarantee it is also not specified whether system suspends count as elapsed time or not. The behavior varies across platforms and Rust versions.

Instants are opaque types that can only be compared to one another. There is no method to get “the number of seconds” from an instant. Instead, it only allows measuring the duration between two instants (or comparing two instants).

The size of an `Instant` struct may vary depending on the target operating system.

Example:

```rust
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
   let now = Instant::now();

   // we sleep for 2 seconds
   sleep(Duration::new(2, 0));
   // it prints '2'
   println!("{}", now.elapsed().as_secs());
}
```

## OS-specific behaviors

An `Instant` is a wrapper around system-specific types and it may behave differently depending on the underlying operating system. For example, the following snippet is fine on Linux but panics on macOS:

```rust
use std::time::{Instant, Duration};

let now = Instant::now();
let days_per_10_millennia = 365_2425;
let solar_seconds_per_day = 60 * 60 * 24;
let millennium_in_solar_seconds = 31_556_952_000;
assert_eq!(millennium_in_solar_seconds, days_per_10_millennia * solar_seconds_per_day / 10);

let duration = Duration::new(millennium_in_solar_seconds, 0);
println!("{:?}", now + duration);
```

For cross-platform code, you can comfortably use durations of up to around one hundred years.

## Underlying System calls

The following system calls are [currently](https://doc.rust-lang.org/nightly/std/io/index.html#platform-specific-behavior) being used by `now()` to find out the current time:

| Platform | System call |
| --- | --- |
| SGX | [`insecure_time` usercall](https://edp.fortanix.com/docs/api/fortanix_sgx_abi/struct.Usercalls.html#method.insecure_time). More information on [timekeeping in SGX](https://edp.fortanix.com/docs/concepts/rust-std/#codestdtimecode) |
| UNIX | [clock_gettime](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) with `CLOCK_MONOTONIC` |
| Darwin | [clock_gettime](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) with `CLOCK_UPTIME_RAW` |
| VXWorks | [clock_gettime](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) with `CLOCK_MONOTONIC` |
| SOLID | `get_tim` |
| WASI | [__wasi_clock_time_get](https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/docs.md#clock_time_get) with `monotonic` |
| Windows | [QueryPerformanceCounter](https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter) |

**Disclaimer:** These system calls might change over time.

> Note: mathematical operations like [`add`](struct.Instant.md) may panic if the underlying structure cannot represent the new point in time.

### Monotonicity

On all platforms `Instant` will try to use an OS API that guarantees monotonic behavior if available, which is the case for all [tier 1](https://doc.rust-lang.org/rustc/platform-support.html) platforms. In practice such guarantees are – under rare circumstances – broken by hardware, virtualization or operating system bugs. To work around these bugs and platforms not offering monotonic clocks [`duration_since`](struct.Instant.md), [`elapsed`](struct.Instant.md) and [`sub`](struct.Instant.md) saturate to zero. In older Rust versions this lead to a panic instead. [`checked_duration_since`](struct.Instant.md) can be used to detect and handle situations where monotonicity is violated, or `Instant`s are subtracted in the wrong order.

This workaround obscures programming errors where earlier and later instants are accidentally swapped. For this reason future Rust versions may reintroduce panics.

**Source:** [time.rs.html#157](https://doc.rust-lang.org/nightly/src/std/time.rs.html#157) (lines 157)

## Methods

### `impl Instant`

#### `now`

```rust
pub fn now() -> Instant
```

Returns an instant corresponding to “now”.

###### Examples

```rust
use std::time::Instant;

let now = Instant::now();
```

#### `duration_since`

```rust
pub fn duration_since(&self, earlier: Instant) -> Duration
```

Returns the amount of time elapsed from another instant to this one, or zero duration if that instant is later than this one.

###### Panics

Previous Rust versions panicked when `earlier` was later than `self`. Currently this method saturates. Future versions may reintroduce the panic in some circumstances. See [Monotonicity](struct.Instant.md).

###### Examples

```rust
use std::time::{Duration, Instant};
use std::thread::sleep;

let now = Instant::now();
sleep(Duration::new(1, 0));
let new_now = Instant::now();
println!("{:?}", new_now.duration_since(now));
println!("{:?}", now.duration_since(new_now)); // 0ns
```

#### `checked_duration_since`

```rust
pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration>
```

Returns the amount of time elapsed from another instant to this one, or None if that instant is later than this one.

Due to [monotonicity bugs](struct.Instant.md), even under correct logical ordering of the passed `Instant`s, this method can return `None`.

###### Examples

```rust
use std::time::{Duration, Instant};
use std::thread::sleep;

let now = Instant::now();
sleep(Duration::new(1, 0));
let new_now = Instant::now();
println!("{:?}", new_now.checked_duration_since(now));
println!("{:?}", now.checked_duration_since(new_now)); // None
```

#### `saturating_duration_since`

```rust
pub fn saturating_duration_since(&self, earlier: Instant) -> Duration
```

Returns the amount of time elapsed from another instant to this one, or zero duration if that instant is later than this one.

###### Examples

```rust
use std::time::{Duration, Instant};
use std::thread::sleep;

let now = Instant::now();
sleep(Duration::new(1, 0));
let new_now = Instant::now();
println!("{:?}", new_now.saturating_duration_since(now));
println!("{:?}", now.saturating_duration_since(new_now)); // 0ns
```

#### `elapsed`

```rust
pub fn elapsed(&self) -> Duration
```

Returns the amount of time elapsed since this instant.

###### Panics

Previous Rust versions panicked when the current time was earlier than self. Currently this method returns a Duration of zero in that case. Future versions may reintroduce the panic. See [Monotonicity](struct.Instant.md).

###### Examples

```rust
use std::thread::sleep;
use std::time::{Duration, Instant};

let instant = Instant::now();
let three_secs = Duration::from_secs(3);
sleep(three_secs);
assert!(instant.elapsed() >= three_secs);
```

#### `checked_add`

```rust
pub fn checked_add(&self, duration: Duration) -> Option<Instant>
```

Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as `Instant` (which means it’s inside the bounds of the underlying data structure), `None` otherwise.

#### `checked_sub`

```rust
pub fn checked_sub(&self, duration: Duration) -> Option<Instant>
```

Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as `Instant` (which means it’s inside the bounds of the underlying data structure), `None` otherwise.

## Trait Implementations

### `impl Add<Duration> for Instant`

#### `add`

```rust
fn add(self, other: Duration) -> Instant
```

###### Panics

This function may panic if the resulting point in time cannot be represented by the underlying data structure. See [`Instant::checked_add`](struct.Instant.md) for a version without panic.

#### `Output`

```rust
type Output = Instant
```

The resulting type after applying the `+` operator.

### `impl AddAssign<Duration> for Instant`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Duration)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl AnimationTime for Instant`

#### `elapsed_since`

```rust
fn elapsed_since(self, time: Instant) -> f32
```

### `impl Clone for Instant`

#### `clone`

```rust
fn clone(&self) -> Instant
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Instant`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<Instant> for Instant`

#### `from`

```rust
fn from(time: Instant) -> Instant
```

Converts to this type from the input type.

### `impl From<Instant> for RedrawRequest`

#### `from`

```rust
fn from(time: Instant) -> RedrawRequest
```

Converts to this type from the input type.

### `impl Hash for Instant`

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

### `impl Ord for Instant`

#### `cmp`

```rust
fn cmp(&self, other: &Instant) -> Ordering
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

### `impl PartialEq for Instant`

#### `eq`

```rust
fn eq(&self, other: &Instant) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd for Instant`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Instant) -> Option<Ordering>
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

### `impl Sub<Duration> for Instant`

#### `Output`

```rust
type Output = Instant
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, other: Duration) -> Instant
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl Sub for Instant`

#### `sub`

```rust
fn sub(self, other: Instant) -> Duration
```

Returns the amount of time elapsed from another instant to this one, or zero duration if that instant is later than this one.

###### Panics

Previous Rust versions panicked when `other` was later than `self`. Currently this method saturates. Future versions may reintroduce the panic in some circumstances. See [Monotonicity](struct.Instant.md).

#### `Output`

```rust
type Output = Duration
```

The resulting type after applying the `-` operator.

### `impl SubAssign<Duration> for Instant`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Duration)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl Copy for Instant`

### `impl Eq for Instant`

### `impl StructuralPartialEq for Instant`

## Auto Trait Implementations

- `impl Freeze for Instant`
- `impl RefUnwindSafe for Instant`
- `impl Send for Instant`
- `impl Sync for Instant`
- `impl Unpin for Instant`
- `impl UnsafeUnpin for Instant`
- `impl UnwindSafe for Instant`
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

### `impl<T> MaybeClone for T where T: Clone,`

### `impl<T> MaybeDebug for T where T: Debug,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
