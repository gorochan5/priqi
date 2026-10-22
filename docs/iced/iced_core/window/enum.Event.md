---
title: "Event"
kind: enum
path: "iced_core::window::Event"
crate: "iced_core"
module: "iced_core::window"
file: "iced_core/window/enum.Event.md"
docs_url: "https://docs.iced.rs/iced_core/window/enum.Event.html"
source_file: "src/iced_core/window/event.rs"
source_lines: "8-79"
---

# `iced_core::window::Event`

```rust
pub enum Event {
    Opened {
        position: Option<Point>,
        size: Size,
        scale_factor: f32,
    },
    Closed,
    Moved(Point),
    Resized(Size),
    Rescaled(f32),
    RedrawRequested(Instant),
    CloseRequested,
    Focused,
    Unfocused,
    FileHovered(PathBuf),
    FileDropped(PathBuf),
    FilesHoveredLeft,
}
```

A window-related event.

**Source:** [event.rs](../../src/iced_core/window/event.rs.md) (lines 8-79)

## Variants

### `Opened`

A window was opened.

**Fields**

- `position: Option<Point>` — The position of the opened window. This is relative to the top-left corner of the desktop the window is on, including virtual desktops. Refers to window’s “outer” position, or the window area, in logical pixels.

**Note**: Not available in Wayland.
- `size: Size` — The size of the created window. This is its “inner” size, or the size of the client area, in logical pixels.
- `scale_factor: f32` — The scale factor of the created window.

### `Closed`

A window was closed.

### `Moved(Point)`

A window was moved.

### `Resized(Size)`

A window was resized.

### `Rescaled(f32)`

A window changed its scale factor.

### `RedrawRequested(Instant)`

A window redraw was requested.

The [`Instant`](../time/struct.Instant.md) contains the current time.

### `CloseRequested`

The user has requested for the window to close.

### `Focused`

A window was focused.

### `Unfocused`

A window was unfocused.

### `FileHovered(PathBuf)`

A file is being hovered over the window.

When the user hovers multiple files at once, this event will be emitted for each file separately.

###### Platform-specific

- **Wayland:** Not implemented.

### `FileDropped(PathBuf)`

A file has been dropped into the window.

When the user drops multiple files at once, this event will be emitted for each file separately.

###### Platform-specific

- **Wayland:** Not implemented.

### `FilesHoveredLeft`

A file was hovered, but has exited the window.

There will be a single `FilesHoveredLeft` event triggered even if multiple files were hovered.

###### Platform-specific

- **Wayland:** Not implemented.

## Trait Implementations

### `impl Clone for Event`

#### `clone`

```rust
fn clone(&self) -> Event
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Event`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl PartialEq for Event`

#### `eq`

```rust
fn eq(&self, other: &Event) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl StructuralPartialEq for Event`

## Auto Trait Implementations

- `impl Freeze for Event`
- `impl RefUnwindSafe for Event`
- `impl Send for Event`
- `impl Sync for Event`
- `impl Unpin for Event`
- `impl UnsafeUnpin for Event`
- `impl UnwindSafe for Event`
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
