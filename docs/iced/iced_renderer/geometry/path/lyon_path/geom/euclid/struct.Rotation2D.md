---
title: "Rotation2D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Rotation2D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Rotation2D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Rotation2D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Rotation2D`

```rust
pub struct Rotation2D<T, Src, Dst> {
    pub angle: T,
    /* private fields */
}
```

A transform that can represent rotations in 2d, represented as an angle in radians.

## Fields

### `angle: T`

Angle in radians

## Methods

### `impl<T, Src, Dst> Rotation2D<T, Src, Dst>`

#### `new`

```rust
pub fn new(angle: Angle<T>) -> Rotation2D<T, Src, Dst>
```

Creates a rotation from an angle in radians.

#### `radians`

```rust
pub fn radians(angle: T) -> Rotation2D<T, Src, Dst>
```

Creates a rotation from an angle in radians.

#### `identity`

```rust
pub fn identity() -> Rotation2D<T, Src, Dst> where T: Zero,
```

Creates the identity rotation.

### `impl<T, Src, Dst> Rotation2D<T, Src, Dst> where T: Copy,`

#### `cast_unit`

```rust
pub fn cast_unit<Src2, Dst2>(&self) -> Rotation2D<T, Src2, Dst2>
```

Cast the unit, preserving the numeric value.

###### Example

```rust
enum Local {}
enum World {}

enum Local2 {}
enum World2 {}

let to_world: Rotation2D<_, Local, World> = Rotation2D::radians(42);

assert_eq!(to_world.angle, to_world.cast_unit::<Local2, World2>().angle);
```

#### `to_untyped`

```rust
pub fn to_untyped(&self) -> Rotation2D<T, UnknownUnit, UnknownUnit>
```

Drop the units, preserving only the numeric value.

###### Example

```rust
enum Local {}
enum World {}

let to_world: Rotation2D<_, Local, World> = Rotation2D::radians(42);

assert_eq!(to_world.angle, to_world.to_untyped().angle);
```

#### `from_untyped`

```rust
pub fn from_untyped( r: &Rotation2D<T, UnknownUnit, UnknownUnit>, ) -> Rotation2D<T, Src, Dst>
```

Tag a unitless value with units.

###### Example

```rust
use euclid::UnknownUnit;
enum Local {}
enum World {}

let rot: Rotation2D<_, UnknownUnit, UnknownUnit> = Rotation2D::radians(42);

assert_eq!(rot.angle, Rotation2D::<_, Local, World>::from_untyped(&rot).angle);
```

### `impl<T, Src, Dst> Rotation2D<T, Src, Dst> where T: Copy,`

#### `get_angle`

```rust
pub fn get_angle(&self) -> Angle<T>
```

Returns self.angle as a strongly typed `Angle<T>`.

### `impl<T, Src, Dst> Rotation2D<T, Src, Dst> where T: Real,`

#### `to_3d`

```rust
pub fn to_3d(&self) -> Rotation3D<T, Src, Dst>
```

Creates a 3d rotation (around the z axis) from this 2d rotation.

#### `inverse`

```rust
pub fn inverse(&self) -> Rotation2D<T, Dst, Src>
```

Returns the inverse of this rotation.

#### `then`

```rust
pub fn then<NewSrc>( &self, other: &Rotation2D<T, NewSrc, Src>, ) -> Rotation2D<T, NewSrc, Dst>
```

Returns a rotation representing the other rotation followed by this rotation.

#### `transform_point`

```rust
pub fn transform_point(&self, point: Point2D<T, Src>) -> Point2D<T, Dst>
```

Returns the given 2d point transformed by this rotation.

The input point must be use the unit Src, and the returned point has the unit Dst.

#### `transform_vector`

```rust
pub fn transform_vector(&self, vector: Vector2D<T, Src>) -> Vector2D<T, Dst>
```

Returns the given 2d vector transformed by this rotation.

The input point must be use the unit Src, and the returned point has the unit Dst.

### `impl<T, Src, Dst> Rotation2D<T, Src, Dst> where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Zero + Trig,`

#### `to_transform`

```rust
pub fn to_transform(&self) -> Transform2D<T, Src, Dst>
```

Returns the matrix representation of this rotation.

## Trait Implementations

### `impl<T, Src, Dst> ApproxEq<T> for Rotation2D<T, Src, Dst> where T: Copy + Neg<Output = T> + ApproxEq<T>,`

#### `approx_epsilon`

```rust
fn approx_epsilon() -> T
```

Default epsilon value

#### `approx_eq_eps`

```rust
fn approx_eq_eps(&self, other: &Rotation2D<T, Src, Dst>, eps: &T) -> bool
```

Returns `true` if this object is approximately equal to the other one, using a provided epsilon value.

#### `approx_eq`

```rust
fn approx_eq(&self, other: &Self) -> bool
```

Returns `true` if this object is approximately equal to the other one, using the [`approx_epsilon`](approxeq/trait.ApproxEq.md) epsilon value.

### `impl<T, Src, Dst> Clone for Rotation2D<T, Src, Dst> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Rotation2D<T, Src, Dst>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, Src, Dst> Debug for Rotation2D<T, Src, Dst> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, Src, Dst> From<Rotation2D<T, Src, Dst>> for Transform2D<T, Src, Dst> where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Zero + Trig,`

#### `from`

```rust
fn from(r: Rotation2D<T, Src, Dst>) -> Transform2D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> Hash for Rotation2D<T, Src, Dst> where T: Hash,`

#### `hash`

```rust
fn hash<H>(&self, h: &mut H) where H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl<T, Src, Dst> PartialEq for Rotation2D<T, Src, Dst> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Rotation2D<T, Src, Dst>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<S> Transformation<S> for Rotation2D<S, UnknownUnit, UnknownUnit> where S: Scalar,`

#### `transform_point`

```rust
fn transform_point(&self, p: Point2D<S, UnknownUnit>) -> Point2D<S, UnknownUnit>
```

#### `transform_vector`

```rust
fn transform_vector( &self, v: Vector2D<S, UnknownUnit>, ) -> Vector2D<S, UnknownUnit>
```

### `impl<T, Src, Dst> Copy for Rotation2D<T, Src, Dst> where T: Copy,`

### `impl<T, Src, Dst> Eq for Rotation2D<T, Src, Dst> where T: Eq,`

## Auto Trait Implementations

- `impl<T, Src, Dst> Freeze for Rotation2D<T, Src, Dst> where T: Freeze,`
- `impl<T, Src, Dst> RefUnwindSafe for Rotation2D<T, Src, Dst> where T: RefUnwindSafe, Src: RefUnwindSafe, Dst: RefUnwindSafe,`
- `impl<T, Src, Dst> Send for Rotation2D<T, Src, Dst> where T: Send, Src: Send, Dst: Send,`
- `impl<T, Src, Dst> Sync for Rotation2D<T, Src, Dst> where T: Sync, Src: Sync, Dst: Sync,`
- `impl<T, Src, Dst> Unpin for Rotation2D<T, Src, Dst> where T: Unpin, Src: Unpin, Dst: Unpin,`
- `impl<T, Src, Dst> UnsafeUnpin for Rotation2D<T, Src, Dst> where T: UnsafeUnpin,`
- `impl<T, Src, Dst> UnwindSafe for Rotation2D<T, Src, Dst> where T: UnwindSafe, Src: UnwindSafe, Dst: UnwindSafe,`
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
