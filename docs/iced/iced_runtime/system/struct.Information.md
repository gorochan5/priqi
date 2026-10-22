---
title: "Information"
kind: struct
path: "iced_runtime::system::Information"
crate: "iced_runtime"
module: "iced_runtime::system"
file: "iced_runtime/system/struct.Information.md"
docs_url: "https://docs.iced.rs/iced_runtime/system/struct.Information.html"
source_file: "src/iced_runtime/system.rs"
source_lines: "22-48"
---

# `iced_runtime::system::Information`

```rust
pub struct Information {
    pub system_name: Option<String>,
    pub system_kernel: Option<String>,
    pub system_version: Option<String>,
    pub system_short_version: Option<String>,
    pub cpu_brand: String,
    pub cpu_cores: Option<usize>,
    pub memory_total: u64,
    pub memory_used: Option<u64>,
    pub graphics_backend: String,
    pub graphics_adapter: String,
}
```

Contains information about the system (e.g. system name, processor, memory, graphics adapter).

**Source:** [system.rs](../../src/iced_runtime/system.rs.md) (lines 22-48)

## Fields

### `system_name: Option<String>`

The operating system name

### `system_kernel: Option<String>`

Operating system kernel version

### `system_version: Option<String>`

Long operating system version

Examples:

- MacOS 10.15 Catalina
- Windows 10 Pro
- Ubuntu 20.04 LTS (Focal Fossa)

### `system_short_version: Option<String>`

Short operating system version number

### `cpu_brand: String`

Detailed processor model information

### `cpu_cores: Option<usize>`

The number of physical cores on the processor

### `memory_total: u64`

Total RAM size, in bytes

### `memory_used: Option<u64>`

Memory used by this process, in bytes

### `graphics_backend: String`

Underlying graphics backend for rendering

### `graphics_adapter: String`

Model information for the active graphics adapter

## Trait Implementations

### `impl Clone for Information`

#### `clone`

```rust
fn clone(&self) -> Information
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Information`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations

- `impl Freeze for Information`
- `impl RefUnwindSafe for Information`
- `impl Send for Information`
- `impl Sync for Information`
- `impl Unpin for Information`
- `impl UnsafeUnpin for Information`
- `impl UnwindSafe for Information`
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

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`
