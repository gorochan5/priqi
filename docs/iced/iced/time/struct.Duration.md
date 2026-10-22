---
title: "Duration"
kind: struct
path: "iced::time::Duration"
crate: "iced"
module: "iced::time"
file: "iced/time/struct.Duration.md"
docs_url: "https://docs.iced.rs/iced/time/struct.Duration.html"
---

# `iced::time::Duration`

```rust
pub struct Duration { /* private fields */ }
```

A `Duration` type to represent a span of time, typically used for system timeouts.

Each `Duration` is composed of a whole number of seconds and a fractional part represented in nanoseconds. If the underlying system does not support nanosecond-level precision, APIs binding a system timeout will typically round up the number of nanoseconds.

[`Duration`](struct.Duration.md)s implement many common traits, including [`Add`](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html), [`Sub`](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html), and other [`ops`](https://doc.rust-lang.org/nightly/core/ops/index.html) traits. It implements [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html) by returning a zero-length `Duration`.

## Examples

```rust
use std::time::Duration;

let five_seconds = Duration::new(5, 0);
let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 5);

assert_eq!(five_seconds_and_five_nanos.as_secs(), 5);
assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), 5);

let ten_millis = Duration::from_millis(10);
```

## Formatting `Duration` values

`Duration` intentionally does not have a `Display` impl, as there are a variety of ways to format spans of time for human readability. `Duration` provides a `Debug` impl that shows the full precision of the value.

The `Debug` output uses the non-ASCII “µs” suffix for microseconds. If your program output may appear in contexts that cannot rely on full Unicode compatibility, you may wish to format `Duration` objects yourself or use a crate to do so.

**Source:** [time.rs.html#81](https://doc.rust-lang.org/nightly/src/core/time.rs.html#81) (lines 81)

## Methods

### `impl Duration`

#### `SECOND`

```rust
pub const SECOND: Duration
```

*🔬This is a nightly-only experimental API. (duration_constants)*

The duration of one second.

###### Examples

```rust
#![feature(duration_constants)]
use std::time::Duration;

assert_eq!(Duration::SECOND, Duration::from_secs(1));
```

#### `MILLISECOND`

```rust
pub const MILLISECOND: Duration
```

*🔬This is a nightly-only experimental API. (duration_constants)*

The duration of one millisecond.

###### Examples

```rust
#![feature(duration_constants)]
use std::time::Duration;

assert_eq!(Duration::MILLISECOND, Duration::from_millis(1));
```

#### `MICROSECOND`

```rust
pub const MICROSECOND: Duration
```

*🔬This is a nightly-only experimental API. (duration_constants)*

The duration of one microsecond.

###### Examples

```rust
#![feature(duration_constants)]
use std::time::Duration;

assert_eq!(Duration::MICROSECOND, Duration::from_micros(1));
```

#### `NANOSECOND`

```rust
pub const NANOSECOND: Duration
```

*🔬This is a nightly-only experimental API. (duration_constants)*

The duration of one nanosecond.

###### Examples

```rust
#![feature(duration_constants)]
use std::time::Duration;

assert_eq!(Duration::NANOSECOND, Duration::from_nanos(1));
```

#### `ZERO`

```rust
pub const ZERO: Duration
```

A duration of zero time.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::ZERO;
assert!(duration.is_zero());
assert_eq!(duration.as_nanos(), 0);
```

#### `MAX`

```rust
pub const MAX: Duration
```

The maximum duration.

May vary by platform as necessary. Must be able to contain the difference between two instances of [`Instant`](https://doc.rust-lang.org/stable/std/time/struct.Instant.html) or two instances of [`SystemTime`](https://doc.rust-lang.org/stable/std/time/struct.SystemTime.html). This constraint gives it a value of about 584,942,417,355 years in practice, which is currently used on all platforms.

###### Examples

```rust
use std::time::Duration;

assert_eq!(Duration::MAX, Duration::new(u64::MAX, 1_000_000_000 - 1));
```

#### `new`

```rust
pub const fn new(secs: u64, nanos: u32) -> Duration
```

Creates a new `Duration` from the specified number of whole seconds and additional nanoseconds.

If the number of nanoseconds is greater than 1 billion (the number of nanoseconds in a second), then it will carry over into the seconds provided.

###### Panics

This constructor will panic if the carry from the nanoseconds overflows the seconds counter.

###### Examples

```rust
use std::time::Duration;

let five_seconds = Duration::new(5, 0);
```

#### `from_secs`

```rust
pub const fn from_secs(secs: u64) -> Duration
```

Creates a new `Duration` from the specified number of whole seconds.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::from_secs(5);

assert_eq!(5, duration.as_secs());
assert_eq!(0, duration.subsec_nanos());
```

#### `from_millis`

```rust
pub const fn from_millis(millis: u64) -> Duration
```

Creates a new `Duration` from the specified number of milliseconds.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::from_millis(2_569);

assert_eq!(2, duration.as_secs());
assert_eq!(569_000_000, duration.subsec_nanos());
```

#### `from_micros`

```rust
pub const fn from_micros(micros: u64) -> Duration
```

Creates a new `Duration` from the specified number of microseconds.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::from_micros(1_000_002);

assert_eq!(1, duration.as_secs());
assert_eq!(2_000, duration.subsec_nanos());
```

#### `from_nanos`

```rust
pub const fn from_nanos(nanos: u64) -> Duration
```

Creates a new `Duration` from the specified number of nanoseconds.

Note: Using this on the return value of `as_nanos()` might cause unexpected behavior: `as_nanos()` returns a u128, and can return values that do not fit in u64, e.g. 585 years. Instead, consider using the pattern `Duration::new(d.as_secs(), d.subsec_nanos())` if you cannot copy/clone the Duration directly.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::from_nanos(1_000_000_123);

assert_eq!(1, duration.as_secs());
assert_eq!(123, duration.subsec_nanos());
```

#### `from_nanos_u128`

```rust
pub const fn from_nanos_u128(nanos: u128) -> Duration
```

Creates a new `Duration` from the specified number of nanoseconds.

###### Panics

Panics if the given number of nanoseconds is greater than [`Duration::MAX`](struct.Duration.md).

###### Examples

```rust
use std::time::Duration;

let nanos = 10_u128.pow(24) + 321;
let duration = Duration::from_nanos_u128(nanos);

assert_eq!(10_u64.pow(15), duration.as_secs());
assert_eq!(321, duration.subsec_nanos());
```

#### `from_weeks`

```rust
pub const fn from_weeks(weeks: u64) -> Duration
```

*🔬This is a nightly-only experimental API. (duration_constructors)*

Creates a new `Duration` from the specified number of weeks.

###### Panics

Panics if the given number of weeks overflows the `Duration` size.

###### Examples

```rust
#![feature(duration_constructors)]
use std::time::Duration;

let duration = Duration::from_weeks(4);

assert_eq!(4 * 7 * 24 * 60 * 60, duration.as_secs());
assert_eq!(0, duration.subsec_nanos());
```

#### `from_days`

```rust
pub const fn from_days(days: u64) -> Duration
```

*🔬This is a nightly-only experimental API. (duration_constructors)*

Creates a new `Duration` from the specified number of days.

###### Panics

Panics if the given number of days overflows the `Duration` size.

###### Examples

```rust
#![feature(duration_constructors)]
use std::time::Duration;

let duration = Duration::from_days(7);

assert_eq!(7 * 24 * 60 * 60, duration.as_secs());
assert_eq!(0, duration.subsec_nanos());
```

#### `from_hours`

```rust
pub const fn from_hours(hours: u64) -> Duration
```

Creates a new `Duration` from the specified number of hours.

###### Panics

Panics if the given number of hours overflows the `Duration` size.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::from_hours(6);

assert_eq!(6 * 60 * 60, duration.as_secs());
assert_eq!(0, duration.subsec_nanos());
```

#### `from_mins`

```rust
pub const fn from_mins(mins: u64) -> Duration
```

Creates a new `Duration` from the specified number of minutes.

###### Panics

Panics if the given number of minutes overflows the `Duration` size.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::from_mins(10);

assert_eq!(10 * 60, duration.as_secs());
assert_eq!(0, duration.subsec_nanos());
```

#### `is_zero`

```rust
pub const fn is_zero(&self) -> bool
```

Returns true if this `Duration` spans no time.

###### Examples

```rust
use std::time::Duration;

assert!(Duration::ZERO.is_zero());
assert!(Duration::new(0, 0).is_zero());
assert!(Duration::from_nanos(0).is_zero());
assert!(Duration::from_secs(0).is_zero());

assert!(!Duration::new(1, 1).is_zero());
assert!(!Duration::from_nanos(1).is_zero());
assert!(!Duration::from_secs(1).is_zero());
```

#### `as_secs`

```rust
pub const fn as_secs(&self) -> u64
```

Returns the number of *whole* seconds contained by this `Duration`.

The returned value does not include the fractional (nanosecond) part of the duration, which can be obtained using [`subsec_nanos`](struct.Duration.md).

###### Examples

```rust
use std::time::Duration;

let duration = Duration::new(5, 730_023_852);
assert_eq!(duration.as_secs(), 5);
```

To determine the total number of seconds represented by the `Duration` including the fractional part, use [`as_secs_f64`](struct.Duration.md) or [`as_secs_f32`](struct.Duration.md)

#### `subsec_millis`

```rust
pub const fn subsec_millis(&self) -> u32
```

Returns the fractional part of this `Duration`, in whole milliseconds.

This method does **not** return the length of the duration when represented by milliseconds. The returned number always represents a fractional portion of a second (i.e., it is less than one thousand).

###### Examples

```rust
use std::time::Duration;

let duration = Duration::from_millis(5_432);
assert_eq!(duration.as_secs(), 5);
assert_eq!(duration.subsec_millis(), 432);
```

#### `subsec_micros`

```rust
pub const fn subsec_micros(&self) -> u32
```

Returns the fractional part of this `Duration`, in whole microseconds.

This method does **not** return the length of the duration when represented by microseconds. The returned number always represents a fractional portion of a second (i.e., it is less than one million).

###### Examples

```rust
use std::time::Duration;

let duration = Duration::from_micros(1_234_567);
assert_eq!(duration.as_secs(), 1);
assert_eq!(duration.subsec_micros(), 234_567);
```

#### `subsec_nanos`

```rust
pub const fn subsec_nanos(&self) -> u32
```

Returns the fractional part of this `Duration`, in nanoseconds.

This method does **not** return the length of the duration when represented by nanoseconds. The returned number always represents a fractional portion of a second (i.e., it is less than one billion).

###### Examples

```rust
use std::time::Duration;

let duration = Duration::from_millis(5_010);
assert_eq!(duration.as_secs(), 5);
assert_eq!(duration.subsec_nanos(), 10_000_000);
```

#### `as_millis`

```rust
pub const fn as_millis(&self) -> u128
```

Returns the total number of whole milliseconds contained by this `Duration`.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::new(5, 730_023_852);
assert_eq!(duration.as_millis(), 5_730);
```

#### `as_micros`

```rust
pub const fn as_micros(&self) -> u128
```

Returns the total number of whole microseconds contained by this `Duration`.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::new(5, 730_023_852);
assert_eq!(duration.as_micros(), 5_730_023);
```

#### `as_nanos`

```rust
pub const fn as_nanos(&self) -> u128
```

Returns the total number of nanoseconds contained by this `Duration`.

###### Examples

```rust
use std::time::Duration;

let duration = Duration::new(5, 730_023_852);
assert_eq!(duration.as_nanos(), 5_730_023_852);
```

#### `abs_diff`

```rust
pub const fn abs_diff(self, other: Duration) -> Duration
```

Computes the absolute difference between `self` and `other`.

###### Examples

```rust
use std::time::Duration;

assert_eq!(Duration::new(100, 0).abs_diff(Duration::new(80, 0)), Duration::new(20, 0));
assert_eq!(Duration::new(100, 400_000_000).abs_diff(Duration::new(110, 0)), Duration::new(9, 600_000_000));
```

#### `checked_add`

```rust
pub const fn checked_add(self, rhs: Duration) -> Option<Duration>
```

Checked `Duration` addition. Computes `self + other`, returning [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) if overflow occurred.

###### Examples

```rust
use std::time::Duration;

assert_eq!(Duration::new(0, 0).checked_add(Duration::new(0, 1)), Some(Duration::new(0, 1)));
assert_eq!(Duration::new(1, 0).checked_add(Duration::new(u64::MAX, 0)), None);
```

#### `saturating_add`

```rust
pub const fn saturating_add(self, rhs: Duration) -> Duration
```

Saturating `Duration` addition. Computes `self + other`, returning [`Duration::MAX`](struct.Duration.md) if overflow occurred.

###### Examples

```rust
use std::time::Duration;

assert_eq!(Duration::new(0, 0).saturating_add(Duration::new(0, 1)), Duration::new(0, 1));
assert_eq!(Duration::new(1, 0).saturating_add(Duration::new(u64::MAX, 0)), Duration::MAX);
```

#### `checked_sub`

```rust
pub const fn checked_sub(self, rhs: Duration) -> Option<Duration>
```

Checked `Duration` subtraction. Computes `self - other`, returning [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) if the result would be negative or if overflow occurred.

###### Examples

```rust
use std::time::Duration;

assert_eq!(Duration::new(0, 1).checked_sub(Duration::new(0, 0)), Some(Duration::new(0, 1)));
assert_eq!(Duration::new(0, 0).checked_sub(Duration::new(0, 1)), None);
```

#### `saturating_sub`

```rust
pub const fn saturating_sub(self, rhs: Duration) -> Duration
```

Saturating `Duration` subtraction. Computes `self - other`, returning [`Duration::ZERO`](struct.Duration.md) if the result would be negative or if overflow occurred.

###### Examples

```rust
use std::time::Duration;

assert_eq!(Duration::new(0, 1).saturating_sub(Duration::new(0, 0)), Duration::new(0, 1));
assert_eq!(Duration::new(0, 0).saturating_sub(Duration::new(0, 1)), Duration::ZERO);
```

#### `checked_mul`

```rust
pub const fn checked_mul(self, rhs: u32) -> Option<Duration>
```

Checked `Duration` multiplication. Computes `self * other`, returning [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) if overflow occurred.

###### Examples

```rust
use std::time::Duration;

assert_eq!(Duration::new(0, 500_000_001).checked_mul(2), Some(Duration::new(1, 2)));
assert_eq!(Duration::new(u64::MAX - 1, 0).checked_mul(2), None);
```

#### `saturating_mul`

```rust
pub const fn saturating_mul(self, rhs: u32) -> Duration
```

Saturating `Duration` multiplication. Computes `self * other`, returning [`Duration::MAX`](struct.Duration.md) if overflow occurred.

###### Examples

```rust
use std::time::Duration;

assert_eq!(Duration::new(0, 500_000_001).saturating_mul(2), Duration::new(1, 2));
assert_eq!(Duration::new(u64::MAX - 1, 0).saturating_mul(2), Duration::MAX);
```

#### `checked_div`

```rust
pub const fn checked_div(self, rhs: u32) -> Option<Duration>
```

Checked `Duration` division. Computes `self / other`, returning [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) if `other == 0`.

###### Examples

```rust
use std::time::Duration;

assert_eq!(Duration::new(2, 0).checked_div(2), Some(Duration::new(1, 0)));
assert_eq!(Duration::new(1, 0).checked_div(2), Some(Duration::new(0, 500_000_000)));
assert_eq!(Duration::new(2, 0).checked_div(0), None);
```

#### `as_secs_f64`

```rust
pub const fn as_secs_f64(&self) -> f64
```

Returns the number of seconds contained by this `Duration` as `f64`.

The returned value includes the fractional (nanosecond) part of the duration.

###### Examples

```rust
use std::time::Duration;

let dur = Duration::new(2, 700_000_000);
assert_eq!(dur.as_secs_f64(), 2.7);
```

#### `as_secs_f32`

```rust
pub const fn as_secs_f32(&self) -> f32
```

Returns the number of seconds contained by this `Duration` as `f32`.

The returned value includes the fractional (nanosecond) part of the duration.

###### Examples

```rust
use std::time::Duration;

let dur = Duration::new(2, 700_000_000);
assert_eq!(dur.as_secs_f32(), 2.7);
```

#### `as_millis_f64`

```rust
pub const fn as_millis_f64(&self) -> f64
```

*🔬This is a nightly-only experimental API. (duration_millis_float)*

Returns the number of milliseconds contained by this `Duration` as `f64`.

The returned value includes the fractional (nanosecond) part of the duration.

###### Examples

```rust
#![feature(duration_millis_float)]
use std::time::Duration;

let dur = Duration::new(2, 345_678_000);
assert_eq!(dur.as_millis_f64(), 2_345.678);
```

#### `as_millis_f32`

```rust
pub const fn as_millis_f32(&self) -> f32
```

*🔬This is a nightly-only experimental API. (duration_millis_float)*

Returns the number of milliseconds contained by this `Duration` as `f32`.

The returned value includes the fractional (nanosecond) part of the duration.

###### Examples

```rust
#![feature(duration_millis_float)]
use std::time::Duration;

let dur = Duration::new(2, 345_678_000);
assert_eq!(dur.as_millis_f32(), 2_345.678);
```

#### `from_secs_f64`

```rust
pub fn from_secs_f64(secs: f64) -> Duration
```

Creates a new `Duration` from the specified number of seconds represented as `f64`.

###### Panics

This constructor will panic if `secs` is negative, overflows `Duration` or not finite.

###### Examples

```rust
use std::time::Duration;

let res = Duration::from_secs_f64(0.0);
assert_eq!(res, Duration::new(0, 0));
let res = Duration::from_secs_f64(1e-20);
assert_eq!(res, Duration::new(0, 0));
let res = Duration::from_secs_f64(4.2e-7);
assert_eq!(res, Duration::new(0, 420));
let res = Duration::from_secs_f64(2.7);
assert_eq!(res, Duration::new(2, 700_000_000));
let res = Duration::from_secs_f64(3e10);
assert_eq!(res, Duration::new(30_000_000_000, 0));
// subnormal float
let res = Duration::from_secs_f64(f64::from_bits(1));
assert_eq!(res, Duration::new(0, 0));
// conversion uses rounding
let res = Duration::from_secs_f64(0.999e-9);
assert_eq!(res, Duration::new(0, 1));
```

#### `from_secs_f32`

```rust
pub fn from_secs_f32(secs: f32) -> Duration
```

Creates a new `Duration` from the specified number of seconds represented as `f32`.

###### Panics

This constructor will panic if `secs` is negative, overflows `Duration` or not finite.

###### Examples

```rust
use std::time::Duration;

let res = Duration::from_secs_f32(0.0);
assert_eq!(res, Duration::new(0, 0));
let res = Duration::from_secs_f32(1e-20);
assert_eq!(res, Duration::new(0, 0));
let res = Duration::from_secs_f32(4.2e-7);
assert_eq!(res, Duration::new(0, 420));
let res = Duration::from_secs_f32(2.7);
assert_eq!(res, Duration::new(2, 700_000_048));
let res = Duration::from_secs_f32(3e10);
assert_eq!(res, Duration::new(30_000_001_024, 0));
// subnormal float
let res = Duration::from_secs_f32(f32::from_bits(1));
assert_eq!(res, Duration::new(0, 0));
// conversion uses rounding
let res = Duration::from_secs_f32(0.999e-9);
assert_eq!(res, Duration::new(0, 1));
```

#### `mul_f64`

```rust
pub fn mul_f64(self, rhs: f64) -> Duration
```

Multiplies `Duration` by `f64`.

###### Panics

This method will panic if result is negative, overflows `Duration` or not finite.

###### Examples

```rust
use std::time::Duration;

let dur = Duration::new(2, 700_000_000);
assert_eq!(dur.mul_f64(3.14), Duration::new(8, 478_000_000));
assert_eq!(dur.mul_f64(3.14e5), Duration::new(847_800, 0));
```

#### `mul_f32`

```rust
pub fn mul_f32(self, rhs: f32) -> Duration
```

Multiplies `Duration` by `f32`.

###### Panics

This method will panic if result is negative, overflows `Duration` or not finite.

###### Examples

```rust
use std::time::Duration;

let dur = Duration::new(2, 700_000_000);
assert_eq!(dur.mul_f32(3.14), Duration::new(8, 478_000_641));
assert_eq!(dur.mul_f32(3.14e5), Duration::new(847_800, 0));
```

#### `div_f64`

```rust
pub fn div_f64(self, rhs: f64) -> Duration
```

Divides `Duration` by `f64`.

###### Panics

This method will panic if result is negative, overflows `Duration` or not finite.

###### Examples

```rust
use std::time::Duration;

let dur = Duration::new(2, 700_000_000);
assert_eq!(dur.div_f64(3.14), Duration::new(0, 859_872_611));
assert_eq!(dur.div_f64(3.14e5), Duration::new(0, 8_599));
```

#### `div_f32`

```rust
pub fn div_f32(self, rhs: f32) -> Duration
```

Divides `Duration` by `f32`.

###### Panics

This method will panic if result is negative, overflows `Duration` or not finite.

###### Examples

```rust
use std::time::Duration;

let dur = Duration::new(2, 700_000_000);
// note that due to rounding errors result is slightly
// different from 0.859_872_611
assert_eq!(dur.div_f32(3.14), Duration::new(0, 859_872_580));
assert_eq!(dur.div_f32(3.14e5), Duration::new(0, 8_599));
```

#### `div_duration_f64`

```rust
pub const fn div_duration_f64(self, rhs: Duration) -> f64
```

Divides `Duration` by `Duration` and returns `f64`.

###### Examples

```rust
use std::time::Duration;

let dur1 = Duration::new(2, 700_000_000);
let dur2 = Duration::new(5, 400_000_000);
assert_eq!(dur1.div_duration_f64(dur2), 0.5);
```

#### `div_duration_f32`

```rust
pub const fn div_duration_f32(self, rhs: Duration) -> f32
```

Divides `Duration` by `Duration` and returns `f32`.

###### Examples

```rust
use std::time::Duration;

let dur1 = Duration::new(2, 700_000_000);
let dur2 = Duration::new(5, 400_000_000);
assert_eq!(dur1.div_duration_f32(dur2), 0.5);
```

#### `div_duration_floor`

```rust
pub const fn div_duration_floor(self, rhs: Duration) -> u128
```

*🔬This is a nightly-only experimental API. (duration_integer_division)*

Divides `Duration` by `Duration` and returns `u128`, rounding the result towards zero.

###### Examples

```rust
#![feature(duration_integer_division)]
use std::time::Duration;

let dur = Duration::new(2, 0);
assert_eq!(dur.div_duration_floor(Duration::new(1, 000_000_001)), 1);
assert_eq!(dur.div_duration_floor(Duration::new(1, 000_000_000)), 2);
assert_eq!(dur.div_duration_floor(Duration::new(0, 999_999_999)), 2);
```

#### `div_duration_ceil`

```rust
pub const fn div_duration_ceil(self, rhs: Duration) -> u128
```

*🔬This is a nightly-only experimental API. (duration_integer_division)*

Divides `Duration` by `Duration` and returns `u128`, rounding the result towards positive infinity.

###### Examples

```rust
#![feature(duration_integer_division)]
use std::time::Duration;

let dur = Duration::new(2, 0);
assert_eq!(dur.div_duration_ceil(Duration::new(1, 000_000_001)), 2);
assert_eq!(dur.div_duration_ceil(Duration::new(1, 000_000_000)), 2);
assert_eq!(dur.div_duration_ceil(Duration::new(0, 999_999_999)), 3);
```

### `impl Duration`

#### `try_from_secs_f32`

```rust
pub fn try_from_secs_f32(secs: f32) -> Result<Duration, TryFromFloatSecsError>
```

The checked version of [`from_secs_f32`](struct.Duration.md).

This constructor will return an `Err` if `secs` is negative, overflows `Duration` or not finite.

###### Examples

```rust
use std::time::Duration;

let res = Duration::try_from_secs_f32(0.0);
assert_eq!(res, Ok(Duration::new(0, 0)));
let res = Duration::try_from_secs_f32(1e-20);
assert_eq!(res, Ok(Duration::new(0, 0)));
let res = Duration::try_from_secs_f32(4.2e-7);
assert_eq!(res, Ok(Duration::new(0, 420)));
let res = Duration::try_from_secs_f32(2.7);
assert_eq!(res, Ok(Duration::new(2, 700_000_048)));
let res = Duration::try_from_secs_f32(3e10);
assert_eq!(res, Ok(Duration::new(30_000_001_024, 0)));
// subnormal float:
let res = Duration::try_from_secs_f32(f32::from_bits(1));
assert_eq!(res, Ok(Duration::new(0, 0)));

let res = Duration::try_from_secs_f32(-5.0);
assert!(res.is_err());
let res = Duration::try_from_secs_f32(f32::NAN);
assert!(res.is_err());
let res = Duration::try_from_secs_f32(2e19);
assert!(res.is_err());

// the conversion uses rounding with tie resolution to even
let res = Duration::try_from_secs_f32(0.999e-9);
assert_eq!(res, Ok(Duration::new(0, 1)));

// this float represents exactly 976562.5e-9
let val = f32::from_bits(0x3A80_0000);
let res = Duration::try_from_secs_f32(val);
assert_eq!(res, Ok(Duration::new(0, 976_562)));

// this float represents exactly 2929687.5e-9
let val = f32::from_bits(0x3B40_0000);
let res = Duration::try_from_secs_f32(val);
assert_eq!(res, Ok(Duration::new(0, 2_929_688)));

// this float represents exactly 1.000_976_562_5
let val = f32::from_bits(0x3F802000);
let res = Duration::try_from_secs_f32(val);
assert_eq!(res, Ok(Duration::new(1, 976_562)));

// this float represents exactly 1.002_929_687_5
let val = f32::from_bits(0x3F806000);
let res = Duration::try_from_secs_f32(val);
assert_eq!(res, Ok(Duration::new(1, 2_929_688)));
```

#### `try_from_secs_f64`

```rust
pub fn try_from_secs_f64(secs: f64) -> Result<Duration, TryFromFloatSecsError>
```

The checked version of [`from_secs_f64`](struct.Duration.md).

This constructor will return an `Err` if `secs` is negative, overflows `Duration` or not finite.

###### Examples

```rust
use std::time::Duration;

let res = Duration::try_from_secs_f64(0.0);
assert_eq!(res, Ok(Duration::new(0, 0)));
let res = Duration::try_from_secs_f64(1e-20);
assert_eq!(res, Ok(Duration::new(0, 0)));
let res = Duration::try_from_secs_f64(4.2e-7);
assert_eq!(res, Ok(Duration::new(0, 420)));
let res = Duration::try_from_secs_f64(2.7);
assert_eq!(res, Ok(Duration::new(2, 700_000_000)));
let res = Duration::try_from_secs_f64(3e10);
assert_eq!(res, Ok(Duration::new(30_000_000_000, 0)));
// subnormal float
let res = Duration::try_from_secs_f64(f64::from_bits(1));
assert_eq!(res, Ok(Duration::new(0, 0)));

let res = Duration::try_from_secs_f64(-5.0);
assert!(res.is_err());
let res = Duration::try_from_secs_f64(f64::NAN);
assert!(res.is_err());
let res = Duration::try_from_secs_f64(2e19);
assert!(res.is_err());

// the conversion uses rounding with tie resolution to even
let res = Duration::try_from_secs_f64(0.999e-9);
assert_eq!(res, Ok(Duration::new(0, 1)));
let res = Duration::try_from_secs_f64(0.999_999_999_499);
assert_eq!(res, Ok(Duration::new(0, 999_999_999)));
let res = Duration::try_from_secs_f64(0.999_999_999_501);
assert_eq!(res, Ok(Duration::new(1, 0)));
let res = Duration::try_from_secs_f64(42.999_999_999_499);
assert_eq!(res, Ok(Duration::new(42, 999_999_999)));
let res = Duration::try_from_secs_f64(42.999_999_999_501);
assert_eq!(res, Ok(Duration::new(43, 0)));

// this float represents exactly 976562.5e-9
let val = f64::from_bits(0x3F50_0000_0000_0000);
let res = Duration::try_from_secs_f64(val);
assert_eq!(res, Ok(Duration::new(0, 976_562)));

// this float represents exactly 2929687.5e-9
let val = f64::from_bits(0x3F68_0000_0000_0000);
let res = Duration::try_from_secs_f64(val);
assert_eq!(res, Ok(Duration::new(0, 2_929_688)));

// this float represents exactly 1.000_976_562_5
let val = f64::from_bits(0x3FF0_0400_0000_0000);
let res = Duration::try_from_secs_f64(val);
assert_eq!(res, Ok(Duration::new(1, 976_562)));

// this float represents exactly 1.002_929_687_5
let val = f64::from_bits(0x3_FF00_C000_0000_000);
let res = Duration::try_from_secs_f64(val);
assert_eq!(res, Ok(Duration::new(1, 2_929_688)));
```

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

### `impl Add for Duration`

#### `Output`

```rust
type Output = Duration
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, rhs: Duration) -> Duration
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl AddAssign<Duration> for Instant`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Duration)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl AddAssign<Duration> for SystemTime`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Duration)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl AddAssign for Duration`

#### `add_assign`

```rust
fn add_assign(&mut self, rhs: Duration)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<'de, __Context> BorrowDecode<'de, __Context> for Duration`

#### `borrow_decode`

```rust
fn borrow_decode<D>(decoder: &mut D) -> Result<Duration, DecodeError> where D: BorrowDecoder<'de, Context = __Context>,
```

Attempt to decode this type with the given [BorrowDecode](https://docs.rs/bincode/2.0.1/bincode/de/trait.BorrowDecode.html).

### `impl Clone for Duration`

#### `clone`

```rust
fn clone(&self) -> Duration
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Duration`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<Context> Decode<Context> for Duration`

#### `decode`

```rust
fn decode<D>(decoder: &mut D) -> Result<Duration, DecodeError> where D: Decoder<Context = Context>,
```

Attempt to decode this type with the given [Decode](https://docs.rs/bincode/2.0.1/bincode/de/trait.Decode.html).

### `impl Default for Duration`

#### `default`

```rust
fn default() -> Duration
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<'de> Deserialize<'de> for Duration`

#### `deserialize`

```rust
fn deserialize<D>( deserializer: D, ) -> Result<Duration, <D as Deserializer<'de>>::Error> where D: Deserializer<'de>,
```

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

### `impl Div<u32> for Duration`

#### `Output`

```rust
type Output = Duration
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, rhs: u32) -> Duration
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl DivAssign<u32> for Duration`

#### `div_assign`

```rust
fn div_assign(&mut self, rhs: u32)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl Encode for Duration`

#### `encode`

```rust
fn encode<E>(&self, encoder: &mut E) -> Result<(), EncodeError> where E: Encoder,
```

Encode a given type.

### `impl From<Delay> for Duration`

#### `from`

```rust
fn from(delay: Delay) -> Duration
```

Converts to this type from the input type.

### `impl Hash for Duration`

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

### `impl Mul<Duration> for u32`

#### `Output`

```rust
type Output = Duration
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, rhs: Duration) -> Duration
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl Mul<u32> for Duration`

#### `Output`

```rust
type Output = Duration
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, rhs: u32) -> Duration
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl MulAssign<u32> for Duration`

#### `mul_assign`

```rust
fn mul_assign(&mut self, rhs: u32)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl Ord for Duration`

#### `cmp`

```rust
fn cmp(&self, other: &Duration) -> Ordering
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

### `impl PartialEq for Duration`

#### `eq`

```rust
fn eq(&self, other: &Duration) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd for Duration`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Duration) -> Option<Ordering>
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

### `impl SampleUniform for Duration`

#### `Sampler`

```rust
type Sampler = UniformDuration
```

The `UniformSampler` implementation supporting type `X`.

### `impl Serialize for Duration`

#### `serialize`

```rust
fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer,
```

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/ser/trait.Serialize.html#tymethod.serialize)

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

### `impl Sub for Duration`

#### `Output`

```rust
type Output = Duration
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, rhs: Duration) -> Duration
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl SubAssign<Duration> for Instant`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Duration)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl SubAssign<Duration> for SystemTime`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Duration)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl SubAssign for Duration`

#### `sub_assign`

```rust
fn sub_assign(&mut self, rhs: Duration)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<'a> Sum<&'a Duration> for Duration`

#### `sum`

```rust
fn sum<I>(iter: I) -> Duration where I: Iterator<Item = &'a Duration>,
```

Takes an iterator and generates `Self` from the elements by “summing up” the items.

### `impl Sum for Duration`

#### `sum`

```rust
fn sum<I>(iter: I) -> Duration where I: Iterator<Item = Duration>,
```

Takes an iterator and generates `Self` from the elements by “summing up” the items.

### `impl TryFrom<Timespec> for Duration`

#### `Error`

```rust
type Error = TryFromIntError
```

The type returned in the event of a conversion error.

#### `try_from`

```rust
fn try_from( ts: Timespec, ) -> Result<Duration, <Duration as TryFrom<Timespec>>::Error>
```

Performs the conversion.

### `impl Type for Duration`

#### `SIGNATURE`

```rust
const SIGNATURE: &'static Signature = <(u64, u32)>::SIGNATURE
```

The signature for the implementing type, in parsed format. Read more

### `impl Copy for Duration`

### `impl Eq for Duration`

### `impl StructuralPartialEq for Duration`

## Auto Trait Implementations

- `impl Freeze for Duration`
- `impl RefUnwindSafe for Duration`
- `impl Send for Duration`
- `impl Sync for Duration`
- `impl Unpin for Duration`
- `impl UnsafeUnpin for Duration`
- `impl UnwindSafe for Duration`
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

### `impl<T> NoneValue for T where T: Default,`

#### `NoneType`

```rust
type NoneType = T
```

#### `null_value`

```rust
fn null_value() -> T
```

The none-equivalent value.

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

### `impl<T> Same for T`

#### `Output`

```rust
type Output = T
```

Should always be `Self`

### `impl<Borrowed> SampleBorrow<Borrowed> for Borrowed where Borrowed: SampleUniform,`

#### `borrow`

```rust
fn borrow(&self) -> &Borrowed
```

Immutably borrows from an owned value. See [`Borrow::borrow`](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

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

### `impl<M> Measure for M where M: Debug + PartialOrd + Add<Output = M> + Default + Clone,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
