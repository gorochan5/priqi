---
title: "application"
kind: function
path: "iced::application"
crate: "iced"
module: "iced"
file: "iced/fn.application.md"
docs_url: "https://docs.iced.rs/iced/fn.application.html"
source_file: "src/iced/application.rs"
source_lines: "82-166"
---

# `iced::application`

```rust
pub fn application<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = Message, Theme = Theme>> where
    State: 'static,
    Message: Send + 'static,
    Theme: Base,
    Renderer: Renderer,
```

Creates an iced [`Application`](application/struct.Application.md) given its boot, update, and view logic.

## Example

```rust
use iced::widget::{button, column, text, Widget};

pub fn main() -> iced::Result {
    iced::application(u64::default, update, view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(value: &mut u64, message: Message) {
    match message {
        Message::Increment => *value += 1,
    }
}

fn view(value: &u64) -> impl Widget<Message> {
    column![
        text(value),
        button("+").on_press(Message::Increment),
    ]
}
```

**Source:** [application.rs](../src/iced/application.rs.md) (lines 82-166)
