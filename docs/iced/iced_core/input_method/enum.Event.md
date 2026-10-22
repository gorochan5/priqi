---
title: "Event"
kind: enum
path: "iced_core::input_method::Event"
crate: "iced_core"
module: "iced_core::input_method"
file: "iced_core/input_method/enum.Event.md"
docs_url: "https://docs.iced.rs/iced_core/input_method/enum.Event.html"
source_file: "src/iced_core/input_method.rs"
source_lines: "157-190"
---

# `iced_core::input_method::Event`

```rust
pub enum Event {
    Opened,
    Preedit(String, Option<Range<usize>>),
    Commit(String),
    Closed,
}
```

Describes [input method](https://en.wikipedia.org/wiki/Input_method) events.

This is also called a “composition event”.

Most keypresses using a latin-like keyboard layout simply generate a [`keyboard::Event::KeyPressed`](../keyboard/enum.Event.md). However, one couldn’t possibly have a key for every single unicode character that the user might want to type. The solution operating systems employ is to allow the user to type these using *a sequence of keypresses* instead.

**Source:** [input_method.rs](../../src/iced_core/input_method.rs.md) (lines 157-190)

## Variants

### `Opened`

Notifies when the IME was opened.

After getting this event you could receive [`Preedit`](enum.Event.md) and [`Commit`](enum.Event.md) events. You should also start performing IME related requests like [`Shell::request_input_method`](../shell/struct.Shell.md).

### `Preedit(String, Option<Range<usize>>)`

Notifies when a new composing text should be set at the cursor position.

The value represents a pair of the preedit string and the cursor begin position and end position. When it’s `None`, the cursor should be hidden. When `String` is an empty string this indicates that preedit was cleared.

The cursor range is byte-wise indexed.

### `Commit(String)`

Notifies when text should be inserted into the editor widget.

Right before this event, an empty [`Self::Preedit`](enum.Event.md) event will be issued.

### `Closed`

Notifies when the IME was disabled.

After receiving this event you won’t get any more [`Preedit`](enum.Event.md) or [`Commit`](enum.Event.md) events until the next [`Opened`](enum.Event.md) event. You should also stop issuing IME related requests like [`Shell::request_input_method`](../shell/struct.Shell.md) and clear pending preedit text.

## Trait Implementations

### `impl Clone for Event`

#### `clone`

```rust
fn clone(&self) -> Event
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Event`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Hash for Event`

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

### `impl PartialEq for Event`

#### `eq`

```rust
fn eq(&self, other: &Event) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Eq for Event`

### `impl StructuralPartialEq for Event`

## Auto Trait Implementations

- `impl Freeze for Event`
- `impl RefUnwindSafe for Event`
- `impl Send for Event`
- `impl Sync for Event`
- `impl Unpin for Event`
- `impl UnsafeUnpin for Event`
- `impl UnwindSafe for Event`
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
