---
title: "Action"
kind: enum
path: "iced_runtime::window::Action"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/enum.Action.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/enum.Action.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "13-186"
---

# `iced_runtime::window::Action`

```rust
pub enum Action {
Show 37 variants    Open(Id, Settings, Sender<Id>),
    Close(Id),
    GetOldest(Sender<Option<Id>>),
    GetLatest(Sender<Option<Id>>),
    Drag(Id),
    DragResize(Id, Direction),
    Resize(Id, Size),
    GetSize(Id, Sender<Size>),
    GetMaximized(Id, Sender<bool>),
    Maximize(Id, bool),
    GetMinimized(Id, Sender<Option<bool>>),
    Minimize(Id, bool),
    GetPosition(Id, Sender<Option<Point>>),
    GetScaleFactor(Id, Sender<f32>),
    Move(Id, Point),
    SetMode(Id, Mode),
    GetMode(Id, Sender<Mode>),
    ToggleMaximize(Id),
    ToggleDecorations(Id),
    RequestUserAttention(Id, Option<UserAttention>),
    GainFocus(Id),
    SetLevel(Id, Level),
    ShowSystemMenu(Id),
    GetRawId(Id, Sender<u64>),
    SetIcon(Id, Icon),
    Run(Id, Box<dyn FnOnce(&dyn Window) + Send>),
    Screenshot(Id, Sender<Screenshot>),
    EnableMousePassthrough(Id),
    DisableMousePassthrough(Id),
    SetMinSize(Id, Option<Size>),
    SetMaxSize(Id, Option<Size>),
    SetResizable(Id, bool),
    SetResizeIncrements(Id, Option<Size>),
    GetMonitorSize(Id, Sender<Option<Size>>),
    SetAllowAutomaticTabbing(bool),
    RedrawAll,
    RelayoutAll,
}
```

An operation to be performed on some window.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 13-186)

## Variants

### `Open(Id, Settings, Sender<Id>)`

Open a new window with some [`Settings`].

### `Close(Id)`

Close the window and exits the application.

### `GetOldest(Sender<Option<Id>>)`

Gets the [`Id`] of the oldest window.

### `GetLatest(Sender<Option<Id>>)`

Gets the [`Id`] of the latest window.

### `Drag(Id)`

Move the window with the left mouse button until the button is released.

There’s no guarantee that this will work unless the left mouse button was pressed immediately before this function is called.

### `DragResize(Id, Direction)`

Resize the window with the left mouse button until the button is released.

There’s no guarantee that this will work unless the left mouse button was pressed immediately before this function is called.

### `Resize(Id, Size)`

Resize the window to the given logical dimensions.

### `GetSize(Id, Sender<Size>)`

Get the current logical dimensions of the window.

### `GetMaximized(Id, Sender<bool>)`

Get if the current window is maximized or not.

### `Maximize(Id, bool)`

Set the window to maximized or back

### `GetMinimized(Id, Sender<Option<bool>>)`

Get if the current window is minimized or not.

###### Platform-specific

- **Wayland:** Always `None`.

### `Minimize(Id, bool)`

Set the window to minimized or back

### `GetPosition(Id, Sender<Option<Point>>)`

Get the current logical coordinates of the window.

### `GetScaleFactor(Id, Sender<f32>)`

Get the current scale factor (DPI) of the window.

### `Move(Id, Point)`

Move the window to the given logical coordinates.

Unsupported on Wayland.

### `SetMode(Id, Mode)`

Change the [`Mode`] of the window.

### `GetMode(Id, Sender<Mode>)`

Get the current [`Mode`] of the window.

### `ToggleMaximize(Id)`

Toggle the window to maximized or back

### `ToggleDecorations(Id)`

Toggle whether window has decorations.

###### Platform-specific

- **X11:** Not implemented.
- **Web:** Unsupported.

### `RequestUserAttention(Id, Option<UserAttention>)`

Request user attention to the window, this has no effect if the application is already focused. How requesting for user attention manifests is platform dependent, see [`UserAttention`] for details.

Providing `None` will unset the request for user attention. Unsetting the request for user attention might not be done automatically by the WM when the window receives input.

###### Platform-specific

- **iOS / Android / Web:** Unsupported.
- **macOS:** `None` has no effect.
- **X11:** Requests for user attention must be manually cleared.
- **Wayland:** Requires `xdg_activation_v1` protocol, `None` has no effect.

### `GainFocus(Id)`

Bring the window to the front and sets input focus. Has no effect if the window is already in focus, minimized, or not visible.

This method steals input focus from other applications. Do not use this method unless you are certain that’s what the user wants. Focus stealing can cause an extremely disruptive user experience.

###### Platform-specific

- **Web / Wayland:** Unsupported.

### `SetLevel(Id, Level)`

Change the window [`Level`].

### `ShowSystemMenu(Id)`

Show the system menu at cursor position.

###### Platform-specific

Android / iOS / macOS / Orbital / Web / X11: Unsupported.

### `GetRawId(Id, Sender<u64>)`

Get the raw identifier unique to the window.

### `SetIcon(Id, Icon)`

Change the window [`Icon`].

On Windows and X11, this is typically the small icon in the top-left corner of the titlebar.

###### Platform-specific

- **Web / Wayland / macOS:** Unsupported.
- **Windows:** Sets `ICON_SMALL`. The base size for a window icon is 16x16, but it’s recommended to account for screen scaling and pick a multiple of that, i.e. 32x32.
- **X11:** Has no universal guidelines for icon sizes, so you’re at the whims of the WM. That said, it’s usually in the same ballpark as on Windows.

### `Run(Id, Box<dyn FnOnce(&dyn Window) + Send>)`

Runs the closure with a reference to the [`Window`] with the given [`Id`].

### `Screenshot(Id, Sender<Screenshot>)`

Screenshot the viewport of the window.

### `EnableMousePassthrough(Id)`

Enable mouse passthrough for the given window.

This disables mouse events for the window and passes mouse events through to whatever window is underneath.

### `DisableMousePassthrough(Id)`

Disable mouse passthrough for the given window.

This enables mouse events for the window and stops mouse events from being passed to whatever is underneath.

### `SetMinSize(Id, Option<Size>)`

Set the minimum inner window size.

### `SetMaxSize(Id, Option<Size>)`

Set the maximum inner window size.

### `SetResizable(Id, bool)`

Set the window to be resizable or not.

### `SetResizeIncrements(Id, Option<Size>)`

Set the window size increment.

### `GetMonitorSize(Id, Sender<Option<Size>>)`

Get the logical dimensions of the monitor containing the window with the given [`Id`].

### `SetAllowAutomaticTabbing(bool)`

Set whether the system can automatically organize windows into tabs.

See [https://developer.apple.com/documentation/appkit/nswindow/1646657-allowsautomaticwindowtabbing](https://developer.apple.com/documentation/appkit/nswindow/1646657-allowsautomaticwindowtabbing)

### `RedrawAll`

Redraw all the windows.

### `RelayoutAll`

Recompute the layouts of all the windows.

## Auto Trait Implementations

- `impl Freeze for Action`
- `impl !RefUnwindSafe for Action`
- `impl Send for Action`
- `impl !Sync for Action`
- `impl Unpin for Action`
- `impl UnsafeUnpin for Action`
- `impl !UnwindSafe for Action`
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

### `impl<T> MaybeSend for T where T: Send,`
