---
title: "SideOffsets2D"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::SideOffsets2D"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/struct.SideOffsets2D.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/struct.SideOffsets2D.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::SideOffsets2D`

```rust
pub struct SideOffsets2D<T, U> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
    /* private fields */
}
```

A group of 2D side offsets, which correspond to top/right/bottom/left for borders, padding, and margins in CSS, optionally tagged with a unit.

## Fields

### `top: T`

### `right: T`

### `bottom: T`

### `left: T`

## Methods

### `impl<T, U> SideOffsets2D<T, U>`

#### `new`

```rust
pub const fn new(top: T, right: T, bottom: T, left: T) -> SideOffsets2D<T, U>
```

Constructor taking a scalar for each side.

Sides are specified in top-right-bottom-left order following CSS’s convention.

#### `from_lengths`

```rust
pub fn from_lengths( top: Length<T, U>, right: Length<T, U>, bottom: Length<T, U>, left: Length<T, U>, ) -> SideOffsets2D<T, U>
```

Constructor taking a typed Length for each side.

Sides are specified in top-right-bottom-left order following CSS’s convention.

#### `from_vectors_outer`

```rust
pub fn from_vectors_outer( min: Vector2D<T, U>, max: Vector2D<T, U>, ) -> SideOffsets2D<T, U> where T: Neg<Output = T>,
```

Construct side offsets from min and a max vector offsets.

The outer rect of the resulting side offsets is equivalent to translating a rectangle’s upper-left corner with the min vector and translating the bottom-right corner with the max vector.

#### `from_vectors_inner`

```rust
pub fn from_vectors_inner( min: Vector2D<T, U>, max: Vector2D<T, U>, ) -> SideOffsets2D<T, U> where T: Neg<Output = T>,
```

Construct side offsets from min and a max vector offsets.

The inner rect of the resulting side offsets is equivalent to translating a rectangle’s upper-left corner with the min vector and translating the bottom-right corner with the max vector.

#### `zero`

```rust
pub fn zero() -> SideOffsets2D<T, U> where T: Zero,
```

Constructor, setting all sides to zero.

#### `is_zero`

```rust
pub fn is_zero(&self) -> bool where T: Zero + PartialEq,
```

Returns `true` if all side offsets are zero.

#### `new_all_same`

```rust
pub fn new_all_same(all: T) -> SideOffsets2D<T, U> where T: Copy,
```

Constructor setting the same value to all sides, taking a scalar value directly.

#### `from_length_all_same`

```rust
pub fn from_length_all_same(all: Length<T, U>) -> SideOffsets2D<T, U> where T: Copy,
```

Constructor setting the same value to all sides, taking a typed Length.

#### `horizontal`

```rust
pub fn horizontal(&self) -> T where T: Copy + Add<Output = T>,
```

#### `vertical`

```rust
pub fn vertical(&self) -> T where T: Copy + Add<Output = T>,
```

## Trait Implementations

### `impl<T, U> Add for SideOffsets2D<T, U> where T: Add<Output = T>,`

#### `Output`

```rust
type Output = SideOffsets2D<T, U>
```

The resulting type after applying the `+` operator.

#### `add`

```rust
fn add(self, other: SideOffsets2D<T, U>) -> SideOffsets2D<T, U>
```

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

### `impl<T, U> AddAssign for SideOffsets2D<T, U> where T: AddAssign,`

#### `add_assign`

```rust
fn add_assign(&mut self, other: SideOffsets2D<T, U>)
```

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

### `impl<T, U> Clone for SideOffsets2D<T, U> where T: Clone,`

#### `clone`

```rust
fn clone(&self) -> SideOffsets2D<T, U>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T, U> Debug for SideOffsets2D<T, U> where T: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T, U> Default for SideOffsets2D<T, U> where T: Default,`

#### `default`

```rust
fn default() -> SideOffsets2D<T, U>
```

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

### `impl<T, U1, U2> Div<Scale<T, U1, U2>> for SideOffsets2D<T, U2> where T: Copy + Div,`

#### `Output`

```rust
type Output = SideOffsets2D<<T as Div>::Output, U1>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div( self, scale: Scale<T, U1, U2>, ) -> <SideOffsets2D<T, U2> as Div<Scale<T, U1, U2>>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U> Div<T> for SideOffsets2D<T, U> where T: Copy + Div,`

#### `Output`

```rust
type Output = SideOffsets2D<<T as Div>::Output, U>
```

The resulting type after applying the `/` operator.

#### `div`

```rust
fn div(self, scale: T) -> <SideOffsets2D<T, U> as Div<T>>::Output
```

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

### `impl<T, U> DivAssign<Scale<T, U, U>> for SideOffsets2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> DivAssign<T> for SideOffsets2D<T, U> where T: Copy + DivAssign,`

#### `div_assign`

```rust
fn div_assign(&mut self, other: T)
```

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

### `impl<T, U> Hash for SideOffsets2D<T, U> where T: Hash,`

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

### `impl<T, U1, U2> Mul<Scale<T, U1, U2>> for SideOffsets2D<T, U1> where T: Copy + Mul,`

#### `Output`

```rust
type Output = SideOffsets2D<<T as Mul>::Output, U2>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul( self, scale: Scale<T, U1, U2>, ) -> <SideOffsets2D<T, U1> as Mul<Scale<T, U1, U2>>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U> Mul<T> for SideOffsets2D<T, U> where T: Copy + Mul,`

#### `Output`

```rust
type Output = SideOffsets2D<<T as Mul>::Output, U>
```

The resulting type after applying the `*` operator.

#### `mul`

```rust
fn mul(self, scale: T) -> <SideOffsets2D<T, U> as Mul<T>>::Output
```

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

### `impl<T, U> MulAssign<Scale<T, U, U>> for SideOffsets2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, other: Scale<T, U, U>)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> MulAssign<T> for SideOffsets2D<T, U> where T: Copy + MulAssign,`

#### `mul_assign`

```rust
fn mul_assign(&mut self, other: T)
```

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

### `impl<T, U> Neg for SideOffsets2D<T, U> where T: Neg<Output = T>,`

#### `Output`

```rust
type Output = SideOffsets2D<T, U>
```

The resulting type after applying the `-` operator.

#### `neg`

```rust
fn neg(self) -> SideOffsets2D<T, U>
```

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

### `impl<T, U> PartialEq for SideOffsets2D<T, U> where T: PartialEq,`

#### `eq`

```rust
fn eq(&self, other: &SideOffsets2D<T, U>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T, U> Sub for SideOffsets2D<T, U> where T: Sub<Output = T>,`

#### `Output`

```rust
type Output = SideOffsets2D<T, U>
```

The resulting type after applying the `-` operator.

#### `sub`

```rust
fn sub(self, other: SideOffsets2D<T, U>) -> SideOffsets2D<T, U>
```

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

### `impl<T, U> SubAssign for SideOffsets2D<T, U> where T: SubAssign,`

#### `sub_assign`

```rust
fn sub_assign(&mut self, other: SideOffsets2D<T, U>)
```

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

### `impl<T, U> Copy for SideOffsets2D<T, U> where T: Copy,`

### `impl<T, U> Eq for SideOffsets2D<T, U> where T: Eq,`

## Auto Trait Implementations

- `impl<T, U> Freeze for SideOffsets2D<T, U> where T: Freeze,`
- `impl<T, U> RefUnwindSafe for SideOffsets2D<T, U> where T: RefUnwindSafe, U: RefUnwindSafe,`
- `impl<T, U> Send for SideOffsets2D<T, U> where T: Send, U: Send,`
- `impl<T, U> Sync for SideOffsets2D<T, U> where T: Sync, U: Sync,`
- `impl<T, U> Unpin for SideOffsets2D<T, U> where T: Unpin, U: Unpin,`
- `impl<T, U> UnsafeUnpin for SideOffsets2D<T, U> where T: UnsafeUnpin,`
- `impl<T, U> UnwindSafe for SideOffsets2D<T, U> where T: UnwindSafe, U: UnwindSafe,`
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
