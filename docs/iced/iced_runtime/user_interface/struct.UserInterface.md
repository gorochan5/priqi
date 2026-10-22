---
title: "UserInterface"
kind: struct
path: "iced_runtime::user_interface::UserInterface"
crate: "iced_runtime"
module: "iced_runtime::user_interface"
file: "iced_runtime/user_interface/struct.UserInterface.md"
docs_url: "https://docs.iced.rs/iced_runtime/user_interface/struct.UserInterface.html"
source_file: "src/iced_runtime/user_interface.rs"
source_lines: "29-36"
---

# `iced_runtime::user_interface::UserInterface`

```rust
pub struct UserInterface<'a, Message, Theme, Renderer> { /* private fields */ }
```

A set of interactive graphical elements with a specific [`Layout`].

It can be updated and drawn.

Iced tries to avoid dictating how to write your event loop. You are in charge of using this type in your system in any way you want.

## Example

The [`integration`](https://github.com/iced-rs/iced/tree/master/examples/integration) example uses a [`UserInterface`](struct.UserInterface.md) to integrate Iced in an existing graphical application.

**Source:** [user_interface.rs](../../src/iced_runtime/user_interface.rs.md) (lines 29-36)

## Methods

### `impl<'a, Message, Theme, Renderer> UserInterface<'a, Message, Theme, Renderer> where Message: 'a, Theme: 'a, Renderer: Renderer + 'a,`

#### `build`

```rust
pub fn build( root: impl Widget<Message, Theme, Renderer> + 'a, bounds: Size, cache: Cache, renderer: &mut Renderer, ) -> Self
```

Builds a user interface for a [`Widget`].

It is able to avoid expensive computations when using a [`Cache`](struct.Cache.md) obtained from a previous instance of a [`UserInterface`](struct.UserInterface.md).

###### Example

Imagine we want to build a [`UserInterface`](struct.UserInterface.md) for [the counter example that we previously wrote](index.md). Here is naive way to set up our application loop:

```rust
use iced_runtime::core::shell;
use iced_runtime::core::window;
use iced_runtime::core::Size;
use iced_runtime::user_interface::{self, UserInterface};
use iced_wgpu::Renderer;

// Initialization
let mut counter = Counter::new();
let mut cache = user_interface::Cache::new();
let mut renderer = Renderer::default();
let mut window = window::Headless; // This should be a proper window, like a `winit` one
let mut waker = shell::Waker::noop();
let mut window_size = Size::new(1024.0, 768.0);

// Application loop
loop {
    // Process system events here...

    // Build the user interface
    let user_interface = UserInterface::build(
        counter.view(),
        window_size,
        cache,
        &mut renderer,
    );

    // Update and draw the user interface here...
    // ...

    // Obtain the cache for the next iteration
    cache = user_interface.into_cache();
}
```

#### `update`

```rust
pub fn update( &mut self, window: &dyn Window, waker: &Waker, events: &[Event], cursor: Cursor, renderer: &mut Renderer, messages: &mut Bus<Message>, ) -> (State, Vec<Status>)
```

Updates the [`UserInterface`](struct.UserInterface.md) by processing each provided [`Event`].

It returns **messages** that may have been produced as a result of user interactions. You should feed these to your **update logic**.

###### Example

Let’s allow our [counter](index.md) to change state by completing the previous example:

```rust
use iced_runtime::core::mouse;
use iced_runtime::core::shell;
use iced_runtime::core::window;
use iced_runtime::core::Size;
use iced_runtime::user_interface::{self, UserInterface};
use iced_wgpu::Renderer;

let mut counter = Counter::new();
let mut cache = user_interface::Cache::new();
let mut renderer = Renderer::default();
let mut window = window::Headless; // This should be a proper window, like a `winit` one
let mut waker = shell::Waker::noop();
let mut window_size = Size::new(1024.0, 768.0);
let mut cursor = mouse::Cursor::default();

// Initialize our event storage
let mut events = Vec::new();
let mut messages = shell::Bus::new();

loop {
    // Obtain system events...

    let mut user_interface = UserInterface::build(
        counter.view(),
        window_size,
        cache,
        &mut renderer,
    );

    // Update the user interface
    let (state, event_statuses) = user_interface.update(
        &window,
        &waker,
        &events,
        cursor,
        &mut renderer,
        &mut messages
    );

    cache = user_interface.into_cache();

    // Process the produced messages
    for (message, _receipt) in messages.drain() {
        counter.update(message);
    }
}
```

#### `draw`

```rust
pub fn draw( &mut self, renderer: &mut Renderer, theme: &Theme, style: &Style, cursor: Cursor, )
```

Draws the [`UserInterface`](struct.UserInterface.md) with the provided `Renderer`.

It returns the current [`mouse::Interaction`]. You should update the icon of the mouse cursor accordingly in your system.

###### Example

We can finally draw our [counter](index.md) by completing the last example:

```rust
use iced_runtime::core::mouse;
use iced_runtime::core::renderer;
use iced_runtime::core::shell;
use iced_runtime::core::window;
use iced_runtime::core::{Element, Size};
use iced_runtime::user_interface::{self, UserInterface};
use iced_wgpu::{Renderer, Theme};

let mut counter = Counter::new();
let mut cache = user_interface::Cache::new();
let mut renderer = Renderer::default();
let mut window = window::Headless; // This should be a proper window, like a `winit` one
let mut waker = shell::Waker::noop();
let mut window_size = Size::new(1024.0, 768.0);
let mut cursor = mouse::Cursor::default();
let mut events = Vec::new();
let mut messages = shell::Bus::new();
let mut theme = Theme::default();

loop {
    // Obtain system events...

    let mut user_interface = UserInterface::build(
        counter.view(),
        window_size,
        cache,
        &mut renderer,
    );

    // Update the user interface
    let event_statuses = user_interface.update(
        &window,
        &waker,
        &events,
        cursor,
        &mut renderer,
        &mut messages
    );

    // Draw the user interface
    let mouse_interaction = user_interface.draw(&mut renderer, &theme, &renderer::Style::default(), cursor);

    cache = user_interface.into_cache();

    for (message, _receipt) in messages.drain() {
        counter.update(message);
    }

    // Update mouse cursor icon...
    // Flush rendering operations...
}
```

#### `operate`

```rust
pub fn operate(&mut self, renderer: &Renderer, operation: &mut dyn Operation)
```

Applies a [`widget::Operation`] to the [`UserInterface`](struct.UserInterface.md).

#### `relayout`

```rust
pub fn relayout(self, bounds: Size, renderer: &mut Renderer) -> Self
```

Relayouts and returns a new [`UserInterface`](struct.UserInterface.md) using the provided bounds.

#### `into_cache`

```rust
pub fn into_cache(self) -> Cache
```

Extract the [`Cache`](struct.Cache.md) of the [`UserInterface`](struct.UserInterface.md), consuming it in the process.

## Auto Trait Implementations

- `impl<'a, Message, Theme, Renderer> Freeze for UserInterface<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !RefUnwindSafe for UserInterface<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !Send for UserInterface<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !Sync for UserInterface<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> Unpin for UserInterface<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> UnsafeUnpin for UserInterface<'a, Message, Theme, Renderer>`
- `impl<'a, Message, Theme, Renderer> !UnwindSafe for UserInterface<'a, Message, Theme, Renderer>`
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
