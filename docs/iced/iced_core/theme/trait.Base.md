---
title: "Base"
kind: trait
path: "iced_core::theme::Base"
crate: "iced_core"
module: "iced_core::theme"
file: "iced_core/theme/trait.Base.md"
docs_url: "https://docs.iced.rs/iced_core/theme/trait.Base.html"
source_file: "src/iced_core/theme.rs"
source_lines: "228-249"
---

# `iced_core::theme::Base`

```rust
pub trait Base: 'static {
    // Required methods
    fn default(preference: Mode) -> Self;
    fn mode(&self) -> Mode;
    fn base(&self) -> Style;
    fn seed(&self) -> Option<Seed>;
    fn name(&self) -> &str;
}
```

The default blank style of a theme.

**Source:** [theme.rs](../../src/iced_core/theme.rs.md) (lines 228-249)

## Required Methods

### `default`

```rust
fn default(preference: Mode) -> Self
```

Returns the default theme for the preferred [`Mode`](enum.Mode.md).

### `mode`

```rust
fn mode(&self) -> Mode
```

Returns the [`Mode`](enum.Mode.md) of the theme.

### `base`

```rust
fn base(&self) -> Style
```

Returns the default base [`Style`](struct.Style.md) of the theme.

### `seed`

```rust
fn seed(&self) -> Option<Seed>
```

Returns the [`Seed`](palette/struct.Seed.md) of the theme.

This may be used by the runtime to recreate a [`Theme`](enum.Theme.md) for debugging purposes; like displaying performance metrics or devtools.

### `name`

```rust
fn name(&self) -> &str
```

Returns the unique name of the theme.

This name may be used to efficiently detect theme changes in some widgets.

## Implementors

- `impl Base for Theme`
