---
title: "Seed"
kind: struct
path: "iced_core::theme::palette::Seed"
crate: "iced_core"
module: "iced_core::theme::palette"
file: "iced_core/theme/palette/struct.Seed.md"
docs_url: "https://docs.iced.rs/iced_core/theme/palette/struct.Seed.html"
source_file: "src/iced_core/theme/palette.rs"
source_lines: "152-165"
---

# `iced_core::theme::palette::Seed`

```rust
pub struct Seed {
    pub background: Color,
    pub text: Color,
    pub primary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
}
```

The base set of colors of a [`Palette`](struct.Palette.md).

**Source:** [palette.rs](../../../src/iced_core/theme/palette.rs.md) (lines 152-165)

## Fields

### `background: Color`

The background [`Color`](../../color/struct.Color.md) of the [`Palette`](struct.Palette.md).

### `text: Color`

The text [`Color`](../../color/struct.Color.md) of the [`Palette`](struct.Palette.md).

### `primary: Color`

The primary [`Color`](../../color/struct.Color.md) of the [`Palette`](struct.Palette.md).

### `success: Color`

The success [`Color`](../../color/struct.Color.md) of the [`Palette`](struct.Palette.md).

### `warning: Color`

The warning [`Color`](../../color/struct.Color.md) of the [`Palette`](struct.Palette.md).

### `danger: Color`

The danger [`Color`](../../color/struct.Color.md) of the [`Palette`](struct.Palette.md).

## Methods

### `impl Seed`

#### `LIGHT`

```rust
pub const LIGHT: Self
```

The built-in light variant of a [`Palette`](struct.Palette.md).

#### `DARK`

```rust
pub const DARK: Self
```

The built-in dark variant of a [`Palette`](struct.Palette.md).

#### `DRACULA`

```rust
pub const DRACULA: Self
```

The built-in [Dracula](https://draculatheme.com) variant of a [`Palette`](struct.Palette.md).

#### `NORD`

```rust
pub const NORD: Self
```

The built-in [Nord](https://www.nordtheme.com/docs/colors-and-palettes) variant of a [`Palette`](struct.Palette.md).

#### `SOLARIZED_LIGHT`

```rust
pub const SOLARIZED_LIGHT: Self
```

The built-in [Solarized](https://ethanschoonover.com/solarized) Light variant of a [`Palette`](struct.Palette.md).

#### `SOLARIZED_DARK`

```rust
pub const SOLARIZED_DARK: Self
```

The built-in [Solarized](https://ethanschoonover.com/solarized) Dark variant of a [`Palette`](struct.Palette.md).

#### `GRUVBOX_LIGHT`

```rust
pub const GRUVBOX_LIGHT: Self
```

The built-in [Gruvbox](https://github.com/morhetz/gruvbox) Light variant of a [`Palette`](struct.Palette.md).

#### `GRUVBOX_DARK`

```rust
pub const GRUVBOX_DARK: Self
```

The built-in [Gruvbox](https://github.com/morhetz/gruvbox) Dark variant of a [`Palette`](struct.Palette.md).

#### `CATPPUCCIN_LATTE`

```rust
pub const CATPPUCCIN_LATTE: Self
```

The built-in [Catppuccin](https://github.com/catppuccin/catppuccin) Latte variant of a [`Palette`](struct.Palette.md).

#### `CATPPUCCIN_FRAPPE`

```rust
pub const CATPPUCCIN_FRAPPE: Self
```

The built-in [Catppuccin](https://github.com/catppuccin/catppuccin) Frappé variant of a [`Palette`](struct.Palette.md).

#### `CATPPUCCIN_MACCHIATO`

```rust
pub const CATPPUCCIN_MACCHIATO: Self
```

The built-in [Catppuccin](https://github.com/catppuccin/catppuccin) Macchiato variant of a [`Palette`](struct.Palette.md).

#### `CATPPUCCIN_MOCHA`

```rust
pub const CATPPUCCIN_MOCHA: Self
```

The built-in [Catppuccin](https://github.com/catppuccin/catppuccin) Mocha variant of a [`Palette`](struct.Palette.md).

#### `TOKYO_NIGHT`

```rust
pub const TOKYO_NIGHT: Self
```

The built-in [Tokyo Night](https://github.com/enkia/tokyo-night-vscode-theme) variant of a [`Palette`](struct.Palette.md).

#### `TOKYO_NIGHT_STORM`

```rust
pub const TOKYO_NIGHT_STORM: Self
```

The built-in [Tokyo Night](https://github.com/enkia/tokyo-night-vscode-theme) Storm variant of a [`Palette`](struct.Palette.md).

#### `TOKYO_NIGHT_LIGHT`

```rust
pub const TOKYO_NIGHT_LIGHT: Self
```

The built-in [Tokyo Night](https://github.com/enkia/tokyo-night-vscode-theme) Light variant of a [`Palette`](struct.Palette.md).

#### `KANAGAWA_WAVE`

```rust
pub const KANAGAWA_WAVE: Self
```

The built-in [Kanagawa](https://github.com/rebelot/kanagawa.nvim) Wave variant of a [`Palette`](struct.Palette.md).

#### `KANAGAWA_DRAGON`

```rust
pub const KANAGAWA_DRAGON: Self
```

The built-in [Kanagawa](https://github.com/rebelot/kanagawa.nvim) Dragon variant of a [`Palette`](struct.Palette.md).

#### `KANAGAWA_LOTUS`

```rust
pub const KANAGAWA_LOTUS: Self
```

The built-in [Kanagawa](https://github.com/rebelot/kanagawa.nvim) Lotus variant of a [`Palette`](struct.Palette.md).

#### `MOONFLY`

```rust
pub const MOONFLY: Self
```

The built-in [Moonfly](https://github.com/bluz71/vim-moonfly-colors) variant of a [`Palette`](struct.Palette.md).

#### `NIGHTFLY`

```rust
pub const NIGHTFLY: Self
```

The built-in [Nightfly](https://github.com/bluz71/vim-nightfly-colors) variant of a [`Palette`](struct.Palette.md).

#### `OXOCARBON`

```rust
pub const OXOCARBON: Self
```

The built-in [Oxocarbon](https://github.com/nyoom-engineering/oxocarbon.nvim) variant of a [`Palette`](struct.Palette.md).

#### `FERRA`

```rust
pub const FERRA: Self
```

The built-in [Ferra](https://github.com/casperstorm/ferra) variant of a [`Palette`](struct.Palette.md).

## Trait Implementations

### `impl Clone for Seed`

#### `clone`

```rust
fn clone(&self) -> Seed
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Seed`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<'de> Deserialize<'de> for Seed`

#### `deserialize`

```rust
fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error> where __D: Deserializer<'de>,
```

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

### `impl PartialEq for Seed`

#### `eq`

```rust
fn eq(&self, other: &Seed) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl Serialize for Seed`

#### `serialize`

```rust
fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error> where __S: Serializer,
```

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/ser/trait.Serialize.html#tymethod.serialize)

### `impl Copy for Seed`

### `impl StructuralPartialEq for Seed`

## Auto Trait Implementations

- `impl Freeze for Seed`
- `impl RefUnwindSafe for Seed`
- `impl Send for Seed`
- `impl Sync for Seed`
- `impl Unpin for Seed`
- `impl UnsafeUnpin for Seed`
- `impl UnwindSafe for Seed`
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
