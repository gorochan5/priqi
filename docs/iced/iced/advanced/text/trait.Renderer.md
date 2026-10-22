---
title: "Renderer"
kind: trait
path: "iced::advanced::text::Renderer"
crate: "iced"
module: "iced::advanced::text"
file: "iced/advanced/text/trait.Renderer.md"
docs_url: "https://docs.iced.rs/iced/advanced/text/trait.Renderer.html"
---

# `iced::advanced::text::Renderer`

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
        text: Text,
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

A renderer capable of measuring and drawing [`Text`](../struct.Text.md).

## Required Associated Types

### `Paragraph`

```rust
type Paragraph: Paragraph + 'static
```

The [`Paragraph`](trait.Paragraph.md) of this [`Renderer`](trait.Renderer.md).

### `Editor`

```rust
type Editor: Editor + 'static
```

The [`Editor`](trait.Editor.md) of this [`Renderer`](trait.Renderer.md).

## Required Methods

### `fill_paragraph`

```rust
fn fill_paragraph( &mut self, text: &Self::Paragraph, position: Point, color: Color, clip_bounds: Rectangle, )
```

Draws the given [`Paragraph`](trait.Paragraph.md) at the given position and with the given [`Color`](../../struct.Color.md).

### `fill_editor`

```rust
fn fill_editor( &mut self, editor: &Self::Editor, position: Point, color: Color, clip_bounds: Rectangle, )
```

Draws the given [`Editor`](trait.Editor.md) at the given position and with the given [`Color`](../../struct.Color.md).

### `fill_text`

```rust
fn fill_text( &mut self, text: Text, position: Point, color: Color, clip_bounds: Rectangle, )
```

Draws the given [`Text`](../struct.Text.md) at the given position and with the given [`Color`](../../struct.Color.md).

## Provided Methods

### `font`

```rust
fn font(&self) -> Font
```

Returns the default [`Font`](../../struct.Font.md).

### `text_size`

```rust
fn text_size(&self) -> Pixels
```

Returns the default size of [`Text`](../struct.Text.md).

### `line_height`

```rust
fn line_height(&self) -> LineHeight
```

Returns the default line height of [`Text`](../struct.Text.md).

## Implementors
