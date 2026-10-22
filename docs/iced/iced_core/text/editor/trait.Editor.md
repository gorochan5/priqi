---
title: "Editor"
kind: trait
path: "iced_core::text::editor::Editor"
crate: "iced_core"
module: "iced_core::text::editor"
file: "iced_core/text/editor/trait.Editor.md"
docs_url: "https://docs.iced.rs/iced_core/text/editor/trait.Editor.html"
source_file: "src/iced_core/text/editor.rs"
source_lines: "22-120"
---

# `iced_core::text::editor::Editor`

```rust
pub trait Editor: Sized + Default {
Show 20 methods    // Required methods
    fn with_text(text: &str) -> Self;
    fn is_empty(&self) -> bool;
    fn cursor(&self) -> Cursor;
    fn selection(&self) -> Option<Selection>;
    fn copy(&self) -> Option<String>;
    fn line(&self, index: usize) -> Option<Line<'_>>;
    fn line_count(&self) -> usize;
    fn perform(&mut self, action: Action);
    fn move_to(&mut self, cursor: Cursor);
    fn bounds(&self) -> Size;
    fn min_bounds(&self) -> Size;
    fn hint_factor(&self) -> Option<f32>;
    fn update(
        &mut self,
        new_bounds: Size,
        new_font: Font,
        new_size: Pixels,
        new_line_height: LineHeight,
        new_wrapping: Wrapping,
        new_alignment: Alignment,
        new_hint_factor: Option<f32>,
        new_parser: &mut impl Parser,
    );
    fn overwrite(&mut self, new_text: &str);
    fn highlight<P: Parser>(
        &mut self,
        font: Font,
        parser: &mut P,
        highlight: impl Fn(P::Output) -> Style,
    );
    fn font(&self) -> Font;
    fn text_size(&self) -> Pixels;
    fn line_height(&self) -> LineHeight;

    // Provided methods
    fn lines(&self) -> impl Iterator<Item = Line<'_>> { ... }
    fn text(&self) -> String { ... }
}
```

A component that can be used by widgets to edit multi-line text.

**Source:** [editor.rs](../../../src/iced_core/text/editor.rs.md) (lines 22-120)

## Required Methods

### `with_text`

```rust
fn with_text(text: &str) -> Self
```

Creates a new [`Editor`](trait.Editor.md) laid out with the given text.

### `is_empty`

```rust
fn is_empty(&self) -> bool
```

Returns true if the [`Editor`](trait.Editor.md) has no contents.

### `cursor`

```rust
fn cursor(&self) -> Cursor
```

Returns the current [`Cursor`](struct.Cursor.md) of the [`Editor`](trait.Editor.md).

### `selection`

```rust
fn selection(&self) -> Option<Selection>
```

Returns the current [`Selection`](enum.Selection.md) of the [`Editor`](trait.Editor.md), if visible.

### `copy`

```rust
fn copy(&self) -> Option<String>
```

Returns the current selected text of the [`Editor`](trait.Editor.md).

### `line`

```rust
fn line(&self, index: usize) -> Option<Line<'_>>
```

Returns the text of the given line in the [`Editor`](trait.Editor.md), if it exists.

### `line_count`

```rust
fn line_count(&self) -> usize
```

Returns the amount of lines in the [`Editor`](trait.Editor.md).

### `perform`

```rust
fn perform(&mut self, action: Action)
```

Performs an [`Action`](enum.Action.md) on the [`Editor`](trait.Editor.md).

### `move_to`

```rust
fn move_to(&mut self, cursor: Cursor)
```

Moves the cursor to the given position.

### `bounds`

```rust
fn bounds(&self) -> Size
```

Returns the current boundaries of the [`Editor`](trait.Editor.md).

### `min_bounds`

```rust
fn min_bounds(&self) -> Size
```

Returns the minimum boundaries to fit the current contents of the [`Editor`](trait.Editor.md).

### `hint_factor`

```rust
fn hint_factor(&self) -> Option<f32>
```

Returns the hint factor of the [`Editor`](trait.Editor.md).

### `update`

```rust
fn update( &mut self, new_bounds: Size, new_font: Font, new_size: Pixels, new_line_height: LineHeight, new_wrapping: Wrapping, new_alignment: Alignment, new_hint_factor: Option<f32>, new_parser: &mut impl Parser, )
```

Updates the [`Editor`](trait.Editor.md) with some new attributes.

### `overwrite`

```rust
fn overwrite(&mut self, new_text: &str)
```

Overwrites the current contents of the [`Editor`](trait.Editor.md).

### `highlight`

```rust
fn highlight<P: Parser>( &mut self, font: Font, parser: &mut P, highlight: impl Fn(P::Output) -> Style, )
```

Runs a [`text::Highlighter`](../highlighter/trait.Highlighter.md) in the [`Editor`](trait.Editor.md).

### `font`

```rust
fn font(&self) -> Font
```

Returns the current [`Font`](../../font/struct.Font.md) of the [`Editor`](trait.Editor.md).

### `text_size`

```rust
fn text_size(&self) -> Pixels
```

Returns the current text size of the [`Editor`](trait.Editor.md).

### `line_height`

```rust
fn line_height(&self) -> LineHeight
```

Returns the current [`LineHeight`](../enum.LineHeight.md) of the [`Editor`](trait.Editor.md).

## Provided Methods

### `lines`

```rust
fn lines(&self) -> impl Iterator<Item = Line<'_>>
```

Returns an iterator of the text of the lines in the [`Editor`](trait.Editor.md).

### `text`

```rust
fn text(&self) -> String
```

Returns the text of the [`Editor`](trait.Editor.md).

## Implementors
