---
title: "Notifier"
kind: trait
path: "iced_graphics::shell::Notifier"
crate: "iced_graphics"
module: "iced_graphics::shell"
file: "iced_graphics/shell/trait.Notifier.md"
docs_url: "https://docs.iced.rs/iced_graphics/shell/trait.Notifier.html"
source_file: "src/iced_graphics/shell.rs"
source_lines: "45-55"
---

# `iced_graphics::shell::Notifier`

```rust
pub trait Notifier:
    Send
    + Sync
    + 'static {
    // Required methods
    fn tick(&self);
    fn request_redraw(&self);
    fn invalidate_layout(&self);
}
```

A type that can notify a shell of certain events.

**Source:** [shell.rs](../../src/iced_graphics/shell.rs.md) (lines 45-55)

## Required Methods

### `tick`

```rust
fn tick(&self)
```

Requests for `Renderer::tick` to be called by the [`Shell`](struct.Shell.md).

### `request_redraw`

```rust
fn request_redraw(&self)
```

Requests for all windows of the [`Shell`](struct.Shell.md) to be redrawn.

### `invalidate_layout`

```rust
fn invalidate_layout(&self)
```

Requests for all layouts of the [`Shell`](struct.Shell.md) to be recomputed.

## Implementors
