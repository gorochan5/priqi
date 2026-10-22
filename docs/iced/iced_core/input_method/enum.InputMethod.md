---
title: "InputMethod"
kind: enum
path: "iced_core::input_method::InputMethod"
crate: "iced_core"
module: "iced_core::input_method"
file: "iced_core/input_method/enum.InputMethod.md"
docs_url: "https://docs.iced.rs/iced_core/input_method/enum.InputMethod.html"
source_file: "src/iced_core/input_method.rs"
source_lines: "8-26"
---

# `iced_core::input_method::InputMethod`

```rust
pub enum InputMethod<T = String> {
    Disabled,
    Enabled {
        cursor: Rectangle,
        purpose: Purpose,
        preedit: Option<Preedit<T>>,
    },
}
```

The input method strategy of a widget.

**Source:** [input_method.rs](../../src/iced_core/input_method.rs.md) (lines 8-26)

## Variants

### `Disabled`

Input method is disabled.

### `Enabled`

Input method is enabled.

**Fields**

- `cursor: Rectangle` — The area of the cursor of the input method.

This area should not be covered.
- `purpose: Purpose` — The [`Purpose`](enum.Purpose.md) of the input method.
- `preedit: Option<Preedit<T>>` — The preedit to overlay on top of the input method dialog, if needed.

Ideally, your widget will show pre-edits on-the-spot; but, since that can be tricky, you can instead provide the current pre-edit here and the runtime will display it as an overlay (i.e. “Over-the-spot IME”).

## Methods

### `impl InputMethod`

#### `merge`

```rust
pub fn merge<T: AsRef<str>>(&mut self, other: &InputMethod<T>)
```

Merges two [`InputMethod`](enum.InputMethod.md) strategies, prioritizing the first one when both open:

```rust
let open = InputMethod::Enabled {
    cursor: Rectangle::new(Point::ORIGIN, Size::UNIT),
    purpose: Purpose::Normal,
    preedit: Some(Preedit { content: "1".to_owned(), selection: None, text_size: None }),
};

let open_2 = InputMethod::Enabled {
    cursor: Rectangle::new(Point::ORIGIN, Size::UNIT),
    purpose: Purpose::Secure,
    preedit: Some(Preedit { content: "2".to_owned(), selection: None, text_size: None }),
};

let mut ime = InputMethod::Disabled;

ime.merge(&open);
assert_eq!(ime, open);

ime.merge(&open_2);
assert_eq!(ime, open);
```

#### `is_enabled`

```rust
pub fn is_enabled(&self) -> bool
```

Returns true if the [`InputMethod`](enum.InputMethod.md) is open.

### `impl<T> InputMethod<T>`

#### `to_owned`

```rust
pub fn to_owned(&self) -> InputMethod where T: AsRef<str>,
```

Turns an [`InputMethod`](enum.InputMethod.md) into its owned version.

## Trait Implementations

### `impl<T: Clone> Clone for InputMethod<T>`

#### `clone`

```rust
fn clone(&self) -> InputMethod<T>
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl<T: Debug> Debug for InputMethod<T>`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<T: PartialEq> PartialEq for InputMethod<T>`

#### `eq`

```rust
fn eq(&self, other: &InputMethod<T>) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl<T> StructuralPartialEq for InputMethod<T>`

## Auto Trait Implementations

- `impl<T> Freeze for InputMethod<T> where T: Freeze,`
- `impl<T> RefUnwindSafe for InputMethod<T> where T: RefUnwindSafe,`
- `impl<T> Send for InputMethod<T> where T: Send,`
- `impl<T> Sync for InputMethod<T> where T: Sync,`
- `impl<T> Unpin for InputMethod<T> where T: Unpin,`
- `impl<T> UnsafeUnpin for InputMethod<T> where T: UnsafeUnpin,`
- `impl<T> UnwindSafe for InputMethod<T> where T: UnwindSafe,`
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
