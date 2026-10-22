---
title: "Key"
kind: trait
path: "iced_widget::sensor::Key"
crate: "iced_widget"
module: "iced_widget::sensor"
file: "iced_widget/sensor/trait.Key.md"
docs_url: "https://docs.iced.rs/iced_widget/sensor/trait.Key.html"
source_file: "src/iced_widget/sensor.rs"
source_lines: "334-343"
---

# `iced_widget::sensor::Key`

```rust
pub trait Key {
    type Owned: 'static;

    // Required methods
    fn to_owned(&self) -> Self::Owned;
    fn eq(&self, other: &Self::Owned) -> bool;
}
```

The key of a widget.

You should generally not need to care about this trait.

**Source:** [sensor.rs](../../src/iced_widget/sensor.rs.md) (lines 334-343)

## Required Associated Types

### `Owned`

```rust
type Owned: 'static
```

The owned version of the key.

## Required Methods

### `to_owned`

```rust
fn to_owned(&self) -> Self::Owned
```

Returns the owned version of the key.

### `eq`

```rust
fn eq(&self, other: &Self::Owned) -> bool
```

Compares the key with the given owned version.

## Implementors
