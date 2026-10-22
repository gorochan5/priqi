---
title: "Animation"
kind: struct
path: "iced_core::animation::Animation"
crate: "iced_core"
module: "iced_core::animation"
file: "iced_core/animation/struct.Animation.md"
docs_url: "https://docs.iced.rs/iced_core/animation/struct.Animation.html"
source_file: "src/iced_core/animation.rs"
source_lines: "11-17"
---

# `iced_core::animation::Animation`

```rust
pub struct Animation<T> where
    T: Clone + Copy + PartialEq + Float, { /* private fields */ }
```

The animation of some particular state.

It tracks state changes and allows projecting interpolated values through time.

**Source:** [animation.rs](../../src/iced_core/animation.rs.md) (lines 11-17)

## Methods

### `impl<T> Animation<T> where T: Clone + Copy + PartialEq + Float,`

#### `new`

```rust
pub fn new(state: T) -> Self
```

Creates a new [`Animation`](struct.Animation.md) with the given initial state.

#### `easing`

```rust
pub fn easing(self, easing: Easing) -> Self
```

Sets the [`Easing`](enum.Easing.md) function of the [`Animation`](struct.Animation.md).

See the [Easing Functions Cheat Sheet](https://easings.net) for details!

#### `very_quick`

```rust
pub fn very_quick(self) -> Self
```

Sets the duration of the [`Animation`](struct.Animation.md) to 100ms.

#### `quick`

```rust
pub fn quick(self) -> Self
```

Sets the duration of the [`Animation`](struct.Animation.md) to 200ms.

#### `slow`

```rust
pub fn slow(self) -> Self
```

Sets the duration of the [`Animation`](struct.Animation.md) to 400ms.

#### `very_slow`

```rust
pub fn very_slow(self) -> Self
```

Sets the duration of the [`Animation`](struct.Animation.md) to 500ms.

#### `duration`

```rust
pub fn duration(self, duration: Duration) -> Self
```

Sets the duration of the [`Animation`](struct.Animation.md) to the given value.

#### `delay`

```rust
pub fn delay(self, duration: Duration) -> Self
```

Sets a delay for the [`Animation`](struct.Animation.md).

#### `repeat`

```rust
pub fn repeat(self, repetitions: u32) -> Self
```

Makes the [`Animation`](struct.Animation.md) repeat a given amount of times.

Providing 1 repetition plays the animation twice in total.

#### `repeat_forever`

```rust
pub fn repeat_forever(self) -> Self
```

Makes the [`Animation`](struct.Animation.md) repeat forever.

#### `auto_reverse`

```rust
pub fn auto_reverse(self) -> Self
```

Makes the [`Animation`](struct.Animation.md) automatically reverse when repeating.

#### `go`

```rust
pub fn go(self, new_state: T, at: Instant) -> Self
```

Transitions the [`Animation`](struct.Animation.md) from its current state to the given new state at the given time.

#### `go_mut`

```rust
pub fn go_mut(&mut self, new_state: T, at: Instant)
```

Transitions the [`Animation`](struct.Animation.md) from its current state to the given new state at the given time, by reference.

#### `is_animating`

```rust
pub fn is_animating(&self, at: Instant) -> bool
```

Returns true if the [`Animation`](struct.Animation.md) is currently in progress.

An [`Animation`](struct.Animation.md) is in progress when it is transitioning to a different state.

#### `interpolate_with`

```rust
pub fn interpolate_with<I>(&self, f: impl Fn(T) -> I, at: Instant) -> I where I: Interpolable,
```

Projects the [`Animation`](struct.Animation.md) into an interpolated value at the given [`Instant`](../time/struct.Instant.md); using the closure provided to calculate the different keyframes of interpolated values.

If the [`Animation`](struct.Animation.md) state is a `bool`, you can use the simpler [`interpolate`](struct.Animation.md) method.

#### `value`

```rust
pub fn value(&self) -> T
```

Retuns the current state of the [`Animation`](struct.Animation.md).

### `impl Animation<f32>`

#### `interpolate`

```rust
pub fn interpolate(&self, at: Instant) -> f32
```

Projects the [`Animation`](struct.Animation.md) into an interpolated value at the given [`Instant`](../time/struct.Instant.md).

### `impl Animation<bool>`

#### `interpolate`

```rust
pub fn interpolate<I>(&self, start: I, end: I, at: Instant) -> I where I: Interpolable + Clone,
```

Projects the [`Animation`](struct.Animation.md) into an interpolated value at the given [`Instant`](../time/struct.Instant.md); using the `start` and `end` values as the origin and destination keyframes.

#### `remaining`

```rust
pub fn remaining(&self, at: Instant) -> Duration
```

Returns the remaining [`Duration`](../time/struct.Duration.md) of the [`Animation`](struct.Animation.md).

## Trait Implementations

### `impl<T> Clone for Animation<T> where T: Clone + Copy + PartialEq + Float + Clone,`

#### `clone`

```rust
fn clone(&self) -> Animation<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T> Debug for Animation<T> where T: Clone + Copy + PartialEq + Float + Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl<T> Freeze for Animation<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for Animation<T> where T: RefUnwindSafe,`
- `impl<T> Send for Animation<T> where T: Send,`
- `impl<T> Sync for Animation<T> where T: Sync,`
- `impl<T> Unpin for Animation<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Animation<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for Animation<T> where T: UnwindSafe,`
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
