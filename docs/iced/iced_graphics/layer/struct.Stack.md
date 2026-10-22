---
title: "Stack"
kind: struct
path: "iced_graphics::layer::Stack"
crate: "iced_graphics"
module: "iced_graphics::layer"
file: "iced_graphics/layer/struct.Stack.md"
docs_url: "https://docs.iced.rs/iced_graphics/layer/struct.Stack.html"
source_file: "src/iced_graphics/layer.rs"
source_lines: "48-54"
---

# `iced_graphics::layer::Stack`

```rust
pub struct Stack<T: Layer> { /* private fields */ }
```

A stack of layers used for drawing.

**Source:** [layer.rs](../../src/iced_graphics/layer.rs.md) (lines 48-54)

## Methods

### `impl<T: Layer> Stack<T>`

#### `new`

```rust
pub fn new() -> Self
```

Creates a new empty [`Stack`](struct.Stack.md).

#### `current_mut`

```rust
pub fn current_mut(&mut self) -> (&mut T, Transformation)
```

Returns a mutable reference to the current [`Layer`](trait.Layer.md) of the [`Stack`](struct.Stack.md), together with the current [`Transformation`].

#### `transformation`

```rust
pub fn transformation(&self) -> Transformation
```

Returns the current [`Transformation`] of the [`Stack`](struct.Stack.md).

#### `push_clip`

```rust
pub fn push_clip(&mut self, bounds: Rectangle)
```

Pushes a new clipping region in the [`Stack`](struct.Stack.md); creating a new layer in the process.

#### `pop_clip`

```rust
pub fn pop_clip(&mut self)
```

Pops the current clipping region from the [`Stack`](struct.Stack.md) and restores the previous one.

The current layer will be recorded for drawing.

#### `push_transformation`

```rust
pub fn push_transformation(&mut self, transformation: Transformation)
```

Pushes a new [`Transformation`] in the [`Stack`](struct.Stack.md).

Future drawing operations will be affected by this new [`Transformation`] until it is popped using [`pop_transformation`](struct.Stack.md).

#### `pop_transformation`

```rust
pub fn pop_transformation(&mut self)
```

Pops the current [`Transformation`] in the [`Stack`](struct.Stack.md).

#### `iter`

```rust
pub fn iter(&self) -> impl Iterator<Item = &T>
```

Returns an iterator over immutable references to the layers in the [`Stack`](struct.Stack.md).

#### `as_slice`

```rust
pub fn as_slice(&self) -> &[T]
```

Returns the slice of layers in the [`Stack`](struct.Stack.md).

#### `flush`

```rust
pub fn flush(&mut self)
```

Flushes and settles any primitives in the [`Stack`](struct.Stack.md).

#### `merge`

```rust
pub fn merge(&mut self)
```

Performs layer merging wherever possible.

Flushes and settles any primitives in the [`Stack`](struct.Stack.md).

#### `reset`

```rust
pub fn reset(&mut self, new_bounds: Rectangle)
```

Clears the layers of the [`Stack`](struct.Stack.md), allowing reuse.

It resizes the base layer bounds to the `new_bounds`.

This will normally keep layer allocations for future drawing operations.

## Trait Implementations

### `impl<T: Debug + Layer> Debug for Stack<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T: Layer> Default for Stack<T>`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations

- `impl<T> Freeze for Stack<T>`
- `impl<T> RefUnwindSafe for Stack<T> where T: RefUnwindSafe,`
- `impl<T> Send for Stack<T> where T: Send,`
- `impl<T> Sync for Stack<T> where T: Sync,`
- `impl<T> Unpin for Stack<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Stack<T>`
- `impl<T> UnwindSafe for Stack<T> where T: UnwindSafe,`
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

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
