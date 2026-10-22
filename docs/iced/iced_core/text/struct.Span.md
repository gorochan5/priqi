---
title: "Span"
kind: struct
path: "iced_core::text::Span"
crate: "iced_core"
module: "iced_core::text"
file: "iced_core/text/struct.Span.md"
docs_url: "https://docs.iced.rs/iced_core/text/struct.Span.html"
source_file: "src/iced_core/text.rs"
source_lines: "405-428"
---

# `iced_core::text::Span`

```rust
pub struct Span<'a, Link = ()> {
    pub text: Fragment<'a>,
    pub size: Option<Pixels>,
    pub line_height: Option<LineHeight>,
    pub font: Option<Font>,
    pub color: Option<Color>,
    pub link: Option<Link>,
    pub highlight: Option<Highlight>,
    pub padding: Padding,
    pub underline: bool,
    pub strikethrough: bool,
}
```

A span of text.

**Source:** [text.rs](../../src/iced_core/text.rs.md) (lines 405-428)

## Fields

### `text: Fragment<'a>`

The [`Fragment`](type.Fragment.md) of text.

### `size: Option<Pixels>`

The size of the [`Span`](struct.Span.md) in [`Pixels`](../struct.Pixels.md).

### `line_height: Option<LineHeight>`

The [`LineHeight`](enum.LineHeight.md) of the [`Span`](struct.Span.md).

### `font: Option<Font>`

The [`Font`](../font/struct.Font.md) of the [`Span`](struct.Span.md).

### `color: Option<Color>`

The [`Color`](../color/struct.Color.md) of the [`Span`](struct.Span.md).

### `link: Option<Link>`

The link of the [`Span`](struct.Span.md).

### `highlight: Option<Highlight>`

The [`Highlight`](struct.Highlight.md) of the [`Span`](struct.Span.md).

### `padding: Padding`

The [`Padding`](../padding/struct.Padding.md) of the [`Span`](struct.Span.md).

Currently, it only affects the bounds of the [`Highlight`](struct.Highlight.md).

### `underline: bool`

Whether the [`Span`](struct.Span.md) should be underlined or not.

### `strikethrough: bool`

Whether the [`Span`](struct.Span.md) should be struck through or not.

## Methods

### `impl<'a, Link> Span<'a, Link>`

#### `new`

```rust
pub fn new(fragment: impl IntoFragment<'a>) -> Self
```

Creates a new [`Span`](struct.Span.md) of text with the given text fragment.

#### `size`

```rust
pub fn size(self, size: impl Into<Pixels>) -> Self
```

Sets the size of the [`Span`](struct.Span.md).

#### `line_height`

```rust
pub fn line_height(self, line_height: impl Into<LineHeight>) -> Self
```

Sets the [`LineHeight`](enum.LineHeight.md) of the [`Span`](struct.Span.md).

#### `font`

```rust
pub fn font(self, font: impl Into<Font>) -> Self
```

Sets the font of the [`Span`](struct.Span.md).

#### `font_maybe`

```rust
pub fn font_maybe(self, font: Option<impl Into<Font>>) -> Self
```

Sets the font of the [`Span`](struct.Span.md), if any.

#### `color`

```rust
pub fn color(self, color: impl Into<Color>) -> Self
```

Sets the [`Color`](../color/struct.Color.md) of the [`Span`](struct.Span.md).

#### `color_maybe`

```rust
pub fn color_maybe(self, color: Option<impl Into<Color>>) -> Self
```

Sets the [`Color`](../color/struct.Color.md) of the [`Span`](struct.Span.md), if any.

#### `link`

```rust
pub fn link(self, link: impl Into<Link>) -> Self
```

Sets the link of the [`Span`](struct.Span.md).

#### `link_maybe`

```rust
pub fn link_maybe(self, link: Option<impl Into<Link>>) -> Self
```

Sets the link of the [`Span`](struct.Span.md), if any.

#### `background`

```rust
pub fn background(self, background: impl Into<Background>) -> Self
```

Sets the [`Background`](../enum.Background.md) of the [`Span`](struct.Span.md).

#### `background_maybe`

```rust
pub fn background_maybe(self, background: Option<impl Into<Background>>) -> Self
```

Sets the [`Background`](../enum.Background.md) of the [`Span`](struct.Span.md), if any.

#### `border`

```rust
pub fn border(self, border: impl Into<Border>) -> Self
```

Sets the [`Border`](../border/struct.Border.md) of the [`Span`](struct.Span.md).

#### `border_maybe`

```rust
pub fn border_maybe(self, border: Option<impl Into<Border>>) -> Self
```

Sets the [`Border`](../border/struct.Border.md) of the [`Span`](struct.Span.md), if any.

#### `padding`

```rust
pub fn padding(self, padding: impl Into<Padding>) -> Self
```

Sets the [`Padding`](../padding/struct.Padding.md) of the [`Span`](struct.Span.md).

It only affects the [`background`](struct.Span.md) and [`border`](struct.Span.md) of the [`Span`](struct.Span.md), currently.

#### `underline`

```rust
pub fn underline(self, underline: bool) -> Self
```

Sets whether the [`Span`](struct.Span.md) should be underlined or not.

#### `strikethrough`

```rust
pub fn strikethrough(self, strikethrough: bool) -> Self
```

Sets whether the [`Span`](struct.Span.md) should be struck through or not.

#### `to_static`

```rust
pub fn to_static(self) -> Span<'static, Link>
```

Turns the [`Span`](struct.Span.md) into a static one.

## Trait Implementations

### `impl<'a, Link: Clone> Clone for Span<'a, Link>`

#### `clone`

```rust
fn clone(&self) -> Span<'a, Link>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<'a, Link: Debug> Debug for Span<'a, Link>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<Link> Default for Span<'_, Link>`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<'a, Link> From<&'a str> for Span<'a, Link>`

#### `from`

```rust
fn from(value: &'a str) -> Self
```

Converts to this type from the input type.

### `impl<Link> PartialEq for Span<'_, Link>`

#### `eq`

```rust
fn eq(&self, other: &Self) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations

- `impl<'a, Link> Freeze for Span<'a, Link> where Link: Freeze,`
- `impl<'a, Link> RefUnwindSafe for Span<'a, Link> where Link: RefUnwindSafe,`
- `impl<'a, Link> Send for Span<'a, Link> where Link: Send,`
- `impl<'a, Link> Sync for Span<'a, Link> where Link: Sync,`
- `impl<'a, Link> Unpin for Span<'a, Link> where Link: Unpin,`
- `impl<'a, Link> UnsafeUnpin for Span<'a, Link> where Link: UnsafeUnpin,`
- `impl<'a, Link> UnwindSafe for Span<'a, Link> where Link: UnwindSafe,`
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
