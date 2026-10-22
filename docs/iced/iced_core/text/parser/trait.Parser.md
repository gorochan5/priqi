---
title: "Parser"
kind: trait
path: "iced_core::text::parser::Parser"
crate: "iced_core"
module: "iced_core::text::parser"
file: "iced_core/text/parser/trait.Parser.md"
docs_url: "https://docs.iced.rs/iced_core/text/parser/trait.Parser.html"
source_file: "src/iced_core/text/parser.rs"
source_lines: "9-41"
---

# `iced_core::text::parser::Parser`

```rust
pub trait Parser: 'static {
    type Settings: PartialEq + Clone;
    type Output;
    type Iterator<'a>: Iterator<Item = (Range<usize>, Self::Output)>
       where Self: 'a;

    // Required methods
    fn new(settings: &Self::Settings) -> Self;
    fn update(&mut self, new_settings: &Self::Settings);
    fn change_line(&mut self, line: usize);
    fn parse_line(&mut self, line: &str) -> Self::Iterator<'_>;
    fn current_line(&self) -> usize;
}
```

A type capable of parsing text, producing some `Output`.

A [`Parser`](trait.Parser.md) parses lines in sequence. When a line changes, it must be notified and the lines after the changed one must be fed again to the [`Parser`](trait.Parser.md).

**Source:** [parser.rs](../../../src/iced_core/text/parser.rs.md) (lines 9-41)

## Required Associated Types

### `Settings`

```rust
type Settings: PartialEq + Clone
```

The settings to configure the [`Parser`](trait.Parser.md).

### `Output`

```rust
type Output
```

The output of this [`Parser`](trait.Parser.md).

### `Iterator`

```rust
type Iterator<'a>: Iterator<Item = (Range<usize>, Self::Output)> where Self: 'a
```

The parse iterator type.

## Required Methods

### `new`

```rust
fn new(settings: &Self::Settings) -> Self
```

Creates a new [`Parser`](trait.Parser.md) from its [`Self::Settings`](trait.Parser.md).

### `update`

```rust
fn update(&mut self, new_settings: &Self::Settings)
```

Updates the [`Parser`](trait.Parser.md) with some new [`Self::Settings`](trait.Parser.md).

### `change_line`

```rust
fn change_line(&mut self, line: usize)
```

Notifies the [`Parser`](trait.Parser.md) that the line at the given index has changed.

### `parse_line`

```rust
fn parse_line(&mut self, line: &str) -> Self::Iterator<'_>
```

Parses the given line.

If a line changed prior to this, the first line provided here will be the line that changed.

### `current_line`

```rust
fn current_line(&self) -> usize
```

Returns the current line of the [`Parser`](trait.Parser.md).

If `change_line` has been called, this will normally be the least index that changed.

## Implementors

- `impl Parser for PlainText`
  - `type Settings = ()`
  - `type Output = ()`
  - `type Iterator<'a> = Empty<(Range<usize>, ())>`
