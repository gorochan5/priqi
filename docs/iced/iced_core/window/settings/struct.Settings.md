---
title: "Settings"
kind: struct
path: "iced_core::window::settings::Settings"
crate: "iced_core"
module: "iced_core::window::settings"
file: "iced_core/window/settings/struct.Settings.md"
docs_url: "https://docs.iced.rs/iced_core/window/settings/struct.Settings.html"
source_file: "src/iced_core/window/settings.rs"
source_lines: "34-101"
---

# `iced_core::window::settings::Settings`

```rust
pub struct Settings {Show 17 fields
    pub size: Size,
    pub maximized: bool,
    pub fullscreen: bool,
    pub position: Position,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub visible: bool,
    pub resizable: bool,
    pub closeable: bool,
    pub minimizable: bool,
    pub decorations: bool,
    pub transparent: bool,
    pub blur: bool,
    pub level: Level,
    pub icon: Option<Icon>,
    pub platform_specific: PlatformSpecific,
    pub exit_on_close_request: bool,
}
```

The window settings of an application.

**Source:** [settings.rs](../../../src/iced_core/window/settings.rs.md) (lines 34-101)

## Fields

### `size: Size`

The initial logical dimensions of the window.

### `maximized: bool`

Whether the window should start maximized.

### `fullscreen: bool`

Whether the window should start fullscreen.

### `position: Position`

The initial position of the window.

### `min_size: Option<Size>`

The minimum size of the window.

### `max_size: Option<Size>`

The maximum size of the window.

### `visible: bool`

Whether the window should be visible or not.

### `resizable: bool`

Whether the window should be resizable or not.

### `closeable: bool`

Whether the title bar has Close button or not

### `minimizable: bool`

Whether the title bar has Minimize button or not

### `decorations: bool`

Whether the window should have a border, a title bar, etc. or not.

### `transparent: bool`

Whether the window should be transparent.

### `blur: bool`

Whether the window should have blurry background.

Note that the blurry effect is applied to the transparent window. You need to enable [`Settings::transparent`](struct.Settings.md) and set a proper opacity value to the background color with `Application::style`.

This option is only supported on macOS and Linux. Please read the [winit document](https://docs.rs/winit/0.30/winit/window/struct.Window.html#method.set_blur) for more details.

### `level: Level`

The window [`Level`](../enum.Level.md).

### `icon: Option<Icon>`

The icon of the window.

### `platform_specific: PlatformSpecific`

Platform specific settings.

### `exit_on_close_request: bool`

Whether the window will close when the user requests it, e.g. when a user presses the close button.

This can be useful if you want to have some behavior that executes before the window is actually destroyed. If you disable this, you must manually close the window with the `window::close` command.

By default this is enabled.

## Trait Implementations

### `impl Clone for Settings`

#### `clone`

```rust
fn clone(&self) -> Settings
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Settings`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for Settings`

#### `default`

```rust
fn default() -> Self
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations

- `impl Freeze for Settings`
- `impl RefUnwindSafe for Settings`
- `impl Send for Settings`
- `impl Sync for Settings`
- `impl Unpin for Settings`
- `impl UnsafeUnpin for Settings`
- `impl UnwindSafe for Settings`
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
