---
title: "Color"
kind: struct
path: "iced_core::color::Color"
crate: "iced_core"
module: "iced_core::color"
file: "iced_core/color/struct.Color.md"
docs_url: "https://docs.iced.rs/iced_core/color/struct.Color.html"
source_file: "src/iced_core/color.rs"
source_lines: "20-29"
---

# `iced_core::color::Color`

```rust
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
```

A color in the `sRGB` color space.

## String Representation

A color can be represented in either of the following valid formats: `#rrggbb`, `#rrggbbaa`, `#rgb`, and `#rgba`. Where `rgba` represent hexadecimal digits. Both uppercase and lowercase letters are supported.

If `a` (transparency) is not specified, `1.0` (completely opaque) would be used by default.

If you have a static color string, using the [`color!`](../macro.color.md) macro should be preferred since it leverages hexadecimal literal notation and arithmetic directly.

**Source:** [color.rs](../../src/iced_core/color.rs.md) (lines 20-29)

## Fields

### `r: f32`

Red component, 0.0 - 1.0

### `g: f32`

Green component, 0.0 - 1.0

### `b: f32`

Blue component, 0.0 - 1.0

### `a: f32`

Transparency, 0.0 - 1.0

## Methods

### `impl Color`

#### `BLACK`

```rust
pub const BLACK: Color
```

The black color.

#### `WHITE`

```rust
pub const WHITE: Color
```

The white color.

#### `TRANSPARENT`

```rust
pub const TRANSPARENT: Color
```

A color with no opacity.

#### `from_rgb`

```rust
pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self
```

Creates a [`Color`](struct.Color.md) from its RGB components.

#### `from_rgba`

```rust
pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self
```

Creates a [`Color`](struct.Color.md) from its RGBA components.

#### `from_rgb8`

```rust
pub const fn from_rgb8(r: u8, g: u8, b: u8) -> Self
```

Creates a [`Color`](struct.Color.md) from its RGB8 components.

#### `from_rgba8`

```rust
pub const fn from_rgba8(r: u8, g: u8, b: u8, a: f32) -> Self
```

Creates a [`Color`](struct.Color.md) from its RGB8 components and an alpha value.

#### `from_packed_rgb8`

```rust
pub const fn from_packed_rgb8(rgb: u32) -> Self
```

Creates a [`Color`](struct.Color.md) from its RGB8 components packed in the lower bits of a `u32`.

#### `from_packed_rgba8`

```rust
pub const fn from_packed_rgba8(rgb: u32, a: f32) -> Self
```

Creates a [`Color`](struct.Color.md) from its RGB8 components packed in the lower bits of a `u32` and an alpha value.

#### `from_linear_rgba`

```rust
pub fn from_linear_rgba(r: f32, g: f32, b: f32, a: f32) -> Self
```

Creates a [`Color`](struct.Color.md) from its linear RGBA components.

#### `from_oklch`

```rust
pub fn from_oklch(oklch: Oklch) -> Color
```

Creates the most approximate [`Color`](struct.Color.md) from its [`Oklch`](struct.Oklch.md) representation.

#### `invert`

```rust
pub const fn invert(&mut self)
```

Inverts the [`Color`](struct.Color.md) in-place.

#### `inverse`

```rust
pub const fn inverse(self) -> Self
```

Returns the inverted [`Color`](struct.Color.md).

#### `scale_alpha`

```rust
pub const fn scale_alpha(self, factor: f32) -> Self
```

Scales the alpha channel of the [`Color`](struct.Color.md) by the given factor.

#### `mix`

```rust
pub fn mix(self, b: Color, factor: f32) -> Color
```

Mixes the current [`Color`](struct.Color.md) with another one by the given factor.

#### `relative_luminance`

```rust
pub fn relative_luminance(self) -> f32
```

Returns the relative luminance of the [`Color`](struct.Color.md). [https://www.w3.org/TR/WCAG21/#dfn-relative-luminance](https://www.w3.org/TR/WCAG21/#dfn-relative-luminance)

#### `relative_contrast`

```rust
pub fn relative_contrast(self, b: Self) -> f32
```

Returns the [relative contrast ratio](https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio) of the [`Color`](struct.Color.md) against another one.

#### `is_readable_on`

```rust
pub fn is_readable_on(self, background: Self) -> bool
```

Returns true if the current [`Color`](struct.Color.md) is readable on top of the given background [`Color`](struct.Color.md).

#### `into_rgba8`

```rust
pub const fn into_rgba8(self) -> [u8; 4]
```

Converts the [`Color`](struct.Color.md) into its RGBA8 equivalent.

#### `into_linear`

```rust
pub fn into_linear(self) -> [f32; 4]
```

Converts the [`Color`](struct.Color.md) into its linear values.

#### `into_oklch`

```rust
pub fn into_oklch(self) -> Oklch
```

Converts the [`Color`](struct.Color.md) into its [`Oklch`](struct.Oklch.md) representation.

## Trait Implementations

### `impl Clone for Color`

#### `clone`

```rust
fn clone(&self) -> Color
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Color`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Color`

#### `default`

```rust
fn default() -> Color
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<'de> Deserialize<'de> for Color`

#### `deserialize`

```rust
fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error> where __D: Deserializer<'de>,
```

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

### `impl Display for Color`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl From<[f32; 3]> for Color`

#### `from`

```rust
fn from([r, g, b]: [f32; 3]) -> Self
```

Converts to this type from the input type.

### `impl From<[f32; 4]> for Color`

#### `from`

```rust
fn from([r, g, b, a]: [f32; 4]) -> Self
```

Converts to this type from the input type.

### `impl From<Color> for Background`

#### `from`

```rust
fn from(color: Color) -> Self
```

Converts to this type from the input type.

### `impl From<Color> for Oklch`

#### `from`

```rust
fn from(color: Color) -> Self
```

Converts to this type from the input type.

### `impl From<Oklch> for Color`

#### `from`

```rust
fn from(oklch: Oklch) -> Self
```

Converts to this type from the input type.

### `impl FromStr for Color`

#### `Err`

```rust
type Err = ParseError
```

The associated error which can be returned from parsing.

#### `from_str`

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

### `impl Interpolable for Color`

#### `interpolated`

```rust
fn interpolated(&self, other: Self, ratio: f32) -> Self
```

Interpolates the color. Equivalent to [`Color::mix`](struct.Color.md).

### `impl PartialEq for Color`

#### `eq`

```rust
fn eq(&self, other: &Color) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Serialize for Color`

#### `serialize`

```rust
fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error> where __S: Serializer,
```

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/ser/trait.Serialize.html#tymethod.serialize)

### `impl Copy for Color`

### `impl StructuralPartialEq for Color`

## Auto Trait Implementations

- `impl Freeze for Color`
- `impl RefUnwindSafe for Color`
- `impl Send for Color`
- `impl Sync for Color`
- `impl Unpin for Color`
- `impl UnsafeUnpin for Color`
- `impl UnwindSafe for Color`
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

### `impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de>,`
