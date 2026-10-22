---
title: "PathSlice"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::PathSlice"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path"
file: "iced_renderer/geometry/path/lyon_path/struct.PathSlice.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/struct.PathSlice.html"
---

# `iced_renderer::geometry::path::lyon_path::PathSlice`

```rust
pub struct PathSlice<'l> { /* private fields */ }
```

A view on a `Path`.

## Methods

### `impl<'l> PathSlice<'l>`

#### `first_endpoint`

```rust
pub fn first_endpoint(&self) -> Option<(Point2D<f32, UnknownUnit>, &'l [f32])>
```

#### `last_endpoint`

```rust
pub fn last_endpoint(&self) -> Option<(Point2D<f32, UnknownUnit>, &'l [f32])>
```

#### `iter`

```rust
pub fn iter<'a>(&'a self) -> Iter<'l> ⓘ
```

Iterates over the path.

#### `id_iter`

```rust
pub fn id_iter(&self) -> IdIter<'_> ⓘ
```

Iterates over the endpoint and control point ids of the `Path`.

#### `iter_with_attributes`

```rust
pub fn iter_with_attributes(&self) -> IterWithAttributes<'_> ⓘ
```

Iterates over the entire `Path` with custom attributes.

#### `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

#### `attributes`

```rust
pub fn attributes(&self, endpoint: EndpointId) -> &'l [f32]
```

Returns a slice over an endpoint’s custom attributes.

#### `reversed`

```rust
pub fn reversed(&self) -> NoAttributes<Reversed<'_>> ⓘ
```

Returns a reversed version of this path in the form of an iterator

## Trait Implementations

### `impl<'l> AttributeStore for PathSlice<'l>`

#### `get`

```rust
fn get(&self, id: EndpointId) -> &'l [f32]
```

Returns the endpoint’s custom attributes as a slice of 32 bits floats. [Read more](trait.AttributeStore.md)

#### `num_attributes`

```rust
fn num_attributes(&self) -> usize
```

Returns the number of float attributes per endpoint. [Read more](trait.AttributeStore.md)

### `impl<'l> Clone for PathSlice<'l>`

#### `clone`

```rust
fn clone(&self) -> PathSlice<'l>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<'l> Debug for PathSlice<'l>`

#### `fmt`

```rust
fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<'l> From<&'l Path> for PathSlice<'l>`

#### `from`

```rust
fn from(path: &'l Path) -> PathSlice<'l>
```

Converts to this type from the input type.

### `impl<'l> FromIterator<PathSlice<'l>> for PathBuffer`

#### `from_iter`

```rust
fn from_iter<T>(iter: T) -> PathBuffer where T: IntoIterator<Item = PathSlice<'l>>,
```

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

### `impl<'l> Index<ControlPointId> for PathSlice<'l>`

#### `Output`

```rust
type Output = Point2D<f32, UnknownUnit>
```

The returned type after indexing.

#### `index`

```rust
fn index(&self, id: ControlPointId) -> &Point2D<f32, UnknownUnit>
```

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

### `impl<'l> Index<EndpointId> for PathSlice<'l>`

#### `Output`

```rust
type Output = Point2D<f32, UnknownUnit>
```

The returned type after indexing.

#### `index`

```rust
fn index(&self, id: EndpointId) -> &Point2D<f32, UnknownUnit>
```

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

### `impl<'l, 'a> IntoIterator for &'a PathSlice<'l>`

#### `Item`

```rust
type Item = Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = Iter<'l>
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> Iter<'l> ⓘ
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

### `impl<'l> IntoIterator for PathSlice<'l>`

#### `Item`

```rust
type Item = Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = Iter<'l>
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> Iter<'l> ⓘ
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

### `impl<'l> PositionStore for PathSlice<'l>`

#### `get_endpoint`

```rust
fn get_endpoint(&self, id: EndpointId) -> Point2D<f32, UnknownUnit>
```

#### `get_control_point`

```rust
fn get_control_point(&self, id: ControlPointId) -> Point2D<f32, UnknownUnit>
```

### `impl<'l> Copy for PathSlice<'l>`

## Auto Trait Implementations

- `impl<'l> Freeze for PathSlice<'l>`
- `impl<'l> RefUnwindSafe for PathSlice<'l>`
- `impl<'l> Send for PathSlice<'l>`
- `impl<'l> Sync for PathSlice<'l>`
- `impl<'l> Unpin for PathSlice<'l>`
- `impl<'l> UnsafeUnpin for PathSlice<'l>`
- `impl<'l> UnwindSafe for PathSlice<'l>`
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

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
