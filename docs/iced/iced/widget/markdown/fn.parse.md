---
title: "parse"
kind: function
path: "iced::widget::markdown::parse"
crate: "iced"
module: "iced::widget::markdown"
file: "iced/widget/markdown/fn.parse.md"
docs_url: "https://docs.iced.rs/iced/widget/markdown/fn.parse.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1001"
---

# `iced::widget::markdown::parse`

```rust
pub fn parse(markdown: &str) -> impl Iterator<Item = Item>
```

Parse the given Markdown content.

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

**Source:** [markdown.rs](../../../src/iced_widget/markdown.rs.md) (lines 1001)
