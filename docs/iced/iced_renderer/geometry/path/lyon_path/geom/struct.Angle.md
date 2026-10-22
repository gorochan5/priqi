---
title: "Angle"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::Angle"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom"
file: "iced_renderer/geometry/path/lyon_path/geom/struct.Angle.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/struct.Angle.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::Angle`

```rust
pub struct Angle<T> {
    pub radians: T,
}
```

An angle in radians

## Fields

### `radians: T`

## Methods

### `impl<T> Angle<T>`

#### `radians`

```rust
pub fn radians(radians: T) -> Angle<T>
```

#### `get`

```rust
pub fn get(self) -> T
```

### `impl<T> Angle<T> where T: Trig,`

#### `degrees`

```rust
pub fn degrees(deg: T) -> Angle<T>
```

#### `to_degrees`

```rust
pub fn to_degrees(self) -> T
```

### `impl<T> Angle<T> where T: Sub<Output = T> + Rem<Output = T> + Add<Output = T> + Zero + FloatConst + PartialOrd + Copy,`

#### `positive`

```rust
pub fn positive(&self) -> Angle<T>
```

Returns this angle in the [0..2*PI[ range.

#### `signed`

```rust
pub fn signed(&self) -> Angle<T>
```

Returns this angle in the ]-PI..PI] range.

### `impl<T> Angle<T> where T: Rem<Output = T> + Sub<Output = T> + Add<Output = T> + Mul<Output = T> + One + FloatConst + Copy,`

#### `angle_to`

```rust
pub fn angle_to(&self, to: Angle<T>) -> Angle<T>
```

Returns the shortest signed angle between two angles.

Takes wrapping and signs into account.

#### `lerp`

```rust
pub fn lerp(&self, other: Angle<T>, t: T) -> Angle<T>
```

Linear interpolation between two angles, using the shortest path.

### `impl<T> Angle<T> where T: Float,`

#### `is_finite`

```rust
pub fn is_finite(self) -> bool
```

Returns `true` if the angle is a finite number.

### `impl<T> Angle<T> where T: Real,`

#### `sin_cos`

```rust
pub fn sin_cos(self) -> (T, T)
```

Returns `(sin(self), cos(self))`.

### `impl<T> Angle<T> where T: Zero,`

#### `zero`

```rust
pub fn zero() -> Angle<T>
```

### `impl<T> Angle<T> where T: FloatConst + Add<Output = T>,`

#### `pi`

```rust
pub fn pi() -> Angle<T>
```

#### `two_pi`

```rust
pub fn two_pi() -> Angle<T>
```

#### `frac_pi_2`

```rust
pub fn frac_pi_2() -> Angle<T>
```

#### `frac_pi_3`

```rust
pub fn frac_pi_3() -> Angle<T>
```

#### `frac_pi_4`

```rust
pub fn frac_pi_4() -> Angle<T>
```

### `impl<T> Angle<T> where T: NumCast + Copy,`

#### `cast`

```rust
pub fn cast<NewT>(&self) -> Angle<NewT> where NewT: NumCast,
```

Cast from one numeric representation to another.

#### `try_cast`

```rust
pub fn try_cast<NewT>(&self) -> Option<Angle<NewT>> where NewT: NumCast,
```

Fallible cast from one numeric representation to another.

#### `to_f32`

```rust
pub fn to_f32(&self) -> Angle<f32>
```

Cast angle to `f32`.

#### `to_f64`

```rust
pub fn to_f64(&self) -> Angle<f64>
```

Cast angle `f64`.

## Trait Implementations

### `impl<T> Add<&Angle<T>> for Angle<T> where T: Copy + Add<Output = T>,`

#### `Output`

```rust
type Output = Angle<T>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, other: &Angle<T>) -> Angle<T>
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T> Add for Angle<T> where T: Add<Output = T>,`

#### `Output`

```rust
type Output = Angle<T>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, other: Angle<T>) -> Angle<T>
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T> AddAssign for Angle<T> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, other: Angle<T>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T> ApproxEq<T> for Angle<T> where T: ApproxEq<T>,`

#### `approx_epsilon`

```rust
fn approx_epsilon() -> T
```

Default epsilon value

#### `approx_eq_eps`

```rust
fn approx_eq_eps(&self, other: &Angle<T>, approx_epsilon: &T) -> bool
```

Returns `true` if this object is approximately equal to the other one, using a provided epsilon value.

#### `approx_eq`

```rust
fn approx_eq(&self, other: &Self) -> bool
```

Returns `true` if this object is approximately equal to the other one, using the [`approx_epsilon`](euclid/approxeq/trait.ApproxEq.md) epsilon value.

### `impl<T> Clone for Angle<T> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> Angle<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T> Debug for Angle<T> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T> Default for Angle<T> where T: Default,`

#### `default`

```rust
fn default() -> Angle<T>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T> Div<T> for Angle<T> where T: Div<Output = T>,`

#### `Output`

```rust
type Output = Angle<T>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, factor: T) -> Angle<T>
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T> Div for Angle<T> where T: Div<Output = T>,`

#### `Output`

```rust
type Output = T
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, other: Angle<T>) -> T
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T> DivAssign<T> for Angle<T> where T: DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, factor: T)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T> Hash for Angle<T> where T: Hash,`

#### `hash`

```rust
fn hash<__H>(&self, state: &mut __H) where __H: Hasher,
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl<T> Mul<T> for Angle<T> where T: Mul<Output = T>,`

#### `Output`

```rust
type Output = Angle<T>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, factor: T) -> Angle<T>
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T> MulAssign<T> for Angle<T> where T: MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, factor: T)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T> Neg for Angle<T> where T: Neg<Output = T>,`

#### `Output`

```rust
type Output = Angle<T>
```

The resulting type after applying the `-` operator.

#### `neg`

```rust
fn neg(self) -> Angle<T>
```

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

### `impl<T> PartialEq for Angle<T> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &Angle<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T> PartialOrd for Angle<T> where T: PartialOrd,`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Angle<T>) -> Option<Ordering>
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

### `impl<T> Sub for Angle<T> where T: Sub<Output = T>,`

#### `Output`

```rust
type Output = Angle<T>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, other: Angle<T>) -> <Angle<T> as Sub>::Output
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T> SubAssign for Angle<T> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: Angle<T>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<'a, T> Sum<&'a Angle<T>> for Angle<T> where T: 'a + Add + Copy + Zero,`

#### `sum`

```rust
fn sum<I>(iter: I) -> Angle<T> where I: Iterator<Item = &'a Angle<T>>,
```

Takes an iterator and generates `Self` from the elements by “summing up” the items.

### `impl<T> Sum for Angle<T> where T: Add + Zero,`

#### `sum`

```rust
fn sum<I>(iter: I) -> Angle<T> where I: Iterator<Item = Angle<T>>,
```

Takes an iterator and generates `Self` from the elements by “summing up” the items.

### `impl<T> Copy for Angle<T> where T: Copy,`

### `impl<T> Eq for Angle<T> where T: Eq,`

### `impl<T> StructuralPartialEq for Angle<T>`

## Auto Trait Implementations

- `impl<T> Freeze for Angle<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for Angle<T> where T: RefUnwindSafe,`
- `impl<T> Send for Angle<T> where T: Send,`
- `impl<T> Sync for Angle<T> where T: Sync,`
- `impl<T> Unpin for Angle<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for Angle<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for Angle<T> where T: UnwindSafe,`
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
