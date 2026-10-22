---
title: "run"
kind: function
path: "iced::run"
crate: "iced"
module: "iced"
file: "iced/fn.run.md"
docs_url: "https://docs.iced.rs/iced/fn.run.html"
source_file: "src/iced/lib.rs"
source_lines: "712-723"
---

# `iced::run`

```rust
pub fn run<State, Message, Theme, Renderer>(
    update: impl UpdateFn<State, Message> + 'static,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer> + 'static,
) -> Resultwhere
    State: Default + 'static,
    Message: Send + MaybeDebug + MaybeClone + 'static,
    Theme: Base + 'static,
    Renderer: Renderer + 'static,
```

Runs a basic iced application with default [`Settings`](struct.Settings.md) given its update and view logic.

This is equivalent to chaining [`application()`](fn.application.md) with [`Application::run`](application/struct.Application.md).

## Example

```rust
use iced::widget::{button, column, text, Widget};

pub fn main() -> iced::Result {
    iced::run(update, view)
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

**Source:** [lib.rs](../src/iced/lib.rs.md) (lines 712-723)
