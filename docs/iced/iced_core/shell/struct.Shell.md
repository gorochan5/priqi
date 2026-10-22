---
title: "Shell"
kind: struct
path: "iced_core::shell::Shell"
crate: "iced_core"
module: "iced_core::shell"
file: "iced_core/shell/struct.Shell.md"
docs_url: "https://docs.iced.rs/iced_core/shell/struct.Shell.html"
source_file: "src/iced_core/shell.rs"
source_lines: "18-27"
---

# `iced_core::shell::Shell`

```rust
pub struct Shell<'a, Message> { /* private fields */ }
```

A connection to the state of a shell.

A [`Widget`](../widget/trait.Widget.md) can leverage a [`Shell`](struct.Shell.md) to trigger changes in an application, like publishing messages or invalidating the current layout.

**Source:** [shell.rs](../../src/iced_core/shell.rs.md) (lines 18-27)

## Methods

### `impl<'a, Message> Shell<'a, Message>`

#### `new`

```rust
pub fn new( window: &'a dyn Window, waker: Waker, bus: &'a mut Bus<Message>, ) -> Self
```

Creates a new [`Shell`](struct.Shell.md) with the provided buffer of messages.

#### `local`

```rust
pub fn local<'b, A>(&self, bus: &'b mut Bus<A>) -> Shell<'b, A> where 'a: 'b,
```

Creates a new [`Shell`](struct.Shell.md) from the current one with the given list of local messages.

#### `window`

```rust
pub fn window(&self) -> &'a dyn Window
```

Returns the [`Window`](../window/trait.Window.md) of the [`Shell`](struct.Shell.md).

#### `waker`

```rust
pub fn waker(&self) -> &Waker
```

Returns the [`Waker`](struct.Waker.md) of the [`Shell`](struct.Shell.md).

#### `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

Returns true if the [`Shell`](struct.Shell.md) contains no published messages

#### `publish`

```rust
pub fn publish(&mut self, message: Message)
```

Publish the given `Message` for an application to process it.

#### `publish_and_track`

```rust
pub fn publish_and_track(&mut self, message: Message) -> Tracking
```

Publish the given `Message` for an application to process it.

The returned [`Tracking`](struct.Tracking.md) can be used to determine if the message was processed.

#### `forward`

```rust
pub fn forward(&mut self, message: Message, receipt: Receipt)
```

Forwards the given `Message` and fulfills the given [`Receipt`](struct.Receipt.md) once processed.

#### `capture_event`

```rust
pub fn capture_event(&mut self)
```

Marks the current event as captured. Prevents “event bubbling”.

A widget should capture an event when no ancestor should handle it.

#### `event_status`

```rust
pub fn event_status(&self) -> Status
```

Returns the current [`event::Status`](../event/enum.Status.md) of the [`Shell`](struct.Shell.md).

#### `is_event_captured`

```rust
pub fn is_event_captured(&self) -> bool
```

Returns whether the current event has been captured.

#### `request_redraw`

```rust
pub fn request_redraw(&mut self)
```

Requests a new frame to be drawn as soon as possible.

#### `request_redraw_at`

```rust
pub fn request_redraw_at(&mut self, redraw_request: impl Into<RedrawRequest>)
```

Requests a new frame to be drawn at the given [`window::RedrawRequest`](../window/enum.RedrawRequest.md).

#### `redraw_request`

```rust
pub fn redraw_request(&self) -> RedrawRequest
```

Returns the request a redraw should happen, if any.

#### `replace_redraw_request`

```rust
pub fn replace_redraw_request(shell: &mut Self, redraw_request: RedrawRequest)
```

Replaces the redraw request of the [`Shell`](struct.Shell.md); without conflict resolution.

This is useful if you want to overwrite the redraw request to a previous value. Since it’s a fairly advanced use case and should rarely be used, it is a static method.

#### `read_clipboard`

```rust
pub fn read_clipboard(&mut self, kind: Kind)
```

Requests the runtime to read the clipboard contents expecting the given [`clipboard::Kind`](../clipboard/enum.Kind.md).

The runtime will produce a [`clipboard::Event::Read`](../clipboard/enum.Event.md) when the contents have been read.

#### `write_clipboard`

```rust
pub fn write_clipboard(&mut self, content: impl Into<Content>)
```

Requests the runtime to write the given [`clipboard::Content`](../clipboard/enum.Content.md) to the clipboard.

The runtime will produce a [`clipboard::Event::Written`](../clipboard/enum.Event.md) when the contents have been written.

#### `clipboard_mut`

```rust
pub fn clipboard_mut(&mut self) -> &mut Clipboard
```

Returns the [`Clipboard`](../clipboard/struct.Clipboard.md) requests of the [`Shell`](struct.Shell.md), mutably.

#### `request_input_method`

```rust
pub fn request_input_method<T: AsRef<str>>(&mut self, ime: &InputMethod<T>)
```

Requests the current [`InputMethod`](../input_method/enum.InputMethod.md) strategy.

**Important**: This request will only be honored by the [`Shell`](struct.Shell.md) only during a [`window::Event::RedrawRequested`](../window/enum.Event.md).

#### `input_method`

```rust
pub fn input_method(&self) -> &InputMethod
```

Returns the current [`InputMethod`](../input_method/enum.InputMethod.md) strategy.

#### `input_method_mut`

```rust
pub fn input_method_mut(&mut self) -> &mut InputMethod
```

Returns the current [`InputMethod`](../input_method/enum.InputMethod.md) strategy.

#### `invalidate_overlay`

```rust
pub fn invalidate_overlay(&mut self)
```

Invalidates the current application overlay.

The shell will recreate the application overlays.

#### `invalidate_layout`

```rust
pub fn invalidate_layout(&mut self)
```

Invalidates the current application layout.

The shell will relayout the application widgets.

#### `invalidate_layout_with`

```rust
pub fn invalidate_layout_with(&mut self, diff: Diff)
```

Invalidates the current application layout with the following [`Diff`](enum.Diff.md) strategy.

#### `invalidate_widgets`

```rust
pub fn invalidate_widgets(&mut self)
```

Invalidates the current application widgets.

The shell will rebuild and relayout the widget tree.

#### `invalidate`

```rust
pub fn invalidate(&mut self, invalidation: Invalidation)
```

Sets the [`Invalidation`](enum.Invalidation.md) of the [`Shell`](struct.Shell.md).

#### `invalidation`

```rust
pub fn invalidation(&self) -> Invalidation
```

Returns the [`Invalidation`](enum.Invalidation.md) of the [`Shell`](struct.Shell.md).

#### `merge`

```rust
pub fn merge<B>(&mut self, other: Shell<'_, B>, f: impl Fn(B) -> Message)
```

Merges the current [`Shell`](struct.Shell.md) with another one by applying the given function to the messages of the latter.

This method is useful for composition.

## Trait Implementations

### `impl<'a, Message: Debug> Debug for Shell<'a, Message>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl<'a, Message> !Freeze for Shell<'a, Message>`
- `impl<'a, Message> !RefUnwindSafe for Shell<'a, Message>`
- `impl<'a, Message> !Send for Shell<'a, Message>`
- `impl<'a, Message> !Sync for Shell<'a, Message>`
- `impl<'a, Message> Unpin for Shell<'a, Message>`
- `impl<'a, Message> UnsafeUnpin for Shell<'a, Message>`
- `impl<'a, Message> !UnwindSafe for Shell<'a, Message>`
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
