---
title: "markdown"
kind: function
path: "iced_widget::markdown"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.markdown.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.markdown.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1759-1777"
---

# `iced_widget::markdown`

```rust
pub fn markdown<'a, Theme, Renderer>(
    items: &'a [Item],
    settings: impl Into<Settings>,
    theme: Theme,
) -> impl Widget<Uri, Theme, Renderer> + 'awhere
    Theme: Catalog + 'static,
    Renderer: Renderer,
```

Display a bunch of Markdown items.

You can obtain the items with [`parse`](markdown/fn.parse.md).

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

**Source:** [markdown.rs](../src/iced_widget/markdown.rs.md) (lines 1759-1777)
