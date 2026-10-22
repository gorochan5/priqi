---
title: "Code"
kind: enum
path: "iced_core::keyboard::key::Code"
crate: "iced_core"
module: "iced_core::keyboard::key"
file: "iced_core/keyboard/key/enum.Code.md"
docs_url: "https://docs.iced.rs/iced_core/keyboard/key/enum.Code.html"
source_file: "src/iced_core/keyboard/key.rs"
source_lines: "849-1293"
---

# `iced_core::keyboard::key::Code`

```rust
#[non_exhaustive]pub enum Code {
Show 194 variants    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Meta,
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}
```

Code representing the location of a physical key.

This mostly conforms to the UI Events Specification’s [`KeyboardEvent.code`](https://w3c.github.io/uievents-code/#code-value-tables) with a few exceptions:

- The keys that the specification calls “MetaLeft” and “MetaRight” are named “SuperLeft” and “SuperRight” here.
- The key that the specification calls “Super” is reported as `Unidentified` here.

**Source:** [key.rs](../../../src/iced_core/keyboard/key.rs.md) (lines 849-1293)

## Variants

### `Backquote`

`` ` `` on a US keyboard. This is also called a backtick or grave. This is the `半角`/`全角`/`漢字` (hankaku/zenkaku/kanji) key on Japanese keyboards

### `Backslash`

Used for both the US `\` (on the 101-key layout) and also for the key located between the `“` and `Enter` keys on row C of the 102-, 104- and 106-key layouts. Labeled `#` on a UK (102) keyboard.

### `BracketLeft`

`[` on a US keyboard.

### `BracketRight`

`]` on a US keyboard.

### `Comma`

`,` on a US keyboard.

### `Digit0`

`0` on a US keyboard.

### `Digit1`

`1` on a US keyboard.

### `Digit2`

`2` on a US keyboard.

### `Digit3`

`3` on a US keyboard.

### `Digit4`

`4` on a US keyboard.

### `Digit5`

`5` on a US keyboard.

### `Digit6`

`6` on a US keyboard.

### `Digit7`

`7` on a US keyboard.

### `Digit8`

`8` on a US keyboard.

### `Digit9`

`9` on a US keyboard.

### `Equal`

`=` on a US keyboard.

### `IntlBackslash`

Located between the left `Shift` and `Z` keys. Labeled `\` on a UK keyboard.

### `IntlRo`

Located between the `/` and right `Shift` keys. Labeled `\` (ro) on a Japanese keyboard.

### `IntlYen`

Located between the `=` and `Backspace` keys. Labeled `¥` (yen) on a Japanese keyboard. `\` on a Russian keyboard.

### `KeyA`

`a` on a US keyboard. Labeled `q` on an AZERTY (e.g., French) keyboard.

### `KeyB`

`b` on a US keyboard.

### `KeyC`

`c` on a US keyboard.

### `KeyD`

`d` on a US keyboard.

### `KeyE`

`e` on a US keyboard.

### `KeyF`

`f` on a US keyboard.

### `KeyG`

`g` on a US keyboard.

### `KeyH`

`h` on a US keyboard.

### `KeyI`

`i` on a US keyboard.

### `KeyJ`

`j` on a US keyboard.

### `KeyK`

`k` on a US keyboard.

### `KeyL`

`l` on a US keyboard.

### `KeyM`

`m` on a US keyboard.

### `KeyN`

`n` on a US keyboard.

### `KeyO`

`o` on a US keyboard.

### `KeyP`

`p` on a US keyboard.

### `KeyQ`

`q` on a US keyboard. Labeled `a` on an AZERTY (e.g., French) keyboard.

### `KeyR`

`r` on a US keyboard.

### `KeyS`

`s` on a US keyboard.

### `KeyT`

`t` on a US keyboard.

### `KeyU`

`u` on a US keyboard.

### `KeyV`

`v` on a US keyboard.

### `KeyW`

`w` on a US keyboard. Labeled `z` on an AZERTY (e.g., French) keyboard.

### `KeyX`

`x` on a US keyboard.

### `KeyY`

`y` on a US keyboard. Labeled `z` on a QWERTZ (e.g., German) keyboard.

### `KeyZ`

`z` on a US keyboard. Labeled `w` on an AZERTY (e.g., French) keyboard, and `y` on a QWERTZ (e.g., German) keyboard.

### `Minus`

`-` on a US keyboard.

### `Period`

`.` on a US keyboard.

### `Quote`

`’` on a US keyboard.

### `Semicolon`

`;` on a US keyboard.

### `Slash`

`/` on a US keyboard.

### `AltLeft`

`Alt`, `Option`, or `⌥`.

### `AltRight`

`Alt`, `Option`, or `⌥`. This is labeled `AltGr` on many keyboard layouts.

### `Backspace`

`Backspace` or `⌫`. Labeled `Delete` on Apple keyboards.

### `CapsLock`

`CapsLock` or `⇪`

### `ContextMenu`

The application context menu key, which is typically found between the right `Super` key and the right `Control` key.

### `ControlLeft`

`Control` or `⌃`

### `ControlRight`

`Control` or `⌃`

### `Enter`

`Enter` or `↵`. Labeled `Return` on Apple keyboards.

### `SuperLeft`

The Windows, `⌘`, `Command`, or other OS symbol key.

### `SuperRight`

The Windows, `⌘`, `Command`, or other OS symbol key.

### `ShiftLeft`

`Shift` or `⇧`

### `ShiftRight`

`Shift` or `⇧`

### `Space`

`` (space)

### `Tab`

`Tab` or `⇥`

### `Convert`

Japanese: `変` (henkan)

### `KanaMode`

Japanese: `カタカナ`/`ひらがな`/`ローマ字` (katakana/hiragana/romaji)

### `Lang1`

Korean: HangulMode `한/영` (han/yeong)

Japanese (Mac keyboard): `か` (kana)

### `Lang2`

Korean: Hanja `한` (hanja)

Japanese (Mac keyboard): `英` (eisu)

### `Lang3`

Japanese (word-processing keyboard): Katakana

### `Lang4`

Japanese (word-processing keyboard): Hiragana

### `Lang5`

Japanese (word-processing keyboard): Zenkaku/Hankaku

### `NonConvert`

Japanese: `無変換` (muhenkan)

### `Delete`

`⌦`. The forward delete key. Note that on Apple keyboards, the key labelled `Delete` on the main part of the keyboard is encoded as [`Backspace`](enum.Code.md).

### `End`

`Page Down`, `End`, or `↘`

### `Help`

`Help`. Not present on standard PC keyboards.

### `Home`

`Home` or `↖`

### `Insert`

`Insert` or `Ins`. Not present on Apple keyboards.

### `PageDown`

`Page Down`, `PgDn`, or `⇟`

### `PageUp`

`Page Up`, `PgUp`, or `⇞`

### `ArrowDown`

`↓`

### `ArrowLeft`

`←`

### `ArrowRight`

`→`

### `ArrowUp`

`↑`

### `NumLock`

On the Mac, this is used for the numpad `Clear` key.

### `Numpad0`

`0 Ins` on a keyboard. `0` on a phone or remote control

### `Numpad1`

`1 End` on a keyboard. `1` or `1 QZ` on a phone or remote control

### `Numpad2`

`2 ↓` on a keyboard. `2 ABC` on a phone or remote control

### `Numpad3`

`3 PgDn` on a keyboard. `3 DEF` on a phone or remote control

### `Numpad4`

`4 ←` on a keyboard. `4 GHI` on a phone or remote control

### `Numpad5`

`5` on a keyboard. `5 JKL` on a phone or remote control

### `Numpad6`

`6 →` on a keyboard. `6 MNO` on a phone or remote control

### `Numpad7`

`7 Home` on a keyboard. `7 PQRS` or `7 PRS` on a phone or remote control

### `Numpad8`

`8 ↑` on a keyboard. `8 TUV` on a phone or remote control

### `Numpad9`

`9 PgUp` on a keyboard. `9 WXYZ` or `9 WXY` on a phone or remote control

### `NumpadAdd`

`+`

### `NumpadBackspace`

Found on the Microsoft Natural Keyboard.

### `NumpadClear`

`C` or `A` (All Clear). Also for use with numpads that have a `Clear` key that is separate from the `NumLock` key. On the Mac, the numpad `Clear` key is encoded as [`NumLock`](enum.Code.md).

### `NumpadClearEntry`

`C` (Clear Entry)

### `NumpadComma`

`,` (thousands separator). For locales where the thousands separator is a “.” (e.g., Brazil), this key may generate a `.`.

### `NumpadDecimal`

`. Del`. For locales where the decimal separator is “,” (e.g., Brazil), this key may generate a `,`.

### `NumpadDivide`

`/`

### `NumpadEnter`

### `NumpadEqual`

`=`

### `NumpadHash`

`#` on a phone or remote control device. This key is typically found below the `9` key and to the right of the `0` key.

### `NumpadMemoryAdd`

`M` Add current entry to the value stored in memory.

### `NumpadMemoryClear`

`M` Clear the value stored in memory.

### `NumpadMemoryRecall`

`M` Replace the current entry with the value stored in memory.

### `NumpadMemoryStore`

`M` Replace the value stored in memory with the current entry.

### `NumpadMemorySubtract`

`M` Subtract current entry from the value stored in memory.

### `NumpadMultiply`

`*` on a keyboard. For use with numpads that provide mathematical operations (`+`, `-` `*` and `/`).

Use `NumpadStar` for the `*` key on phones and remote controls.

### `NumpadParenLeft`

`(` Found on the Microsoft Natural Keyboard.

### `NumpadParenRight`

`)` Found on the Microsoft Natural Keyboard.

### `NumpadStar`

`*` on a phone or remote control device.

This key is typically found below the `7` key and to the left of the `0` key.

Use `“NumpadMultiply”` for the `*` key on numeric keypads.

### `NumpadSubtract`

`-`

### `Escape`

`Esc` or `⎋`

### `Fn`

`Fn` This is typically a hardware key that does not generate a separate code.

### `FnLock`

`FLock` or `FnLock`. Function Lock key. Found on the Microsoft Natural Keyboard.

### `PrintScreen`

`PrtScr SysRq` or `Print Screen`

### `ScrollLock`

`Scroll Lock`

### `Pause`

`Pause Break`

### `BrowserBack`

Some laptops place this key to the left of the `↑` key.

This also the “back” button (triangle) on Android.

### `BrowserFavorites`

### `BrowserForward`

Some laptops place this key to the right of the `↑` key.

### `BrowserHome`

The “home” button on Android.

### `BrowserRefresh`

### `BrowserSearch`

### `BrowserStop`

### `Eject`

`Eject` or `⏏`. This key is placed in the function section on some Apple keyboards.

### `LaunchApp1`

Sometimes labelled `My Computer` on the keyboard

### `LaunchApp2`

Sometimes labelled `Calculator` on the keyboard

### `LaunchMail`

### `MediaPlayPause`

### `MediaSelect`

### `MediaStop`

### `MediaTrackNext`

### `MediaTrackPrevious`

### `Power`

This key is placed in the function section on some Apple keyboards, replacing the `Eject` key.

### `Sleep`

### `AudioVolumeDown`

### `AudioVolumeMute`

### `AudioVolumeUp`

### `WakeUp`

### `Meta`

### `Hyper`

### `Turbo`

### `Abort`

### `Resume`

### `Suspend`

### `Again`

Found on Sun’s USB keyboard.

### `Copy`

Found on Sun’s USB keyboard.

### `Cut`

Found on Sun’s USB keyboard.

### `Find`

Found on Sun’s USB keyboard.

### `Open`

Found on Sun’s USB keyboard.

### `Paste`

Found on Sun’s USB keyboard.

### `Props`

Found on Sun’s USB keyboard.

### `Select`

Found on Sun’s USB keyboard.

### `Undo`

Found on Sun’s USB keyboard.

### `Hiragana`

Use for dedicated `ひらがな` key found on some Japanese word processing keyboards.

### `Katakana`

Use for dedicated `カタカナ` key found on some Japanese word processing keyboards.

### `F1`

General-purpose function key. Usually found at the top of the keyboard.

### `F2`

General-purpose function key. Usually found at the top of the keyboard.

### `F3`

General-purpose function key. Usually found at the top of the keyboard.

### `F4`

General-purpose function key. Usually found at the top of the keyboard.

### `F5`

General-purpose function key. Usually found at the top of the keyboard.

### `F6`

General-purpose function key. Usually found at the top of the keyboard.

### `F7`

General-purpose function key. Usually found at the top of the keyboard.

### `F8`

General-purpose function key. Usually found at the top of the keyboard.

### `F9`

General-purpose function key. Usually found at the top of the keyboard.

### `F10`

General-purpose function key. Usually found at the top of the keyboard.

### `F11`

General-purpose function key. Usually found at the top of the keyboard.

### `F12`

General-purpose function key. Usually found at the top of the keyboard.

### `F13`

General-purpose function key. Usually found at the top of the keyboard.

### `F14`

General-purpose function key. Usually found at the top of the keyboard.

### `F15`

General-purpose function key. Usually found at the top of the keyboard.

### `F16`

General-purpose function key. Usually found at the top of the keyboard.

### `F17`

General-purpose function key. Usually found at the top of the keyboard.

### `F18`

General-purpose function key. Usually found at the top of the keyboard.

### `F19`

General-purpose function key. Usually found at the top of the keyboard.

### `F20`

General-purpose function key. Usually found at the top of the keyboard.

### `F21`

General-purpose function key. Usually found at the top of the keyboard.

### `F22`

General-purpose function key. Usually found at the top of the keyboard.

### `F23`

General-purpose function key. Usually found at the top of the keyboard.

### `F24`

General-purpose function key. Usually found at the top of the keyboard.

### `F25`

General-purpose function key.

### `F26`

General-purpose function key.

### `F27`

General-purpose function key.

### `F28`

General-purpose function key.

### `F29`

General-purpose function key.

### `F30`

General-purpose function key.

### `F31`

General-purpose function key.

### `F32`

General-purpose function key.

### `F33`

General-purpose function key.

### `F34`

General-purpose function key.

### `F35`

General-purpose function key.

## Trait Implementations

### `impl Clone for Code`

#### `clone`

```rust
fn clone(&self) -> Code
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Code`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl Hash for Code`

#### `hash`

```rust
fn hash<__H: Hasher>(&self, state: &mut __H)
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl Ord for Code`

#### `cmp`

```rust
fn cmp(&self, other: &Code) -> Ordering
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

### `impl PartialEq<Code> for Physical`

#### `eq`

```rust
fn eq(&self, rhs: &Code) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq<Physical> for Code`

#### `eq`

```rust
fn eq(&self, rhs: &Physical) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialEq for Code`

#### `eq`

```rust
fn eq(&self, other: &Code) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd for Code`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Code) -> Option<Ordering>
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

### `impl Copy for Code`

### `impl Eq for Code`

### `impl StructuralPartialEq for Code`

## Auto Trait Implementations

- `impl Freeze for Code`
- `impl RefUnwindSafe for Code`
- `impl Send for Code`
- `impl Sync for Code`
- `impl Unpin for Code`
- `impl UnsafeUnpin for Code`
- `impl UnwindSafe for Code`
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
