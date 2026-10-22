---
title: "Program"
kind: trait
path: "iced_widget::transition::Program"
crate: "iced_widget"
module: "iced_widget::transition"
file: "iced_widget/transition/trait.Program.md"
docs_url: "https://docs.iced.rs/iced_widget/transition/trait.Program.html"
source_file: "src/iced_widget/transition.rs"
source_lines: "15-25"
---

# `iced_widget::transition::Program`

```rust
pub trait Program: 'static {
    type Value: Copy + 'static;

    // Required methods
    fn go(&mut self, value: Self::Value, now: Instant);
    fn is_animating(&self, now: Instant) -> bool;
}
```

The logic of a [`Transition`](struct.Transition.md).

**Source:** [transition.rs](../../src/iced_widget/transition.rs.md) (lines 15-25)

## Required Associated Types

### `Value`

```rust
type Value: Copy + 'static
```

The type of value that the [`Program`](trait.Program.md) animates.

## Required Methods

### `go`

```rust
fn go(&mut self, value: Self::Value, now: Instant)
```

Transitions the [`Program`](trait.Program.md) from its current state towards the given value at the given time.

### `is_animating`

```rust
fn is_animating(&self, now: Instant) -> bool
```

Returns true if the [`Program`](trait.Program.md) is currently in progress.

## Implementors
