---
title: "markdown"
kind: module
path: "iced::widget::markdown"
crate: "iced"
module: "iced::widget"
file: "iced/widget/markdown/index.md"
docs_url: "https://docs.iced.rs/iced/widget/markdown/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "156"
---

# `iced::widget::markdown`

Markdown widgets can parse and display Markdown.

You can enable the `highlighter` feature for syntax highlighting in code blocks.

Only the variants of [`Item`](enum.Item.md) are currently supported.

## Example

```rust
use iced::widget::markdown;
use iced::{Theme, Widget};

struct State {
   markdown: Vec<markdown::Item>,
}

enum Message {
    LinkClicked(markdown::Uri),
}

impl State {
    pub fn new() -> Self {
        Self {
            markdown: markdown::parse("This is some **Markdown**!").collect(),
        }
    }

    fn view(&self) -> impl Widget<Message> {
        markdown::view(
            &self.markdown,
            markdown::Settings::default(),
            Theme::TokyoNight,
        )
            .map(Message::LinkClicked)
    }

    fn update(state: &mut State, message: Message) {
        match message {
            Message::LinkClicked(url) => {
                println!("The following url was clicked: {url}");
            }
        }
    }
}
```

## Structs

- [`Column`](struct.Column.md) — The column of a table.
- [`Content`](struct.Content.md) — A bunch of Markdown that has been parsed.
- [`DefaultViewer`](struct.DefaultViewer.md) — The default [`Viewer`](trait.Viewer.md).
- [`Highlight`](struct.Highlight.md) — A text highlight.
- [`InlineCode`](struct.InlineCode.md) — The style of some inline code.
- [`Row`](struct.Row.md) — The row of a table.
- [`Settings`](struct.Settings.md) — Configuration controlling Markdown rendering in [`view`](../fn.markdown.md).
- [`Text`](struct.Text.md) — A bunch of parsed Markdown text.

## Enums

- [`Bullet`](enum.Bullet.md) — The item of a list.
- [`HeadingLevel`](enum.HeadingLevel.md)
- [`Item`](enum.Item.md) — A Markdown item.

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of Markdown items.
- [`Highlighter`](trait.Highlighter.md) — A type that describes how to highlight an `Input` with some [`Style`](../../advanced/text/highlighter/struct.Style.md).
- [`Viewer`](trait.Viewer.md) — A view strategy to display a Markdown [`Item`](enum.Item.md).

## Functions

- [`code_block`](fn.code_block.md) — Displays a code block using the default look.
- [`heading`](fn.heading.md) — Displays a heading using the default look.
- [`item`](fn.item.md) — Displays an [`Item`](enum.Item.md) using the given [`Viewer`](trait.Viewer.md).
- [`items`](fn.items.md) — Displays a column of items with the default look.
- [`ordered_list`](fn.ordered_list.md) — Displays an ordered list using the default look and calling the [`Viewer`](trait.Viewer.md) for each numbered item.
- [`paragraph`](fn.paragraph.md) — Displays a paragraph using the default look.
- [`parse`](fn.parse.md) — Parse the given Markdown content.
- [`quote`](fn.quote.md) — Displays a quote using the default look.
- [`rule`](fn.rule.md) — Displays a rule using the default look.
- [`sections`](fn.sections.md) — Groups the given Markdown [`Item`](enum.Item.md)s by [`Item::Heading`](enum.Item.md).
- [`table`](fn.table.md) — Displays a table using the default look.
- [`unordered_list`](fn.unordered_list.md) — Displays an unordered list using the default look and calling the [`Viewer`](trait.Viewer.md) for each bullet point item.
- [`view`](fn.view.md) — Display a bunch of Markdown items.
- [`view_with`](fn.view_with.md) — Runs [`view`](../fn.markdown.md) but with a custom [`Viewer`](trait.Viewer.md) to turn an [`Item`](enum.Item.md) into a widget.

## Type Aliases

- [`Uri`](type.Uri.md) — A [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html) representing a [URI](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier) in a Markdown document
