---
title: "Viewer"
kind: trait
path: "iced::widget::markdown::Viewer"
crate: "iced"
module: "iced::widget::markdown"
file: "iced/widget/markdown/trait.Viewer.md"
docs_url: "https://docs.iced.rs/iced/widget/markdown/trait.Viewer.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "2132"
---

# `iced::widget::markdown::Viewer`

```rust
pub trait Viewer<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>: Sized + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a, {
    // Required methods
    fn theme(&self) -> &Theme;
    fn highlighter(&self) -> &dyn Highlighter<Code, Theme>;
    fn on_link_click(url: String) -> Message;

    // Provided methods
    fn image(
        &self,
        settings: Settings,
        url: &'a String,
        title: &'a str,
        alt: &Text,
    ) -> impl Widget<Message, Theme, Renderer> + 'a { ... }
    fn heading(
        &self,
        settings: Settings,
        level: &'a HeadingLevel,
        text: &'a Text,
    ) -> impl Widget<Message, Theme, Renderer> + 'a { ... }
    fn paragraph(
        &self,
        settings: Settings,
        text: &Text,
    ) -> impl Widget<Message, Theme, Renderer> + 'a { ... }
    fn code_block(
        &self,
        settings: Settings,
        language: Option<&'a str>,
        code: &'a str,
        lines: &'a [Text],
    ) -> impl Widget<Message, Theme, Renderer> + 'a { ... }
    fn unordered_list(
        &self,
        settings: Settings,
        bullets: &'a [Bullet],
    ) -> impl Widget<Message, Theme, Renderer> + 'a { ... }
    fn ordered_list(
        &self,
        settings: Settings,
        start: u64,
        bullets: &'a [Bullet],
    ) -> impl Widget<Message, Theme, Renderer> + 'a { ... }
    fn quote(
        &self,
        settings: Settings,
        contents: &'a [Item],
    ) -> impl Widget<Message, Theme, Renderer> + 'a { ... }
    fn rule(&self) -> impl Widget<Message, Theme, Renderer> + 'a { ... }
    fn table(
        &self,
        settings: Settings,
        columns: &'a [Column],
        rows: &'a [Row],
    ) -> impl Widget<Message, Theme, Renderer> + 'a { ... }
}
```

A view strategy to display a Markdown [`Item`](enum.Item.md).

A [`Viewer`](trait.Viewer.md) is in charge of turning each [`Item`](enum.Item.md) into a widget. It also provides the [`Theme`](../../enum.Theme.md) and [`text::Highlighter`](../../advanced/text/trait.Highlighter.md) used for rendering.

**Source:** [markdown.rs](../../../src/iced_widget/markdown.rs.md) (lines 2132)

## Required Methods

### `theme`

```rust
fn theme(&self) -> &Theme
```

The [`Theme`](../../enum.Theme.md) used for styling the Markdown elements.

### `highlighter`

```rust
fn highlighter(&self) -> &dyn Highlighter<Code, Theme>
```

The [`text::Highlighter`](../../advanced/text/trait.Highlighter.md) used for highligthing [`Code`](../../enum.Code.md) regions.

### `on_link_click`

```rust
fn on_link_click(url: String) -> Message
```

Produces a message when a link is clicked with the given [`Uri`](type.Uri.md).

## Provided Methods

### `image`

```rust
fn image( &self, settings: Settings, url: &'a String, title: &'a str, alt: &Text, ) -> impl Widget<Message, Theme, Renderer> + 'a
```

Displays an image.

By default, it will show a container with the image title.

### `heading`

```rust
fn heading( &self, settings: Settings, level: &'a HeadingLevel, text: &'a Text, ) -> impl Widget<Message, Theme, Renderer> + 'a
```

Displays a heading.

By default, it calls [`heading`](fn.heading.md).

### `paragraph`

```rust
fn paragraph( &self, settings: Settings, text: &Text, ) -> impl Widget<Message, Theme, Renderer> + 'a
```

Displays a paragraph.

By default, it calls [`paragraph`](fn.paragraph.md).

### `code_block`

```rust
fn code_block( &self, settings: Settings, language: Option<&'a str>, code: &'a str, lines: &'a [Text], ) -> impl Widget<Message, Theme, Renderer> + 'a
```

Displays a code block.

By default, it calls [`code_block`](fn.code_block.md).

### `unordered_list`

```rust
fn unordered_list( &self, settings: Settings, bullets: &'a [Bullet], ) -> impl Widget<Message, Theme, Renderer> + 'a
```

Displays an unordered list.

By default, it calls [`unordered_list`](fn.unordered_list.md).

### `ordered_list`

```rust
fn ordered_list( &self, settings: Settings, start: u64, bullets: &'a [Bullet], ) -> impl Widget<Message, Theme, Renderer> + 'a
```

Displays an ordered list.

By default, it calls [`ordered_list`](fn.ordered_list.md).

### `quote`

```rust
fn quote( &self, settings: Settings, contents: &'a [Item], ) -> impl Widget<Message, Theme, Renderer> + 'a
```

Displays a quote.

By default, it calls [`quote`](fn.quote.md).

### `rule`

```rust
fn rule(&self) -> impl Widget<Message, Theme, Renderer> + 'a
```

Displays a rule.

By default, it calls [`rule`](fn.rule.md).

### `table`

```rust
fn table( &self, settings: Settings, columns: &'a [Column], rows: &'a [Row], ) -> impl Widget<Message, Theme, Renderer> + 'a
```

Displays a table.

By default, it calls [`table`](fn.table.md).

## Implementors

- `impl<'a, Theme, Renderer> Viewer<'a, String, Theme, Renderer> for DefaultViewer<'a, Theme> where Theme: Catalog + 'a, Renderer: Renderer + 'a,`
