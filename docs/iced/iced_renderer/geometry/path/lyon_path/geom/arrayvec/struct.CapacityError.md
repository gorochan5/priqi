---
title: "CapacityError"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::arrayvec::CapacityError"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::arrayvec"
file: "iced_renderer/geometry/path/lyon_path/geom/arrayvec/struct.CapacityError.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/arrayvec/struct.CapacityError.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::arrayvec::CapacityError`

```rust
pub struct CapacityError<T = ()> { /* private fields */ }
```

Error value indicating insufficient capacity

**Source:** [errors.rs.html#9](https://docs.rs/arrayvec/0.7/src/arrayvec/errors.rs.html#9) (lines 9)

## Methods

### `impl<T> CapacityError<T>`

#### `new`

```rust
pub const fn new(element: T) -> CapacityError<T>
```

Create a new `CapacityError` from `element`.

#### `element`

```rust
pub fn element(self) -> T
```

Extract the overflowing element

#### `simplify`

```rust
pub fn simplify(self) -> CapacityError
```

Convert into a `CapacityError` that does not carry an element.

## Trait Implementations

### `impl<T> Clone for CapacityError<T> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> CapacityError<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T> Debug for CapacityError<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T> Display for CapacityError<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl<T> Error for CapacityError<T> where T: Any,`

#### `source`

```rust
fn source(&self) -> Option<&(dyn Error + 'static)>
```

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

#### `description`

```rust
fn description(&self) -> &str
```

*👎Deprecated since 1.42.0: use the Display impl or to_string()*

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

#### `cause`

```rust
fn cause(&self) -> Option<&dyn Error>
```

*👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting*

#### `provide`

```rust
fn provide<'a>(&'a self, request: &mut Request<'a>)
```

*🔬This is a nightly-only experimental API. (error_generic_member_access)*

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

### `impl<T> Ord for CapacityError<T> where T: Ord,`

#### `cmp`

```rust
fn cmp(&self, other: &CapacityError<T>) -> Ordering
```

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

#### `max`

```rust
fn max(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

#### `min`

```rust
fn min(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

#### `clamp`

```rust
fn clamp(self, min: Self, max: Self) -> Self where Self: Sized,
```

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

### `impl<T> PartialEq for CapacityError<T> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &CapacityError<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T> PartialOrd for CapacityError<T> where T: PartialOrd,`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &CapacityError<T>) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, other: &Rhs) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, other: &Rhs) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `gt`

```rust
fn gt(&self, other: &Rhs) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

#### `ge`

```rust
fn ge(&self, other: &Rhs) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

### `impl<T> Copy for CapacityError<T> where T: Copy,`

### `impl<T> Eq for CapacityError<T> where T: Eq,`

### `impl<T> StructuralPartialEq for CapacityError<T>`

## Auto Trait Implementations

- `impl<T> Freeze for CapacityError<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for CapacityError<T> where T: RefUnwindSafe,`
- `impl<T> Send for CapacityError<T> where T: Send,`
- `impl<T> Sync for CapacityError<T> where T: Sync,`
- `impl<T> Unpin for CapacityError<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for CapacityError<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for CapacityError<T> where T: UnwindSafe,`
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

### `impl<Q, K> Comparable<K> for Q where Q: Ord + ?Sized, K: Borrow<Q> + ?Sized,`

#### `compare`

```rust
fn compare(&self, key: &K) -> Ordering
```

Compare self to `key` and return their ordering.

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

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Checks if this value is equivalent to the given key. Read more

### `impl<Q, K> Equivalent<K> for Q where Q: Eq + ?Sized, K: Borrow<Q> + ?Sized,`

#### `equivalent`

```rust
fn equivalent(&self, key: &K) -> bool
```

Compare self to `key` and return `true` if they are equal.

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
