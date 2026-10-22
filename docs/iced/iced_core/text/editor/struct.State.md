---
title: "State"
kind: struct
path: "iced_core::text::editor::State"
crate: "iced_core"
module: "iced_core::text::editor"
file: "iced_core/text/editor/struct.State.md"
docs_url: "https://docs.iced.rs/iced_core/text/editor/struct.State.html"
source_file: "src/iced_core/text/editor.rs"
source_lines: "315-321"
---

# `iced_core::text::editor::State`

```rust
pub struct State { /* private fields */ }
```

The internal state of an [`Editor`](trait.Editor.md).

**Source:** [editor.rs](../../../src/iced_core/text/editor.rs.md) (lines 315-321)

## Methods

### `impl State`

#### `new`

```rust
pub fn new() -> Self
```

Creates a new [`State`](struct.State.md).

#### `update`

```rust
pub fn update<Message>( &mut self, editor: &impl Editor, event: &Event, bounds: Rectangle, padding: Padding, cursor: Cursor, key_binding: impl Fn(KeyPress) -> Option<Binding<Message>>, ) -> Option<Update<Message>>
```

Updates the [`State`](struct.State.md) for the given [`Editor`](trait.Editor.md) and returns any relevant [`Update`](enum.Update.md).

#### `input_method`

```rust
pub fn input_method<'a>( &'a self, editor: &impl Editor, position: Point, ) -> InputMethod<&'a str>
```

Returns the current [`InputMethod`](../../input_method/enum.InputMethod.md) of the [`State`](struct.State.md) for the given [`Editor`](trait.Editor.md).

#### `draw`

```rust
pub fn draw<Renderer: Renderer>( &self, editor: &Renderer::Editor, renderer: &mut Renderer, position: Point, clip_bounds: Rectangle, style: Style, )
```

Draws the given [`Editor`](trait.Editor.md) with the current [`State`](struct.State.md).

#### `is_cursor_visible`

```rust
pub fn is_cursor_visible(&self) -> bool
```

Returns whether the cursor of the [`Editor`](trait.Editor.md) is visible.

### `impl State`

#### `is_focused`

```rust
pub fn is_focused(&self) -> bool
```

Returns whether the [`Editor`](trait.Editor.md) is currently focused or not.

## Trait Implementations

### `impl Clone for State`

#### `clone`

```rust
fn clone(&self) -> State
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for State`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Default for State`

#### `default`

```rust
fn default() -> State
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl Focusable for State`

#### `is_focused`

```rust
fn is_focused(&self) -> bool
```

Returns whether the widget is focused or not.

#### `focus`

```rust
fn focus(&mut self)
```

Focuses the widget.

#### `unfocus`

```rust
fn unfocus(&mut self)
```

Unfocuses the widget.

## Auto Trait Implementations

- `impl Freeze for State`
- `impl RefUnwindSafe for State`
- `impl Send for State`
- `impl Sync for State`
- `impl Unpin for State`
- `impl UnsafeUnpin for State`
- `impl UnwindSafe for State`
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
