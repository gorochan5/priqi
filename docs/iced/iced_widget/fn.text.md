---
title: "text"
kind: function
path: "iced_widget::text"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.text.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.text.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "825-830"
---

# `iced_widget::text`

```rust
pub fn text<'a, Theme>(text: impl IntoFragment<'a>) -> Text<'a, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Text`](text/type.Text.md) widget with the provided content.

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

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 825-830)
