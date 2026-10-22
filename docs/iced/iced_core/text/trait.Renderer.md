---
title: "Renderer"
kind: trait
path: "iced_core::text::Renderer"
crate: "iced_core"
module: "iced_core::text"
file: "iced_core/text/trait.Renderer.md"
docs_url: "https://docs.iced.rs/iced_core/text/trait.Renderer.html"
source_file: "src/iced_core/text.rs"
source_lines: "312-401"
---

# `iced_core::text::Renderer`

```rust
pub trait Renderer: Renderer {
    type Paragraph: Paragraph + 'static;
    type Editor: Editor + 'static;

    const ICON_FONT: Font;
    const CHECKMARK_ICON: char;
    const ARROW_DOWN_ICON: char;
    const SCROLL_UP_ICON: char;
    const SCROLL_DOWN_ICON: char;
    const SCROLL_LEFT_ICON: char;
    const SCROLL_RIGHT_ICON: char;
    const ICED_LOGO: char;

    // Required methods
    fn fill_paragraph(
        &mut self,
        text: &Self::Paragraph,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    );
    fn fill_editor(
        &mut self,
        editor: &Self::Editor,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    );
    fn fill_text(
        &mut self,
        text: Text<String>,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    );

    // Provided methods
    fn font(&self) -> Font { ... }
    fn text_size(&self) -> Pixels { ... }
    fn line_height(&self) -> LineHeight { ... }
}
```

A renderer capable of measuring and drawing [`Text`](struct.Text.md).

**Source:** [text.rs](../../src/iced_core/text.rs.md) (lines 312-401)

## Required Associated Types

### `Paragraph`

```rust
type Paragraph: Paragraph + 'static
```

The [`Paragraph`](paragraph/trait.Paragraph.md) of this [`Renderer`](trait.Renderer.md).

### `Editor`

```rust
type Editor: Editor + 'static
```

The [`Editor`](editor/trait.Editor.md) of this [`Renderer`](trait.Renderer.md).

## Required Methods

### `fill_paragraph`

```rust
fn fill_paragraph( &mut self, text: &Self::Paragraph, position: Point, color: Color, clip_bounds: Rectangle, )
```

Draws the given [`Paragraph`](paragraph/trait.Paragraph.md) at the given position and with the given [`Color`](../color/struct.Color.md).

### `fill_editor`

```rust
fn fill_editor( &mut self, editor: &Self::Editor, position: Point, color: Color, clip_bounds: Rectangle, )
```

Draws the given [`Editor`](editor/trait.Editor.md) at the given position and with the given [`Color`](../color/struct.Color.md).

### `fill_text`

```rust
fn fill_text( &mut self, text: Text<String>, position: Point, color: Color, clip_bounds: Rectangle, )
```

Draws the given [`Text`](struct.Text.md) at the given position and with the given [`Color`](../color/struct.Color.md).

## Provided Methods

### `font`

```rust
fn font(&self) -> Font
```

Returns the default [`Font`](../font/struct.Font.md).

### `text_size`

```rust
fn text_size(&self) -> Pixels
```

Returns the default size of [`Text`](struct.Text.md).

### `line_height`

```rust
fn line_height(&self) -> LineHeight
```

Returns the default line height of [`Text`](struct.Text.md).

## Implementors
