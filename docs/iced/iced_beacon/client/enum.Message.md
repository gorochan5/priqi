---
title: "Message"
kind: enum
path: "iced_beacon::client::Message"
crate: "iced_beacon"
module: "iced_beacon::client"
file: "iced_beacon/client/enum.Message.md"
docs_url: "https://docs.iced.rs/iced_beacon/client/enum.Message.html"
source_file: "src/iced_beacon/client.rs"
source_lines: "26-41"
---

# `iced_beacon::client::Message`

```rust
pub enum Message {
    Connected {
        at: SystemTime,
        name: String,
        version: Version,
        theme: Option<Seed>,
        can_time_travel: bool,
    },
    EventLogged {
        at: SystemTime,
        event: Event,
    },
    Quit {
        at: SystemTime,
    },
}
```

**Source:** [client.rs](../../src/iced_beacon/client.rs.md) (lines 26-41)

## Variants

### `Connected`

**Fields**

- `at: SystemTime`
- `name: String`
- `version: Version`
- `theme: Option<Seed>`
- `can_time_travel: bool`

### `EventLogged`

**Fields**

- `at: SystemTime`
- `event: Event`

### `Quit`

**Fields**

- `at: SystemTime`

## Trait Implementations

### `impl Clone for Message`

#### `clone`

```rust
fn clone(&self) -> Message
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Message`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<'de> Deserialize<'de> for Message`

#### `deserialize`

```rust
fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error> where __D: Deserializer<'de>,
```

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

### `impl Serialize for Message`

#### `serialize`

```rust
fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error> where __S: Serializer,
```

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/ser/trait.Serialize.html#tymethod.serialize)

## Auto Trait Implementations

- `impl Freeze for Message`
- `impl RefUnwindSafe for Message`
- `impl Send for Message`
- `impl Sync for Message`
- `impl Unpin for Message`
- `impl UnsafeUnpin for Message`
- `impl UnwindSafe for Message`
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

### `impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de>,`
