---
title: "Version"
kind: struct
path: "iced_beacon::Version"
crate: "iced_beacon"
module: "iced_beacon"
file: "iced_beacon/struct.Version.md"
docs_url: "https://docs.iced.rs/iced_beacon/struct.Version.html"
---

# `iced_beacon::Version`

```rust
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Prerelease,
    pub build: BuildMetadata,
}
```

**SemVer version** as defined by [https://semver.org](https://semver.org).

## Syntax

- The major, minor, and patch numbers may be any integer 0 through u64::MAX. When representing a SemVer version as a string, each number is written as a base 10 integer. For example, `1.0.119`.
- Leading zeros are forbidden in those positions. For example `1.01.00` is invalid as a SemVer version.
- The pre-release identifier, if present, must conform to the syntax documented for [`Prerelease`](https://docs.rs/semver/1.0.28/semver/struct.Prerelease.html).
- The build metadata, if present, must conform to the syntax documented for [`BuildMetadata`](https://docs.rs/semver/1.0.28/semver/struct.BuildMetadata.html).
- Whitespace is not allowed anywhere in the version.

## Total ordering

Given any two SemVer versions, one is less than, greater than, or equal to the other. Versions may be compared against one another using Rust’s usual comparison operators.

- The major, minor, and patch number are compared numerically from left to right, lexicographically ordered as a 3-tuple of integers. So for example version `1.5.0` is less than version `1.19.0`, despite the fact that “1.19.0” < “1.5.0” as ASCIIbetically compared strings and 1.19 < 1.5 as real numbers.
- When major, minor, and patch are equal, a pre-release version is considered less than the ordinary release: version `1.0.0-alpha.1` is less than version `1.0.0`.
- Two pre-releases of the same major, minor, patch are compared by lexicographic ordering of dot-separated components of the pre-release string.

  - Identifiers consisting of only digits are compared numerically: `1.0.0-pre.8` is less than `1.0.0-pre.12`.
  - Identifiers that contain a letter or hyphen are compared in ASCII sort order: `1.0.0-pre12` is less than `1.0.0-pre8`.
  - Any numeric identifier is always less than any non-numeric identifier: `1.0.0-pre.1` is less than `1.0.0-pre.x`.

Example: `1.0.0-alpha` < `1.0.0-alpha.1` < `1.0.0-alpha.beta` < `1.0.0-beta` < `1.0.0-beta.2` < `1.0.0-beta.11` < `1.0.0-rc.1` < `1.0.0`

**Source:** [lib.rs.html#158](https://docs.rs/semver/1.0.28/src/semver/lib.rs.html#158) (lines 158)

## Fields

### `major: u64`

### `minor: u64`

### `patch: u64`

### `pre: Prerelease`

### `build: BuildMetadata`

## Methods

### `impl Version`

#### `new`

```rust
pub const fn new(major: u64, minor: u64, patch: u64) -> Version
```

Create `Version` with an empty pre-release and build metadata.

Equivalent to:

```rust
Version {
    major,
    minor,
    patch,
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY,
}
```

#### `parse`

```rust
pub fn parse(text: &str) -> Result<Version, Error>
```

Create `Version` by parsing from string representation.

###### Errors

Possible reasons for the parse to fail include:

- `1.0` — too few numeric components. A SemVer version must have exactly three. If you are looking at something that has fewer than three numbers in it, it’s possible it is a `VersionReq` instead (with an implicit default `^` comparison operator).
- `1.0.01` — a numeric component has a leading zero.
- `1.0.unknown` — unexpected character in one of the components.
- `1.0.0-` or `1.0.0+` — the pre-release or build metadata are indicated present but empty.
- `1.0.0-alpha_123` — pre-release or build metadata have something outside the allowed characters, which are `0-9`, `A-Z`, `a-z`, `-`, and `.` (dot).
- `23456789999999999999.0.0` — overflow of a u64.

#### `cmp_precedence`

```rust
pub fn cmp_precedence(&self, other: &Version) -> Ordering
```

Compare the major, minor, patch, and pre-release value of two versions, disregarding build metadata. Versions that differ only in build metadata are considered equal. This comparison is what the SemVer spec refers to as “precedence”.

###### Example

```rust
use semver::Version;

let mut versions = [
    "1.20.0+c144a98".parse::<Version>().unwrap(),
    "1.20.0".parse().unwrap(),
    "1.0.0".parse().unwrap(),
    "1.0.0-alpha".parse().unwrap(),
    "1.20.0+bc17664".parse().unwrap(),
];

// This is a stable sort, so it preserves the relative order of equal
// elements. The three 1.20.0 versions differ only in build metadata so
// they are not reordered relative to one another.
versions.sort_by(Version::cmp_precedence);
assert_eq!(versions, [
    "1.0.0-alpha".parse().unwrap(),
    "1.0.0".parse().unwrap(),
    "1.20.0+c144a98".parse().unwrap(),
    "1.20.0".parse().unwrap(),
    "1.20.0+bc17664".parse().unwrap(),
]);

// Totally order the versions, including comparing the build metadata.
versions.sort();
assert_eq!(versions, [
    "1.0.0-alpha".parse().unwrap(),
    "1.0.0".parse().unwrap(),
    "1.20.0".parse().unwrap(),
    "1.20.0+bc17664".parse().unwrap(),
    "1.20.0+c144a98".parse().unwrap(),
]);
```

## Trait Implementations

### `impl Clone for Version`

#### `clone`

```rust
fn clone(&self) -> Version
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Version`

#### `fmt`

```rust
fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<'de> Deserialize<'de> for Version`

#### `deserialize`

```rust
fn deserialize<D>( deserializer: D, ) -> Result<Version, <D as Deserializer<'de>>::Error> where D: Deserializer<'de>,
```

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

### `impl Display for Version`

#### `fmt`

```rust
fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

### `impl FromStr for Version`

#### `Err`

```rust
type Err = Error
```

The associated error which can be returned from parsing.

#### `from_str`

```rust
fn from_str(text: &str) -> Result<Version, <Version as FromStr>::Err>
```

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

### `impl Hash for Version`

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

### `impl Ord for Version`

#### `cmp`

```rust
fn cmp(&self, other: &Version) -> Ordering
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

### `impl PartialEq for Version`

#### `eq`

```rust
fn eq(&self, other: &Version) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd for Version`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Version) -> Option<Ordering>
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

### `impl Serialize for Version`

#### `serialize`

```rust
fn serialize<S>( &self, serializer: S, ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer,
```

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.229/serde_core/ser/trait.Serialize.html#tymethod.serialize)

### `impl Eq for Version`

### `impl StructuralPartialEq for Version`

## Auto Trait Implementations

- `impl Freeze for Version`
- `impl RefUnwindSafe for Version`
- `impl Send for Version`
- `impl Sync for Version`
- `impl Unpin for Version`
- `impl UnsafeUnpin for Version`
- `impl UnwindSafe for Version`
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

### `impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de>,`
