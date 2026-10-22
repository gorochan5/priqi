---
title: "Text"
kind: type
path: "iced::widget::Text"
crate: "iced"
module: "iced::widget"
file: "iced/widget/type.Text.md"
docs_url: "https://docs.iced.rs/iced/widget/type.Text.html"
source_file: "src/iced_widget/text.rs"
source_lines: "29"
---

# `iced::widget::Text`

```rust
pub type Text<'a, Theme = Theme> = Text<'a, Theme>;
```

A bunch of text.

## Example

```rust
use iced::widget::text;
use iced::color;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    text("Hello, this is iced!")
        .size(20)
        .color(color!(0x0000ff))
}
```

**Source:** [text.rs](../../src/iced_widget/text.rs.md) (lines 29)
