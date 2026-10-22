---
title: "Theme"
kind: enum
path: "iced_core::theme::Theme"
crate: "iced_core"
module: "iced_core::theme"
file: "iced_core/theme/enum.Theme.md"
docs_url: "https://docs.iced.rs/iced_core/theme/enum.Theme.html"
source_file: "src/iced_core/theme.rs"
source_lines: "14-61"
---

# `iced_core::theme::Theme`

```rust
pub enum Theme {
Show 23 variants    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
    Custom(Arc<Custom>),
}
```

A built-in theme.

**Source:** [theme.rs](../../src/iced_core/theme.rs.md) (lines 14-61)

## Variants

### `Light`

The built-in light variant.

### `Dark`

The built-in dark variant.

### `Dracula`

The built-in Dracula variant.

### `Nord`

The built-in Nord variant.

### `SolarizedLight`

The built-in Solarized Light variant.

### `SolarizedDark`

The built-in Solarized Dark variant.

### `GruvboxLight`

The built-in Gruvbox Light variant.

### `GruvboxDark`

The built-in Gruvbox Dark variant.

### `CatppuccinLatte`

The built-in Catppuccin Latte variant.

### `CatppuccinFrappe`

The built-in Catppuccin Frappé variant.

### `CatppuccinMacchiato`

The built-in Catppuccin Macchiato variant.

### `CatppuccinMocha`

The built-in Catppuccin Mocha variant.

### `TokyoNight`

The built-in Tokyo Night variant.

### `TokyoNightStorm`

The built-in Tokyo Night Storm variant.

### `TokyoNightLight`

The built-in Tokyo Night Light variant.

### `KanagawaWave`

The built-in Kanagawa Wave variant.

### `KanagawaDragon`

The built-in Kanagawa Dragon variant.

### `KanagawaLotus`

The built-in Kanagawa Lotus variant.

### `Moonfly`

The built-in Moonfly variant.

### `Nightfly`

The built-in Nightfly variant.

### `Oxocarbon`

The built-in Oxocarbon variant.

### `Ferra`

The built-in Ferra variant:

### `Custom(Arc<Custom>)`

A [`Theme`](enum.Theme.md) that uses a [`Custom`](struct.Custom.md) palette.

## Methods

### `impl Theme`

#### `ALL`

```rust
pub const ALL: &'static [Self]
```

A list with all the defined themes.

#### `custom`

```rust
pub fn custom(name: impl Into<Cow<'static, str>>, seed: Seed) -> Self
```

Creates a new custom [`Theme`](enum.Theme.md) from the given [`Seed`](palette/struct.Seed.md).

#### `custom_with_fn`

```rust
pub fn custom_with_fn( name: impl Into<Cow<'static, str>>, palette: Seed, generate: impl FnOnce(Seed) -> Palette, ) -> Self
```

Creates a new custom [`Theme`](enum.Theme.md) from the given [`Seed`](palette/struct.Seed.md), with a custom generator of a [`Palette`](palette/struct.Palette.md).

#### `palette`

```rust
pub fn palette(&self) -> &Palette
```

Returns the [`Palette`](palette/struct.Palette.md) of the [`Theme`](enum.Theme.md).

#### `seed`

```rust
pub fn seed(&self) -> Seed
```

Returns the [`Seed`](palette/struct.Seed.md) of the [`Theme`](enum.Theme.md).

## Trait Implementations

### `impl Base for Theme`

#### `default`

```rust
fn default(preference: Mode) -> Self
```

Returns the default theme for the preferred [`Mode`](enum.Mode.md).

#### `mode`

```rust
fn mode(&self) -> Mode
```

Returns the [`Mode`](enum.Mode.md) of the theme.

#### `base`

```rust
fn base(&self) -> Style
```

Returns the default base [`Style`](struct.Style.md) of the theme.

#### `seed`

```rust
fn seed(&self) -> Option<Seed>
```

Returns the [`Seed`](palette/struct.Seed.md) of the theme. [Read more](trait.Base.md)

#### `name`

```rust
fn name(&self) -> &str
```

Returns the unique name of the theme. [Read more](trait.Base.md)

### `impl Catalog for Theme`

#### `Class`

```rust
type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>
```

The item class of this [`Catalog`](../widget/text/trait.Catalog.md).

#### `default`

```rust
fn default<'a>() -> Self::Class<'a>
```

The default class produced by this [`Catalog`](../widget/text/trait.Catalog.md).

#### `selection`

```rust
fn selection(&self) -> Color
```

The global selection [`Color`](../color/struct.Color.md).

#### `style`

```rust
fn style(&self, class: &Self::Class<'_>) -> Style
```

The [`Style`](../widget/text/struct.Style.md) of a class with the given status.

### `impl Clone for Theme`

#### `clone`

```rust
fn clone(&self) -> Theme
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Theme`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Display for Theme`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl PartialEq for Theme`

#### `eq`

```rust
fn eq(&self, other: &Theme) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl StructuralPartialEq for Theme`

## Auto Trait Implementations

- `impl Freeze for Theme`
- `impl RefUnwindSafe for Theme`
- `impl Send for Theme`
- `impl Sync for Theme`
- `impl Unpin for Theme`
- `impl UnsafeUnpin for Theme`
- `impl UnwindSafe for Theme`
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
