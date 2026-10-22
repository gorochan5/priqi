---
title: "Fragment"
kind: type
path: "iced_core::text::Fragment"
crate: "iced_core"
module: "iced_core::text"
file: "iced_core/text/type.Fragment.md"
docs_url: "https://docs.iced.rs/iced_core/text/type.Fragment.html"
source_file: "src/iced_core/text.rs"
source_lines: "655"
---

# `iced_core::text::Fragment`

```rust
pub type Fragment<'a> = Cow<'a, str>;
```

A fragment of [`Text`](struct.Text.md).

This is just an alias to a string that may be either borrowed or owned.

**Source:** [text.rs](../../src/iced_core/text.rs.md) (lines 655)

## Variants

### `Borrowed(&'a str)`

Borrowed data.

### `Owned(String)`

Owned data.

## Trait Implementations

### `impl<'a> IntoFragment<'a> for &'a Fragment<'_>`

#### `into_fragment`

```rust
fn into_fragment(self) -> Fragment<'a>
```

Converts the value to some text [`Fragment`](type.Fragment.md).

### `impl<'a> IntoFragment<'a> for Fragment<'a>`

#### `into_fragment`

```rust
fn into_fragment(self) -> Fragment<'a>
```

Converts the value to some text [`Fragment`](type.Fragment.md).
