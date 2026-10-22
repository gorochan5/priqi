---
title: "State"
kind: struct
path: "iced::widget::pane_grid::State"
crate: "iced"
module: "iced::widget::pane_grid"
file: "iced/widget/pane_grid/struct.State.md"
docs_url: "https://docs.iced.rs/iced/widget/pane_grid/struct.State.html"
source_file: "src/iced_widget/pane_grid/state.rs"
source_lines: "23"
---

# `iced::widget::pane_grid::State`

```rust
pub struct State<T> {
    pub panes: BTreeMap<Pane, T>,
    pub internal: Internal,
}
```

The state of a [`PaneGrid`](../struct.PaneGrid.md).

It keeps track of the state of each [`Pane`](struct.Pane.md) and the position of each [`Split`](struct.Split.md).

The [`State`](struct.State.md) needs to own any mutable contents a [`Pane`](struct.Pane.md) may need. This is why this struct is generic over the type `T`. Values of this type are provided to the view function of [`PaneGrid::new`](../struct.PaneGrid.md) for displaying each [`Pane`](struct.Pane.md).

**Source:** [state.rs](../../../src/iced_widget/pane_grid/state.rs.md) (lines 23)

## Fields

### `panes: BTreeMap<Pane, T>`

The panes of the [`PaneGrid`](../struct.PaneGrid.md).

### `internal: Internal`

The internal state of the [`PaneGrid`](../struct.PaneGrid.md).

## Methods

### `impl<T> State<T>`

#### `new`

```rust
pub fn new(first_pane_state: T) -> (State<T>, Pane)
```

Creates a new [`State`](struct.State.md), initializing the first pane with the provided state.

Alongside the [`State`](struct.State.md), it returns the first [`Pane`](struct.Pane.md) identifier.

#### `with_configuration`

```rust
pub fn with_configuration(config: impl Into<Configuration<T>>) -> State<T>
```

Creates a new [`State`](struct.State.md) with the given [`Configuration`](enum.Configuration.md).

#### `len`

```rust
pub fn len(&self) -> usize
```

Returns the total amount of panes in the [`State`](struct.State.md).

#### `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

Returns `true` if the amount of panes in the [`State`](struct.State.md) is 0.

#### `get`

```rust
pub fn get(&self, pane: Pane) -> Option<&T>
```

Returns the internal state of the given [`Pane`](struct.Pane.md), if it exists.

#### `get_mut`

```rust
pub fn get_mut(&mut self, pane: Pane) -> Option<&mut T>
```

Returns the internal state of the given [`Pane`](struct.Pane.md) with mutability, if it exists.

#### `iter`

```rust
pub fn iter(&self) -> impl Iterator<Item = (&Pane, &T)>
```

Returns an iterator over all the panes of the [`State`](struct.State.md), alongside its internal state.

#### `iter_mut`

```rust
pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Pane, &mut T)>
```

Returns a mutable iterator over all the panes of the [`State`](struct.State.md), alongside its internal state.

#### `layout`

```rust
pub fn layout(&self) -> &Node
```

Returns the layout of the [`State`](struct.State.md).

#### `adjacent`

```rust
pub fn adjacent(&self, pane: Pane, direction: Direction) -> Option<Pane>
```

Returns the adjacent [`Pane`](struct.Pane.md) of another [`Pane`](struct.Pane.md) in the given direction, if there is one.

#### `split`

```rust
pub fn split( &mut self, axis: Axis, pane: Pane, state: T, ) -> Option<(Pane, Split)>
```

Splits the given [`Pane`](struct.Pane.md) into two in the given [`Axis`](enum.Axis.md) and initializing the new [`Pane`](struct.Pane.md) with the provided internal state.

#### `split_with`

```rust
pub fn split_with(&mut self, target: Pane, pane: Pane, region: Region)
```

Split a target [`Pane`](struct.Pane.md) with a given [`Pane`](struct.Pane.md) on a given [`Region`](enum.Region.md).

Panes will be swapped by default for [`Region::Center`](enum.Region.md).

#### `drop`

```rust
pub fn drop(&mut self, pane: Pane, target: Target)
```

Drops the given [`Pane`](struct.Pane.md) into the provided [`Target`](enum.Target.md).

#### `move_to_edge`

```rust
pub fn move_to_edge(&mut self, pane: Pane, edge: Edge)
```

Move [`Pane`](struct.Pane.md) to an [`Edge`](enum.Edge.md) of the [`PaneGrid`](../struct.PaneGrid.md).

#### `swap`

```rust
pub fn swap(&mut self, a: Pane, b: Pane)
```

Swaps the position of the provided panes in the [`State`](struct.State.md).

If you want to swap panes on drag and drop in your [`PaneGrid`](../struct.PaneGrid.md), you will need to call this method when handling a [`DragEvent`](enum.DragEvent.md).

#### `resize`

```rust
pub fn resize(&mut self, split: Split, ratio: f32)
```

Resizes two panes by setting the position of the provided [`Split`](struct.Split.md).

The ratio is a value in [0, 1], representing the exact position of a [`Split`](struct.Split.md) between two panes.

If you want to enable resize interactions in your [`PaneGrid`](../struct.PaneGrid.md), you will need to call this method when handling a [`ResizeEvent`](struct.ResizeEvent.md).

#### `close`

```rust
pub fn close(&mut self, pane: Pane) -> Option<(T, Pane)>
```

Closes the given [`Pane`](struct.Pane.md) and returns its internal state and its closest sibling, if it exists.

#### `maximize`

```rust
pub fn maximize(&mut self, pane: Pane)
```

Maximize the given [`Pane`](struct.Pane.md). Only this pane will be rendered by the [`PaneGrid`](../struct.PaneGrid.md) until [`Self::restore()`](struct.State.md) is called.

#### `restore`

```rust
pub fn restore(&mut self)
```

Restore the currently maximized [`Pane`](struct.Pane.md) to it’s normal size. All panes will be rendered by the [`PaneGrid`](../struct.PaneGrid.md).

#### `maximized`

```rust
pub fn maximized(&self) -> Option<Pane>
```

Returns the maximized [`Pane`](struct.Pane.md) of the [`PaneGrid`](../struct.PaneGrid.md).

## Trait Implementations

### `impl<T> Clone for State<T> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> State<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T> Debug for State<T> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl<T> Freeze for State<T>`
- `impl<T> RefUnwindSafe for State<T> where T: RefUnwindSafe,`
- `impl<T> Send for State<T> where T: Send,`
- `impl<T> Sync for State<T> where T: Sync,`
- `impl<T> Unpin for State<T>`
- `impl<T> UnsafeUnpin for State<T>`
- `impl<T> UnwindSafe for State<T> where T: RefUnwindSafe,`
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

### `impl<T> Downcast<T> for T`

#### `downcast`

```rust
fn downcast(&self) -> &T
```

### `impl<T> Downcast for T where T: Any,`

#### `into_any`

```rust
fn into_any(self: Box<T>) -> Box<dyn Any>
```

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `into_any_rc`

```rust
fn into_any_rc(self: Rc<T>) -> Rc<dyn Any>
```

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `as_any`

```rust
fn as_any(&self) -> &(dyn Any + 'static)
```

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

#### `as_any_mut`

```rust
fn as_any_mut(&mut self) -> &mut (dyn Any + 'static)
```

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

### `impl<T> DowncastSync for T where T: Any + Send + Sync,`

#### `into_any_arc`

```rust
fn into_any_arc(self: Arc<T>) -> Arc<dyn Any + Sync + Send>
```

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

### `impl<T> From<T> for T`

#### `from`

```rust
fn from(t: T) -> T
```

Returns the argument unchanged.

### `impl<T> Instrument for T`

#### `instrument`

```rust
fn instrument(self, span: Span) -> Instrumented<Self>
```

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

#### `in_current_span`

```rust
fn in_current_span(self) -> Instrumented<Self>
```

Instruments this type with the current `Span`, returning an `Instrumented` wrapper. Read more

### `impl<T, U> Into<U> for T where U: From<T>,`

#### `into`

```rust
fn into(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

### `impl<State, Message> IntoBoot<State, Message> for State`

#### `into_boot`

```rust
fn into_boot(self) -> (State, Task<Message>)
```

Turns some type into the initial state of some [`Application`](../../application/struct.Application.md).

### `impl<T> IntoEither for T`

#### `into_either`

```rust
fn into_either(self, into_left: bool) -> Either<Self, Self>
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either)

#### `into_either_with`

```rust
fn into_either_with<F>(self, into_left: F) -> Either<Self, Self> where F: FnOnce(&Self) -> bool,
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either_with)

### `impl<T> Pointable for T`

#### `ALIGN`

```rust
const ALIGN: usize
```

The alignment of pointer.

#### `Init`

```rust
type Init = T
```

The type for initializers.

#### `init`

```rust
unsafe fn init(init: <T as Pointable>::Init) -> usize
```

Initializes a with the given initializer. Read more

#### `deref`

```rust
unsafe fn deref<'a>(ptr: usize) -> &'a T
```

Dereferences the given pointer. Read more

#### `deref_mut`

```rust
unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T
```

Mutably dereferences the given pointer. Read more

#### `drop`

```rust
unsafe fn drop(ptr: usize)
```

Drops the object pointed to by the given pointer. Read more

### `impl<T> Same for T`

#### `Output`

```rust
type Output = T
```

Should always be `Self`

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

### `impl<T> Upcast<T> for T`

#### `upcast`

```rust
fn upcast(&self) -> Option<&T>
```

### `impl<V, T> VZip<V> for T where V: MultiLane<T>,`

#### `vzip`

```rust
fn vzip(self) -> V
```

### `impl<T> WithSubscriber for T`

#### `with_subscriber`

```rust
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> where S: Into<Dispatch>,
```

Attaches the provided `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

#### `with_current_subscriber`

```rust
fn with_current_subscriber(self) -> WithDispatch<Self>
```

Attaches the current default `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeClone for T where T: Clone,`

### `impl<T> MaybeDebug for T where T: Debug,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
