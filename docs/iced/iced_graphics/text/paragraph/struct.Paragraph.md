---
title: "Paragraph"
kind: struct
path: "iced_graphics::text::paragraph::Paragraph"
crate: "iced_graphics"
module: "iced_graphics::text::paragraph"
file: "iced_graphics/text/paragraph/struct.Paragraph.md"
docs_url: "https://docs.iced.rs/iced_graphics/text/paragraph/struct.Paragraph.html"
source_file: "src/iced_graphics/text/paragraph.rs"
source_lines: "15"
---

# `iced_graphics::text::paragraph::Paragraph`

```rust
pub struct Paragraph(/* private fields */);
```

A bunch of text.

**Source:** [paragraph.rs](../../../src/iced_graphics/text/paragraph.rs.md) (lines 15)

## Methods

### `impl Paragraph`

#### `new`

```rust
pub fn new() -> Self
```

Creates a new empty [`Paragraph`](struct.Paragraph.md).

#### `buffer`

```rust
pub fn buffer(&self) -> &Buffer
```

Returns the buffer of the [`Paragraph`](struct.Paragraph.md).

#### `downgrade`

```rust
pub fn downgrade(&self) -> Weak
```

Creates a [`Weak`](struct.Weak.md) reference to the [`Paragraph`](struct.Paragraph.md).

This is useful to avoid cloning the [`Paragraph`](struct.Paragraph.md) when referential guarantees are unnecessary. For instance, when creating a rendering tree.

## Trait Implementations

### `impl Clone for Paragraph`

#### `clone`

```rust
fn clone(&self) -> Paragraph
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Paragraph`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Paragraph`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Paragraph for Paragraph`

#### `with_text`

```rust
fn with_text(text: Text<&str>) -> Self
```

Creates a new [`Paragraph`] laid out with the given [`Text`].

#### `with_spans`

```rust
fn with_spans<Link>(text: Text<&[Span<'_, Link>]>) -> Self
```

Creates a new [`Paragraph`] laid out with the given [`Text`].

#### `resize`

```rust
fn resize(&mut self, new_bounds: Size)
```

Lays out the [`Paragraph`] with some new boundaries.

#### `compare`

```rust
fn compare(&self, text: Text<()>) -> Difference
```

Compares the [`Paragraph`] with some desired [`Text`] and returns the [`Difference`].

#### `hint_factor`

```rust
fn hint_factor(&self) -> Option<f32>
```

Returns the hint factor of the [`Paragraph`].

#### `size`

```rust
fn size(&self) -> Pixels
```

Returns the text size of the [`Paragraph`] in [`Pixels`].

#### `font`

```rust
fn font(&self) -> Font
```

Returns the [`Font`] of the [`Paragraph`].

#### `line_height`

```rust
fn line_height(&self) -> LineHeight
```

Returns the [`LineHeight`] of the [`Paragraph`].

#### `align_x`

```rust
fn align_x(&self) -> Alignment
```

Returns the horizontal alignment of the [`Paragraph`].

#### `align_y`

```rust
fn align_y(&self) -> Vertical
```

Returns the vertical alignment of the [`Paragraph`].

#### `wrapping`

```rust
fn wrapping(&self) -> Wrapping
```

Returns the [`Wrapping`] strategy of the [`Paragraph`]>

#### `ellipsis`

```rust
fn ellipsis(&self) -> Ellipsis
```

Returns the [`Ellipsis`] strategy of the [`Paragraph`]>

#### `shaping`

```rust
fn shaping(&self) -> Shaping
```

Returns the [`Shaping`] strategy of the [`Paragraph`]>

#### `bounds`

```rust
fn bounds(&self) -> Size
```

Returns the available bounds used to layout the [`Paragraph`].

#### `min_bounds`

```rust
fn min_bounds(&self) -> Size
```

Returns the minimum boundaries that can fit the contents of the [`Paragraph`].

#### `hit_test`

```rust
fn hit_test(&self, point: Point) -> Option<Hit>
```

Tests whether the provided point is within the boundaries of the [`Paragraph`], returning information about the nearest character.

#### `hit_glyph`

```rust
fn hit_glyph(&self, point: Point) -> bool
```

Tests whether the provided point is inside the boundaries of a glyph in the [`Paragraph`].

#### `hit_span`

```rust
fn hit_span(&self, point: Point) -> Option<usize>
```

Tests whether the provided point is within the boundaries of a [`Span`] in the [`Paragraph`], returning the index of the [`Span`] that was hit.

#### `span_bounds`

```rust
fn span_bounds(&self, index: usize) -> Vec<Rectangle>
```

Returns all bounds for the provided [`Span`] index of the [`Paragraph`]. A [`Span`] can have multiple bounds for each line it’s on.

#### `select`

```rust
fn select(&mut self, start: Point, end: Point, target: Target)
```

Selects the text between the given positions. Read more

#### `select_all`

```rust
fn select_all(&mut self)
```

Selects all the text of the [`Paragraph`].

#### `deselect`

```rust
fn deselect(&mut self)
```

Deselects any selected text.

#### `selection`

```rust
fn selection(&self) -> &[Rectangle]
```

Returns the regions of the current selection of the [`Paragraph`]. Read more

#### `copy`

```rust
fn copy(&mut self) -> Option<String>
```

Returns the current selected text of the [`Paragraph`].

#### `min_width`

```rust
fn min_width(&self) -> f32
```

Returns the minimum width that can fit the contents of the [`Paragraph`].

#### `min_height`

```rust
fn min_height(&self) -> f32
```

Returns the minimum height that can fit the contents of the [`Paragraph`].

### `impl PartialEq for Paragraph`

#### `eq`

```rust
fn eq(&self, other: &Paragraph) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl StructuralPartialEq for Paragraph`

## Auto Trait Implementations

- `impl Freeze for Paragraph`
- `impl RefUnwindSafe for Paragraph`
- `impl Send for Paragraph`
- `impl Sync for Paragraph`
- `impl Unpin for Paragraph`
- `impl UnsafeUnpin for Paragraph`
- `impl UnwindSafe for Paragraph`
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

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
