---
title: "Translation3D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Translation3D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Translation3D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.Translation3D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Translation3D`

```rust
pub struct Translation3D<T, Src, Dst> {
    pub x: T,
    pub y: T,
    pub z: T,
    /* private fields */
}
```

A 3d transformation from a space to another that can only express translations.

The main benefit of this type over a [`Vector3D`](struct.Vector3D.md) is the ability to cast between source and destination spaces.

## Fields

### `x: T`

### `y: T`

### `z: T`

## Methods

### `impl<T, Src, Dst> Translation3D<T, Src, Dst>`

#### `new`

```rust
pub const fn new(x: T, y: T, z: T) -> Translation3D<T, Src, Dst>
```

#### `splat`

```rust
pub fn splat(v: T) -> Translation3D<T, Src, Dst> where T: Clone,
```

#### `identity`

```rust
pub fn identity() -> Translation3D<T, Src, Dst> where T: Zero,
```

Creates no-op translation (`x`, `y` and `z` is `zero()`).

#### `is_identity`

```rust
pub fn is_identity(&self) -> bool where T: Zero + PartialEq,
```

Check if translation does nothing (`x`, `y` and `z` is `zero()`).

```rust
use euclid::default::Translation3D;

assert_eq!(Translation3D::<f32>::identity().is_identity(), true);
assert_eq!(Translation3D::new(0, 0, 0).is_identity(), true);
assert_eq!(Translation3D::new(1, 0, 0).is_identity(), false);
assert_eq!(Translation3D::new(0, 1, 0).is_identity(), false);
assert_eq!(Translation3D::new(0, 0, 1).is_identity(), false);
```

#### `transform_size`

```rust
pub fn transform_size(self, s: Size2D<T, Src>) -> Size2D<T, Dst>
```

No-op, just cast the unit.

### `impl<T, Src, Dst> Translation3D<T, Src, Dst> where T: Copy,`

#### `to_vector`

```rust
pub fn to_vector(&self) -> Vector3D<T, Src>
```

Cast into a 3D vector.

#### `to_array`

```rust
pub fn to_array(&self) -> [T; 3]
```

Cast into an array with x, y and z.

#### `to_tuple`

```rust
pub fn to_tuple(&self) -> (T, T, T)
```

Cast into a tuple with x, y and z.

#### `to_untyped`

```rust
pub fn to_untyped(&self) -> Translation3D<T, UnknownUnit, UnknownUnit>
```

Drop the units, preserving only the numeric value.

#### `from_untyped`

```rust
pub fn from_untyped( t: &Translation3D<T, UnknownUnit, UnknownUnit>, ) -> Translation3D<T, Src, Dst>
```

Tag a unitless value with units.

#### `to_transform`

```rust
pub fn to_transform(&self) -> Transform3D<T, Src, Dst> where T: Zero + One,
```

Returns the matrix representation of this translation.

#### `transform_point3d`

```rust
pub fn transform_point3d( &self, p: &Point3D<T, Src>, ) -> Point3D<<T as Add>::Output, Dst> where T: Add,
```

Translate a point and cast its unit.

#### `transform_point2d`

```rust
pub fn transform_point2d( &self, p: &Point2D<T, Src>, ) -> Point2D<<T as Add>::Output, Dst> where T: Add,
```

Translate a point and cast its unit.

#### `transform_box2d`

```rust
pub fn transform_box2d( &self, b: &Box2D<T, Src>, ) -> Box2D<<T as Add>::Output, Dst> where T: Add,
```

Translate a 2D box and cast its unit.

#### `transform_box3d`

```rust
pub fn transform_box3d( &self, b: &Box3D<T, Src>, ) -> Box3D<<T as Add>::Output, Dst> where T: Add,
```

Translate a 3D box and cast its unit.

#### `transform_rect`

```rust
pub fn transform_rect(&self, r: &Rect<T, Src>) -> Rect<T, Dst> where T: Add<Output = T>,
```

Translate a rectangle and cast its unit.

#### `inverse`

```rust
pub fn inverse(&self) -> Translation3D<<T as Neg>::Output, Dst, Src> where T: Neg,
```

Return the inverse transformation.

### `impl<T, Src, Dst> Translation3D<T, Src, Dst> where T: NumCast + Copy,`

#### `cast`

```rust
pub fn cast<NewT>(self) -> Translation3D<NewT, Src, Dst> where NewT: NumCast,
```

Cast from one numeric representation to another, preserving the units.

When casting from floating vector to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.

#### `try_cast`

```rust
pub fn try_cast<NewT>(self) -> Option<Translation3D<NewT, Src, Dst>> where NewT: NumCast,
```

Fallible cast from one numeric representation to another, preserving the units.

When casting from floating vector to integer coordinates, the decimals are truncated as one would expect from a simple cast, but this behavior does not always make sense geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.

#### `to_f32`

```rust
pub fn to_f32(self) -> Translation3D<f32, Src, Dst>
```

Cast into an `f32` vector.

#### `to_f64`

```rust
pub fn to_f64(self) -> Translation3D<f64, Src, Dst>
```

Cast into an `f64` vector.

#### `to_usize`

```rust
pub fn to_usize(self) -> Translation3D<usize, Src, Dst>
```

Cast into an `usize` vector, truncating decimals if any.

When casting from floating vector vectors, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_u32`

```rust
pub fn to_u32(self) -> Translation3D<u32, Src, Dst>
```

Cast into an `u32` vector, truncating decimals if any.

When casting from floating vector vectors, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_i32`

```rust
pub fn to_i32(self) -> Translation3D<i32, Src, Dst>
```

Cast into an i32 vector, truncating decimals if any.

When casting from floating vector vectors, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

#### `to_i64`

```rust
pub fn to_i64(self) -> Translation3D<i64, Src, Dst>
```

Cast into an i64 vector, truncating decimals if any.

When casting from floating vector vectors, it is worth considering whether to `round()`, `ceil()` or `floor()` before the cast in order to obtain the desired conversion behavior.

## Trait Implementations

### `impl<T, Src, Dst1, Dst2> Add<Translation3D<T, Dst1, Dst2>> for Translation3D<T, Src, Dst1> where T: Add,`

#### `Output`

```rust
type Output = Translation3D<<T as Add>::Output, Src, Dst2>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add( self, other: Translation3D<T, Dst1, Dst2>, ) -> <Translation3D<T, Src, Dst1> as Add<Translation3D<T, Dst1, Dst2>>>::Output
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T, Src, Dst> AddAssign<Translation3D<T, Dst, Dst>> for Translation3D<T, Src, Dst> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Translation3D<T, Dst, Dst>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T, Src, Dst> Clone for Translation3D<T, Src, Dst> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Translation3D<T, Src, Dst>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, Src, Dst> Debug for Translation3D<T, Src, Dst> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, Src, Dst> Default for Translation3D<T, Src, Dst> where T: Zero,`

#### `default`

```rust
fn default() -> Translation3D<T, Src, Dst>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T, Src, Dst> From<Translation3D<T, Src, Dst>> for Transform3D<T, Src, Dst> where T: Zero + One,`

#### `from`

```rust
fn from(t: Translation3D<T, Src, Dst>) -> Transform3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Translation3D<T, Src, Dst>> for Vector3D<T, Src>`

#### `from`

```rust
fn from(t: Translation3D<T, Src, Dst>) -> Vector3D<T, Src>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> From<Vector3D<T, Src>> for Translation3D<T, Src, Dst>`

#### `from`

```rust
fn from(v: Vector3D<T, Src>) -> Translation3D<T, Src, Dst>
```

Converts to this type from the input type.

### `impl<T, Src, Dst> Hash for Translation3D<T, Src, Dst> where T: Hash,`

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

### `impl<T, Src, Dst> PartialEq for Translation3D<T, Src, Dst> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Translation3D<T, Src, Dst>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, Src, Dst1, Dst2> Sub<Translation3D<T, Dst1, Dst2>> for Translation3D<T, Src, Dst2> where T: Sub,`

#### `Output`

```rust
type Output = Translation3D<<T as Sub>::Output, Src, Dst1>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub( self, other: Translation3D<T, Dst1, Dst2>, ) -> <Translation3D<T, Src, Dst2> as Sub<Translation3D<T, Dst1, Dst2>>>::Output
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T, Src, Dst> SubAssign<Translation3D<T, Dst, Dst>> for Translation3D<T, Src, Dst> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Translation3D<T, Dst, Dst>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<T, Src, Dst> Copy for Translation3D<T, Src, Dst> where T: Copy,`

### `impl<T, Src, Dst> Eq for Translation3D<T, Src, Dst> where T: Eq,`

## Auto Trait Implementations

- `impl<T, Src, Dst> Freeze for Translation3D<T, Src, Dst> where T: Freeze,`
- `impl<T, Src, Dst> RefUnwindSafe for Translation3D<T, Src, Dst> where T: RefUnwindSafe, Src: RefUnwindSafe, Dst: RefUnwindSafe,`
- `impl<T, Src, Dst> Send for Translation3D<T, Src, Dst> where T: Send, Src: Send, Dst: Send,`
- `impl<T, Src, Dst> Sync for Translation3D<T, Src, Dst> where T: Sync, Src: Sync, Dst: Sync,`
- `impl<T, Src, Dst> Unpin for Translation3D<T, Src, Dst> where T: Unpin, Src: Unpin, Dst: Unpin,`
- `impl<T, Src, Dst> UnsafeUnpin for Translation3D<T, Src, Dst> where T: UnsafeUnpin,`
- `impl<T, Src, Dst> UnwindSafe for Translation3D<T, Src, Dst> where T: UnwindSafe, Src: UnwindSafe, Dst: UnwindSafe,`
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

### `impl<R, P> ReadPrimitive<R> for P where R: Read + ReadEndian<P>, P: Default,`

#### `read_from_little_endian`

```rust
fn read_from_little_endian(read: &mut R) -> Result<Self, Error>
```

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

#### `read_from_big_endian`

```rust
fn read_from_big_endian(read: &mut R) -> Result<Self, Error>
```

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

#### `read_from_native_endian`

```rust
fn read_from_native_endian(read: &mut R) -> Result<Self, Error>
```

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

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
