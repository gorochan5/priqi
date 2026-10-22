---
title: "ContentFit"
kind: enum
path: "iced_core::ContentFit"
crate: "iced_core"
module: "iced_core"
file: "iced_core/enum.ContentFit.md"
docs_url: "https://docs.iced.rs/iced_core/enum.ContentFit.html"
source_file: "src/iced_core/content_fit.rs"
source_lines: "17-66"
---

# `iced_core::ContentFit`

```rust
pub enum ContentFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}
```

The strategy used to fit the contents of a widget to its bounding box.

Each variant of this enum is a strategy that can be applied for resolving differences in aspect ratio and size between the image being displayed and the space its being displayed in.

For an interactive demonstration of these properties as they are implemented in CSS, see [Mozilla’s docs](https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit), or run the `tour` example

**Source:** [content_fit.rs](../src/iced_core/content_fit.rs.md) (lines 17-66)

## Variants

### `Contain`

Scale as big as it can be without needing to crop or hide parts.

The image will be scaled (preserving aspect ratio) so that it just fits within the window. This won’t distort the image or crop/hide any edges, but if the image doesn’t fit perfectly, there may be whitespace on the top/bottom or left/right.

This is a great fit for when you need to display an image without losing any part of it, particularly when the image itself is the focus of the screen.

### `Cover`

Scale the image to cover all of the bounding box, cropping if needed.

This doesn’t distort the image, and it ensures that the widget’s area is completely covered, but it might crop off a bit of the edges of the widget, particularly when there is a big difference between the aspect ratio of the widget and the aspect ratio of the image.

This is best for when you’re using an image as a background, or to fill space, and any details of the image around the edge aren’t too important.

### `Fill`

Distort the image so the widget is 100% covered without cropping.

This stretches the image to fit the widget, without any whitespace or cropping. However, because of the stretch, the image may look distorted or elongated, particularly when there’s a mismatch of aspect ratios.

### `None`

Don’t resize or scale the image at all.

This will not apply any transformations to the provided image, but also means that unless you do the math yourself, the widget’s area will not be completely covered, or the image might be cropped.

This is best for when you’ve sized the image yourself.

### `ScaleDown`

Scale the image down if it’s too big for the space, but never scale it up.

This works much like [`Contain`](enum.ContentFit.md), except that if the image would have been scaled up, it keeps its original resolution to avoid the bluring that accompanies upscaling images.

## Methods

### `impl ContentFit`

#### `fit`

```rust
pub fn fit(&self, content: Size, bounds: Size) -> Size
```

Attempt to apply the given fit for a content size within some bounds.

The returned value is the recommended scaled size of the content.

## Trait Implementations

### `impl Clone for ContentFit`

#### `clone`

```rust
fn clone(&self) -> ContentFit
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for ContentFit`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for ContentFit`

#### `default`

```rust
fn default() -> ContentFit
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Display for ContentFit`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl Hash for ContentFit`

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

### `impl PartialEq for ContentFit`

#### `eq`

```rust
fn eq(&self, other: &ContentFit) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Copy for ContentFit`

### `impl Eq for ContentFit`

### `impl StructuralPartialEq for ContentFit`

## Auto Trait Implementations

- `impl Freeze for ContentFit`
- `impl RefUnwindSafe for ContentFit`
- `impl Send for ContentFit`
- `impl Sync for ContentFit`
- `impl Unpin for ContentFit`
- `impl UnsafeUnpin for ContentFit`
- `impl UnwindSafe for ContentFit`
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
