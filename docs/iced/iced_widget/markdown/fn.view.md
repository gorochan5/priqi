---
title: "view"
kind: function
path: "iced_widget::markdown::view"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.view.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.view.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1759-1777"
---

# `iced_widget::markdown::view`

```rust
pub fn view<'a, Theme, Renderer>(
    items: &'a [Item],
    settings: impl Into<Settings>,
    theme: Theme,
) -> impl Widget<Uri, Theme, Renderer> + 'awhere
    Theme: Catalog + 'static,
    Renderer: Renderer,
```

Display a bunch of Markdown items.

You can obtain the items with [`parse`](fn.parse.md).

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

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 1759-1777)
