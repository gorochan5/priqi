---
title: "Plain"
kind: struct
path: "iced_core::text::paragraph::Plain"
crate: "iced_core"
module: "iced_core::text::paragraph"
file: "iced_core/text/paragraph/struct.Plain.md"
docs_url: "https://docs.iced.rs/iced_core/text/paragraph/struct.Plain.html"
source_file: "src/iced_core/text/paragraph.rs"
source_lines: "113-116"
---

# `iced_core::text::paragraph::Plain`

```rust
pub struct Plain<P: Paragraph> { /* private fields */ }
```

A [`Paragraph`](trait.Paragraph.md) of plain text.

**Source:** [paragraph.rs](../../../src/iced_core/text/paragraph.rs.md) (lines 113-116)

## Methods

### `impl<P: Paragraph> Plain<P>`

#### `new`

```rust
pub fn new(text: Text<String>) -> Self
```

Creates a new [`Plain`](struct.Plain.md) paragraph.

#### `update`

```rust
pub fn update(&mut self, text: Text<&str>) -> bool
```

Updates the plain [`Paragraph`](trait.Paragraph.md) to match the given [`Text`](../struct.Text.md), if needed.

Returns true if the [`Paragraph`](trait.Paragraph.md) changed.

#### `align_x`

```rust
pub fn align_x(&self) -> Alignment
```

Returns the horizontal alignment of the [`Paragraph`](trait.Paragraph.md).

#### `align_y`

```rust
pub fn align_y(&self) -> Vertical
```

Returns the vertical alignment of the [`Paragraph`](trait.Paragraph.md).

#### `min_bounds`

```rust
pub fn min_bounds(&self) -> Size
```

Returns the minimum boundaries that can fit the contents of the [`Paragraph`](trait.Paragraph.md).

#### `min_width`

```rust
pub fn min_width(&self) -> f32
```

Returns the minimum width that can fit the contents of the [`Paragraph`](trait.Paragraph.md).

#### `min_height`

```rust
pub fn min_height(&self) -> f32
```

Returns the minimum height that can fit the contents of the [`Paragraph`](trait.Paragraph.md).

#### `raw`

```rust
pub fn raw(&self) -> &P
```

Returns the cached [`Paragraph`](trait.Paragraph.md).

#### `raw_mut`

```rust
pub fn raw_mut(&mut self) -> &mut P
```

Returns the cached [`Paragraph`](trait.Paragraph.md), mutably.

#### `content`

```rust
pub fn content(&self) -> &str
```

Returns the current content of the plain [`Paragraph`](trait.Paragraph.md).

#### `as_text`

```rust
pub fn as_text(&self) -> Text<&str>
```

Returns the [`Paragraph`](trait.Paragraph.md) as a [`Text`](../struct.Text.md) definition.

## Trait Implementations

### `impl<P: Clone + Paragraph> Clone for Plain<P>`

#### `clone`

```rust
fn clone(&self) -> Plain<P>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<P: Debug + Paragraph> Debug for Plain<P>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<P: Default + Paragraph> Default for Plain<P>`

#### `default`

```rust
fn default() -> Plain<P>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations

- `impl<P> Freeze for Plain<P> where P: Freeze,`
- `impl<P> RefUnwindSafe for Plain<P> where P: RefUnwindSafe,`
- `impl<P> Send for Plain<P> where P: Send,`
- `impl<P> Sync for Plain<P> where P: Sync,`
- `impl<P> Unpin for Plain<P> where P: Unpin,`
- `impl<P> UnsafeUnpin for Plain<P> where P: UnsafeUnpin,`
- `impl<P> UnwindSafe for Plain<P> where P: UnwindSafe,`
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
