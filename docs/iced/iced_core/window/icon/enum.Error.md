---
title: "Error"
kind: enum
path: "iced_core::window::icon::Error"
crate: "iced_core"
module: "iced_core::window::icon"
file: "iced_core/window/icon/enum.Error.md"
docs_url: "https://docs.iced.rs/iced_core/window/icon/enum.Error.html"
source_file: "src/iced_core/window/icon.rs"
source_lines: "58-85"
---

# `iced_core::window::icon::Error`

```rust
pub enum Error {
    ByteCountNotDivisibleBy4 {
        byte_count: usize,
    },
    DimensionsVsPixelCount {
        width: u32,
        height: u32,
        width_x_height: usize,
        pixel_count: usize,
    },
}
```

An error produced when using [`from_rgba`](fn.from_rgba.md) with invalid arguments.

**Source:** [icon.rs](../../../src/iced_core/window/icon.rs.md) (lines 58-85)

## Variants

### `ByteCountNotDivisibleBy4`

Produced when the length of the `rgba` argument isn’t divisible by 4, thus `rgba` can’t be safely interpreted as 32bpp RGBA pixels.

**Fields**

- `byte_count: usize` — The length of the provided RGBA data.

### `DimensionsVsPixelCount`

Produced when the number of pixels (`rgba.len() / 4`) isn’t equal to `width * height`. At least one of your arguments is incorrect.

**Fields**

- `width: u32` — The provided width.
- `height: u32` — The provided height.
- `width_x_height: usize` — The product of `width` and `height`.
- `pixel_count: usize` — The amount of pixels of the provided RGBA data.

## Trait Implementations

### `impl Debug for Error`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Display for Error`

#### `fmt`

```rust
fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl Error for Error`

#### `source`

```rust
fn source(&self) -> Option<&(dyn Error + 'static)>
```

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

#### `description`

```rust
fn description(&self) -> &str
```

*👎Deprecated since 1.42.0: use the Display impl or to_string()*

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

#### `cause`

```rust
fn cause(&self) -> Option<&dyn Error>
```

*👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting*

#### `provide`

```rust
fn provide<'a>(&'a self, request: &mut Request<'a>)
```

*🔬This is a nightly-only experimental API. (error_generic_member_access)*

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

## Auto Trait Implementations

- `impl Freeze for Error`
- `impl RefUnwindSafe for Error`
- `impl Send for Error`
- `impl Sync for Error`
- `impl Unpin for Error`
- `impl UnsafeUnpin for Error`
- `impl UnwindSafe for Error`
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
