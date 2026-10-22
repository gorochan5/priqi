---
title: "Cache"
kind: struct
path: "iced_graphics::geometry::Cache"
crate: "iced_graphics"
module: "iced_graphics::geometry"
file: "iced_graphics/geometry/struct.Cache.md"
docs_url: "https://docs.iced.rs/iced_graphics/geometry/struct.Cache.html"
source_file: "src/iced_graphics/geometry/cache.rs"
source_lines: "11-16"
---

# `iced_graphics::geometry::Cache`

```rust
pub struct Cache<Renderer> where
    Renderer: Renderer, { /* private fields */ }
```

A simple cache that stores generated geometry to avoid recomputation.

A [`Cache`](struct.Cache.md) will not redraw its geometry unless the dimensions of its layer change or it is explicitly cleared.

**Source:** [cache.rs](../../src/iced_graphics/geometry/cache.rs.md) (lines 11-16)

## Methods

### `impl<Renderer> Cache<Renderer> where Renderer: Renderer,`

#### `new`

```rust
pub fn new() -> Self
```

Creates a new empty [`Cache`](struct.Cache.md).

#### `with_group`

```rust
pub fn with_group(group: Group) -> Self
```

Creates a new empty [`Cache`](struct.Cache.md) with the given [`Group`](../cache/struct.Group.md).

Caches within the same group may reuse internal rendering storage.

You should generally group caches that are likely to change together.

#### `clear`

```rust
pub fn clear(&self)
```

Clears the [`Cache`](struct.Cache.md), forcing a redraw the next time it is used.

#### `draw`

```rust
pub fn draw( &self, renderer: &Renderer, size: Size, draw_fn: impl FnOnce(&mut Frame<Renderer>), ) -> Renderer::Geometry
```

Draws geometry using the provided closure and stores it in the [`Cache`](struct.Cache.md).

The closure will only be called when

- the size has changed since the previous draw call.
- the [`Cache`](struct.Cache.md) is empty or has been explicitly cleared.

Otherwise, the previously stored geometry will be returned. The [`Cache`](struct.Cache.md) is not cleared in this case. In other words, it will keep returning the stored geometry if needed.

#### `draw_with_bounds`

```rust
pub fn draw_with_bounds( &self, renderer: &Renderer, bounds: Rectangle, draw_fn: impl FnOnce(&mut Frame<Renderer>), ) -> Renderer::Geometry
```

Draws geometry using the provided closure and stores it in the [`Cache`](struct.Cache.md).

Analogous to [`draw`](struct.Cache.md), but takes a clipping [`Rectangle`] instead of a [`Size`].

## Trait Implementations

### `impl<Renderer> Debug for Cache<Renderer> where Renderer: Renderer, <Renderer::Geometry as Cached>::Cache: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<Renderer> Default for Cache<Renderer> where Renderer: Renderer,`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations

- `impl<Renderer> !Freeze for Cache<Renderer>`
- `impl<Renderer> !RefUnwindSafe for Cache<Renderer>`
- `impl<Renderer> Send for Cache<Renderer> where <<Renderer as Renderer>::Geometry as Cached>::Cache: Send,`
- `impl<Renderer> !Sync for Cache<Renderer>`
- `impl<Renderer> Unpin for Cache<Renderer> where <<Renderer as Renderer>::Geometry as Cached>::Cache: Unpin,`
- `impl<Renderer> UnsafeUnpin for Cache<Renderer> where <<Renderer as Renderer>::Geometry as Cached>::Cache: UnsafeUnpin,`
- `impl<Renderer> UnwindSafe for Cache<Renderer> where <<Renderer as Renderer>::Geometry as Cached>::Cache: UnwindSafe,`
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

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
