---
title: "Rotation3D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Rotation3D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Rotation3D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Rotation3D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Rotation3D`

```rust
pub struct Rotation3D<T, Src, Dst> {
    pub i: T,
    pub j: T,
    pub k: T,
    pub r: T,
    /* private fields */
}
```

A transform that can represent rotations in 3d, represented as a quaternion.

Most methods expect the quaternion to be normalized. When in doubt, use [`unit_quaternion`](struct.Rotation3D.md) instead of [`quaternion`](struct.Rotation3D.md) to create a rotation as the former will ensure that its result is normalized.

Some people use the `x, y, z, w` (or `w, x, y, z`) notations. The equivalence is as follows: `x -> i`, `y -> j`, `z -> k`, `w -> r`. The memory layout of this type corresponds to the `x, y, z, w` notation

## Fields

### `i: T`

Component multiplied by the imaginary number `i`.

### `j: T`

Component multiplied by the imaginary number `j`.

### `k: T`

Component multiplied by the imaginary number `k`.

### `r: T`

The real part.

## Methods

### `impl<T, Src, Dst> Rotation3D<T, Src, Dst>`

#### `quaternion`

```rust
pub fn quaternion(a: T, b: T, c: T, r: T) -> Rotation3D<T, Src, Dst>
```

Creates a rotation around from a quaternion representation.

The parameters are a, b, c and r compose the quaternion `a*i + b*j + c*k + r` where `a`, `b` and `c` describe the vector part and the last parameter `r` is the real part.

The resulting quaternion is not necessarily normalized. See [`unit_quaternion`](struct.Rotation3D.md).

#### `identity`

```rust
pub fn identity() -> Rotation3D<T, Src, Dst> where T: Zero + One,
```

Creates the identity rotation.

### `impl<T, Src, Dst> Rotation3D<T, Src, Dst> where T: Copy,`

#### `vector_part`

```rust
pub fn vector_part(&self) -> Vector3D<T, UnknownUnit>
```

Returns the vector part (i, j, k) of this quaternion.

The vector part represents the axis of the rotation.

#### `cast_unit`

```rust
pub fn cast_unit<Src2, Dst2>(&self) -> Rotation3D<T, Src2, Dst2>
```

Cast the unit, preserving the numeric value.

###### Example

```rust
enum Local {}
enum World {}

enum Local2 {}
enum World2 {}

let to_world: Rotation3D<_, Local, World> = Rotation3D::quaternion(1, 2, 3, 4);

assert_eq!(to_world.i, to_world.cast_unit::<Local2, World2>().i);
assert_eq!(to_world.j, to_world.cast_unit::<Local2, World2>().j);
assert_eq!(to_world.k, to_world.cast_unit::<Local2, World2>().k);
assert_eq!(to_world.r, to_world.cast_unit::<Local2, World2>().r);
```

#### `to_untyped`

```rust
pub fn to_untyped(&self) -> Rotation3D<T, UnknownUnit, UnknownUnit>
```

Drop the units, preserving only the numeric value.

###### Example

```rust
enum Local {}
enum World {}

let to_world: Rotation3D<_, Local, World> = Rotation3D::quaternion(1, 2, 3, 4);

assert_eq!(to_world.i, to_world.to_untyped().i);
assert_eq!(to_world.j, to_world.to_untyped().j);
assert_eq!(to_world.k, to_world.to_untyped().k);
assert_eq!(to_world.r, to_world.to_untyped().r);
```

#### `from_untyped`

```rust
pub fn from_untyped( r: &Rotation3D<T, UnknownUnit, UnknownUnit>, ) -> Rotation3D<T, Src, Dst>
```

Tag a unitless value with units.

###### Example

```rust
use euclid::UnknownUnit;
enum Local {}
enum World {}

let rot: Rotation3D<_, UnknownUnit, UnknownUnit> = Rotation3D::quaternion(1, 2, 3, 4);

assert_eq!(rot.i, Rotation3D::<_, Local, World>::from_untyped(&rot).i);
assert_eq!(rot.j, Rotation3D::<_, Local, World>::from_untyped(&rot).j);
assert_eq!(rot.k, Rotation3D::<_, Local, World>::from_untyped(&rot).k);
assert_eq!(rot.r, Rotation3D::<_, Local, World>::from_untyped(&rot).r);
```

### `impl<T, Src, Dst> Rotation3D<T, Src, Dst> where T: Real,`

#### `unit_quaternion`

```rust
pub fn unit_quaternion(i: T, j: T, k: T, r: T) -> Rotation3D<T, Src, Dst>
```

Creates a rotation around from a quaternion representation and normalizes it.

The parameters are a, b, c and r compose the quaternion `a*i + b*j + c*k + r` before normalization, where `a`, `b` and `c` describe the vector part and the last parameter `r` is the real part.

#### `around_axis`

```rust
pub fn around_axis( axis: Vector3D<T, Src>, angle: Angle<T>, ) -> Rotation3D<T, Src, Dst>
```

Creates a rotation around a given axis.

#### `around_x`

```rust
pub fn around_x(angle: Angle<T>) -> Rotation3D<T, Src, Dst>
```

Creates a rotation around the x axis.

#### `around_y`

```rust
pub fn around_y(angle: Angle<T>) -> Rotation3D<T, Src, Dst>
```

Creates a rotation around the y axis.

#### `around_z`

```rust
pub fn around_z(angle: Angle<T>) -> Rotation3D<T, Src, Dst>
```

Creates a rotation around the z axis.

#### `euler`

```rust
pub fn euler( roll: Angle<T>, pitch: Angle<T>, yaw: Angle<T>, ) -> Rotation3D<T, Src, Dst>
```

Creates a rotation from Euler angles.

The rotations are applied in roll then pitch then yaw order.

- Roll (also called bank) is a rotation around the x axis.
- Pitch (also called bearing) is a rotation around the y axis.
- Yaw (also called heading) is a rotation around the z axis.

#### `inverse`

```rust
pub fn inverse(&self) -> Rotation3D<T, Dst, Src>
```

Returns the inverse of this rotation.

#### `norm`

```rust
pub fn norm(&self) -> T
```

Computes the norm of this quaternion.

#### `square_norm`

```rust
pub fn square_norm(&self) -> T
```

Computes the squared norm of this quaternion.

#### `normalize`

```rust
pub fn normalize(&self) -> Rotation3D<T, Src, Dst>
```

Returns a [unit quaternion](https://en.wikipedia.org/wiki/Quaternion#Unit_quaternion) from this one.

#### `is_normalized`

```rust
pub fn is_normalized(&self) -> bool where T: ApproxEq<T>,
```

Returns `true` if [norm](struct.Rotation3D.md) of this quaternion is (approximately) one.

#### `slerp`

```rust
pub fn slerp( &self, other: &Rotation3D<T, Src, Dst>, t: T, ) -> Rotation3D<T, Src, Dst> where T: ApproxEq<T>,
```

Spherical linear interpolation between this rotation and another rotation.

`t` is expected to be between zero and one.

#### `lerp`

```rust
pub fn lerp( &self, other: &Rotation3D<T, Src, Dst>, t: T, ) -> Rotation3D<T, Src, Dst>
```

Basic Linear interpolation between this rotation and another rotation.

#### `transform_point3d`

```rust
pub fn transform_point3d(&self, point: Point3D<T, Src>) -> Point3D<T, Dst> where T: ApproxEq<T>,
```

Returns the given 3d point transformed by this rotation.

The input point must be use the unit Src, and the returned point has the unit Dst.

#### `transform_point2d`

```rust
pub fn transform_point2d(&self, point: Point2D<T, Src>) -> Point2D<T, Dst> where T: ApproxEq<T>,
```

Returns the given 2d point transformed by this rotation then projected on the xy plane.

The input point must be use the unit Src, and the returned point has the unit Dst.

#### `transform_vector3d`

```rust
pub fn transform_vector3d(&self, vector: Vector3D<T, Src>) -> Vector3D<T, Dst> where T: ApproxEq<T>,
```

Returns the given 3d vector transformed by this rotation.

The input vector must be use the unit Src, and the returned point has the unit Dst.

#### `transform_vector2d`

```rust
pub fn transform_vector2d(&self, vector: Vector2D<T, Src>) -> Vector2D<T, Dst> where T: ApproxEq<T>,
```

Returns the given 2d vector transformed by this rotation then projected on the xy plane.

The input vector must be use the unit Src, and the returned point has the unit Dst.

#### `to_transform`

```rust
pub fn to_transform(&self) -> Transform3D<T, Src, Dst> where T: ApproxEq<T>,
```

Returns the matrix representation of this rotation.

#### `then`

```rust
pub fn then<NewDst>( &self, other: &Rotation3D<T, Dst, NewDst>, ) -> Rotation3D<T, Src, NewDst> where T: ApproxEq<T>,
```

Returns a rotation representing this rotation followed by the other rotation.

#### `get_angle`

```rust
pub fn get_angle(&self) -> Angle<T>
```

Returns the angle of rotation about the stored axis.

## Trait Implementations

### `impl<T, Src, Dst> ApproxEq<T> for Rotation3D<T, Src, Dst> where T: Copy + Neg<Output = T> + ApproxEq<T>,`

#### `approx_epsilon`

```rust
fn approx_epsilon() -> T
```

Default epsilon value

#### `approx_eq_eps`

```rust
fn approx_eq_eps(&self, other: &Rotation3D<T, Src, Dst>, eps: &T) -> bool
```

Returns `true` if this object is approximately equal to the other one, using a provided epsilon value.

#### `approx_eq`

```rust
fn approx_eq(&self, other: &Self) -> bool
```

Returns `true` if this object is approximately equal to the other one, using the [`approx_epsilon`](approxeq/trait.ApproxEq.md) epsilon value.

### `impl<T, Src, Dst> Clone for Rotation3D<T, Src, Dst> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Rotation3D<T, Src, Dst>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, Src, Dst> Debug for Rotation3D<T, Src, Dst> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, Src, Dst> From<Rotation3D<T, Src, Dst>> for RigidTransform3D<T, Src, Dst> where T: Real + ApproxEq<T>,`

#### `from`

```rust
fn from(rot: Rotation3D<T, Src, Dst>) -> RigidTransform3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Rotation3D<T, Src, Dst>> for Transform3D<T, Src, Dst> where T: Real + ApproxEq<T>,`

#### `from`

```rust
fn from(r: Rotation3D<T, Src, Dst>) -> Transform3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> Hash for Rotation3D<T, Src, Dst> where T: Hash,`

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

### `impl<T, Src, Dst> PartialEq for Rotation3D<T, Src, Dst> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Rotation3D<T, Src, Dst>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, Src, Dst> Copy for Rotation3D<T, Src, Dst> where T: Copy,`

### `impl<T, Src, Dst> Eq for Rotation3D<T, Src, Dst> where T: Eq,`

## Auto Trait Implementations

- `impl<T, Src, Dst> Freeze for Rotation3D<T, Src, Dst> where T: Freeze,`
- `impl<T, Src, Dst> RefUnwindSafe for Rotation3D<T, Src, Dst> where T: RefUnwindSafe, Src: RefUnwindSafe, Dst: RefUnwindSafe,`
- `impl<T, Src, Dst> Send for Rotation3D<T, Src, Dst> where T: Send, Src: Send, Dst: Send,`
- `impl<T, Src, Dst> Sync for Rotation3D<T, Src, Dst> where T: Sync, Src: Sync, Dst: Sync,`
- `impl<T, Src, Dst> Unpin for Rotation3D<T, Src, Dst> where T: Unpin, Src: Unpin, Dst: Unpin,`
- `impl<T, Src, Dst> UnsafeUnpin for Rotation3D<T, Src, Dst> where T: UnsafeUnpin,`
- `impl<T, Src, Dst> UnwindSafe for Rotation3D<T, Src, Dst> where T: UnwindSafe, Src: UnwindSafe, Dst: UnwindSafe,`
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
