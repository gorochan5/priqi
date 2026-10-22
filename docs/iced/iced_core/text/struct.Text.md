---
title: "Text"
kind: struct
path: "iced_core::text::Text"
crate: "iced_core"
module: "iced_core::text"
file: "iced_core/text/struct.Text.md"
docs_url: "https://docs.iced.rs/iced_core/text/struct.Text.html"
source_file: "src/iced_core/text.rs"
source_lines: "23-63"
---

# `iced_core::text::Text`

```rust
pub struct Text<Content = String> {
    pub content: Content,
    pub bounds: Size,
    pub size: Pixels,
    pub line_height: LineHeight,
    pub font: Font,
    pub align_x: Alignment,
    pub align_y: Vertical,
    pub shaping: Shaping,
    pub wrapping: Wrapping,
    pub ellipsis: Ellipsis,
    pub hint_factor: Option<f32>,
}
```

A paragraph.

**Source:** [text.rs](../../src/iced_core/text.rs.md) (lines 23-63)

## Fields

### `content: Content`

The content of the paragraph.

### `bounds: Size`

The bounds of the paragraph.

### `size: Pixels`

The size of the [`Text`](struct.Text.md) in logical pixels.

### `line_height: LineHeight`

The line height of the [`Text`](struct.Text.md).

### `font: Font`

The [`Font`](../font/struct.Font.md) of the [`Text`](struct.Text.md).

### `align_x: Alignment`

The horizontal alignment of the [`Text`](struct.Text.md).

### `align_y: Vertical`

The vertical alignment of the [`Text`](struct.Text.md).

### `shaping: Shaping`

The [`Shaping`](enum.Shaping.md) strategy of the [`Text`](struct.Text.md).

### `wrapping: Wrapping`

The [`Wrapping`](enum.Wrapping.md) strategy of the [`Text`](struct.Text.md).

### `ellipsis: Ellipsis`

The [`Ellipsis`](enum.Ellipsis.md) strategy of the [`Text`](struct.Text.md).

### `hint_factor: Option<f32>`

The scale factor that may be used to internally scale the layout calculation of the [`Paragraph`](paragraph/trait.Paragraph.md) and leverage metrics hinting.

Effectively, this defines the “base” layout that will be used for linear scaling.

If `None`, hinting will be disabled and subpixel positioning will be performed.

## Methods

### `impl<Content> Text<Content>`

#### `with_content`

```rust
pub fn with_content<T>(&self, content: T) -> Text<T>
```

Returns a new [`Text`](struct.Text.md) replacing only the content with the given value.

### `impl<Content: AsRef<str>> Text<Content>`

#### `as_ref`

```rust
pub fn as_ref(&self) -> Text<&str>
```

Returns a borrowed version of [`Text`](struct.Text.md).

## Trait Implementations

### `impl<Content: Clone> Clone for Text<Content>`

#### `clone`

```rust
fn clone(&self) -> Text<Content>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<Content: Debug> Debug for Text<Content>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<Content: Copy> Copy for Text<Content>`

## Auto Trait Implementations

- `impl<Content> Freeze for Text<Content> where Content: Freeze,`
- `impl<Content> RefUnwindSafe for Text<Content> where Content: RefUnwindSafe,`
- `impl<Content> Send for Text<Content> where Content: Send,`
- `impl<Content> Sync for Text<Content> where Content: Sync,`
- `impl<Content> Unpin for Text<Content> where Content: Unpin,`
- `impl<Content> UnsafeUnpin for Text<Content> where Content: UnsafeUnpin,`
- `impl<Content> UnwindSafe for Text<Content> where Content: UnwindSafe,`
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
