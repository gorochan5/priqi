---
title: "Paragraph"
kind: trait
path: "iced_core::text::paragraph::Paragraph"
crate: "iced_core"
module: "iced_core::text::paragraph"
file: "iced_core/text/paragraph/trait.Paragraph.md"
docs_url: "https://docs.iced.rs/iced_core/text/paragraph/trait.Paragraph.html"
source_file: "src/iced_core/text/paragraph.rs"
source_lines: "9-109"
---

# `iced_core::text::paragraph::Paragraph`

```rust
pub trait Paragraph: Sized + Default {
Show 26 methods    // Required methods
    fn with_text(text: Text<&str>) -> Self;
    fn with_spans<Link>(text: Text<&[Span<'_, Link>]>) -> Self;
    fn resize(&mut self, new_bounds: Size);
    fn compare(&self, text: Text<()>) -> Difference;
    fn size(&self) -> Pixels;
    fn hint_factor(&self) -> Option<f32>;
    fn font(&self) -> Font;
    fn line_height(&self) -> LineHeight;
    fn align_x(&self) -> Alignment;
    fn align_y(&self) -> Vertical;
    fn wrapping(&self) -> Wrapping;
    fn ellipsis(&self) -> Ellipsis;
    fn shaping(&self) -> Shaping;
    fn bounds(&self) -> Size;
    fn min_bounds(&self) -> Size;
    fn hit_test(&self, point: Point) -> Option<Hit>;
    fn hit_span(&self, point: Point) -> Option<usize>;
    fn span_bounds(&self, index: usize) -> Vec<Rectangle>;
    fn select(&mut self, start: Point, end: Point, target: Target);
    fn select_all(&mut self);
    fn deselect(&mut self);
    fn selection(&self) -> &[Rectangle];
    fn copy(&mut self) -> Option<String>;

    // Provided methods
    fn hit_glyph(&self, point: Point) -> bool { ... }
    fn min_width(&self) -> f32 { ... }
    fn min_height(&self) -> f32 { ... }
}
```

A text paragraph.

**Source:** [paragraph.rs](../../../src/iced_core/text/paragraph.rs.md) (lines 9-109)

## Required Methods

### `with_text`

```rust
fn with_text(text: Text<&str>) -> Self
```

Creates a new [`Paragraph`](trait.Paragraph.md) laid out with the given [`Text`](../struct.Text.md).

### `with_spans`

```rust
fn with_spans<Link>(text: Text<&[Span<'_, Link>]>) -> Self
```

Creates a new [`Paragraph`](trait.Paragraph.md) laid out with the given [`Text`](../struct.Text.md).

### `resize`

```rust
fn resize(&mut self, new_bounds: Size)
```

Lays out the [`Paragraph`](trait.Paragraph.md) with some new boundaries.

### `compare`

```rust
fn compare(&self, text: Text<()>) -> Difference
```

Compares the [`Paragraph`](trait.Paragraph.md) with some desired [`Text`](../struct.Text.md) and returns the [`Difference`](../enum.Difference.md).

### `size`

```rust
fn size(&self) -> Pixels
```

Returns the text size of the [`Paragraph`](trait.Paragraph.md) in [`Pixels`](../../struct.Pixels.md).

### `hint_factor`

```rust
fn hint_factor(&self) -> Option<f32>
```

Returns the hint factor of the [`Paragraph`](trait.Paragraph.md).

### `font`

```rust
fn font(&self) -> Font
```

Returns the [`Font`](../../font/struct.Font.md) of the [`Paragraph`](trait.Paragraph.md).

### `line_height`

```rust
fn line_height(&self) -> LineHeight
```

Returns the [`LineHeight`](../enum.LineHeight.md) of the [`Paragraph`](trait.Paragraph.md).

### `align_x`

```rust
fn align_x(&self) -> Alignment
```

Returns the horizontal alignment of the [`Paragraph`](trait.Paragraph.md).

### `align_y`

```rust
fn align_y(&self) -> Vertical
```

Returns the vertical alignment of the [`Paragraph`](trait.Paragraph.md).

### `wrapping`

```rust
fn wrapping(&self) -> Wrapping
```

Returns the [`Wrapping`](../enum.Wrapping.md) strategy of the [`Paragraph`](trait.Paragraph.md)>

### `ellipsis`

```rust
fn ellipsis(&self) -> Ellipsis
```

Returns the [`Ellipsis`](../enum.Ellipsis.md) strategy of the [`Paragraph`](trait.Paragraph.md)>

### `shaping`

```rust
fn shaping(&self) -> Shaping
```

Returns the [`Shaping`](../enum.Shaping.md) strategy of the [`Paragraph`](trait.Paragraph.md)>

### `bounds`

```rust
fn bounds(&self) -> Size
```

Returns the available bounds used to layout the [`Paragraph`](trait.Paragraph.md).

### `min_bounds`

```rust
fn min_bounds(&self) -> Size
```

Returns the minimum boundaries that can fit the contents of the [`Paragraph`](trait.Paragraph.md).

### `hit_test`

```rust
fn hit_test(&self, point: Point) -> Option<Hit>
```

Tests whether the provided point is within the boundaries of the [`Paragraph`](trait.Paragraph.md), returning information about the nearest character.

### `hit_span`

```rust
fn hit_span(&self, point: Point) -> Option<usize>
```

Tests whether the provided point is within the boundaries of a [`Span`](../struct.Span.md) in the [`Paragraph`](trait.Paragraph.md), returning the index of the [`Span`](../struct.Span.md) that was hit.

### `span_bounds`

```rust
fn span_bounds(&self, index: usize) -> Vec<Rectangle>
```

Returns all bounds for the provided [`Span`](../struct.Span.md) index of the [`Paragraph`](trait.Paragraph.md). A [`Span`](../struct.Span.md) can have multiple bounds for each line it’s on.

### `select`

```rust
fn select(&mut self, start: Point, end: Point, target: Target)
```

Selects the text between the given positions.

The positions are relative to the origin of the [`Paragraph`](trait.Paragraph.md). `start` is the position where the selection began, while `end` is its final position. The [`Target`](../enum.Target.md) determines how the selection is expanded around these positions.

### `select_all`

```rust
fn select_all(&mut self)
```

Selects all the text of the [`Paragraph`](trait.Paragraph.md).

### `deselect`

```rust
fn deselect(&mut self)
```

Deselects any selected text.

### `selection`

```rust
fn selection(&self) -> &[Rectangle]
```

Returns the regions of the current selection of the [`Paragraph`](trait.Paragraph.md).

Each [`Rectangle`](../../struct.Rectangle.md) corresponds to a line of the selection.

### `copy`

```rust
fn copy(&mut self) -> Option<String>
```

Returns the current selected text of the [`Paragraph`](trait.Paragraph.md).

## Provided Methods

### `hit_glyph`

```rust
fn hit_glyph(&self, point: Point) -> bool
```

Tests whether the provided point is inside the boundaries of a glyph in the [`Paragraph`](trait.Paragraph.md).

### `min_width`

```rust
fn min_width(&self) -> f32
```

Returns the minimum width that can fit the contents of the [`Paragraph`](trait.Paragraph.md).

### `min_height`

```rust
fn min_height(&self) -> f32
```

Returns the minimum height that can fit the contents of the [`Paragraph`](trait.Paragraph.md).

## Implementors
