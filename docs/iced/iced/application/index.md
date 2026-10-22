---
title: "application"
kind: module
path: "iced::application"
crate: "iced"
module: "iced"
file: "iced/application/index.md"
docs_url: "https://docs.iced.rs/iced/application/index.html"
source_file: "src/iced/application.rs"
source_lines: "1-956"
---

# `iced::application`

Create and run iced applications step by step.

## Example

```rust
use iced::widget::{button, column, text, Widget};
use iced::Theme;

pub fn main() -> iced::Result {
    iced::application(u64::default, update, view)
        .theme(Theme::Dark)
        .centered()
        .run()
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

## Re-exports

- [`timed`](timed/fn.timed.md)

## Modules

- [`timed`](timed/index.md) — An [`Application`](struct.Application.md) that receives an [`Instant`](../time/struct.Instant.md) in update logic.

## Structs

- [`Application`](struct.Application.md) — The underlying definition and configuration of an iced application.

## Traits

- [`BootFn`](trait.BootFn.md) — The logic to initialize the `State` of some [`Application`](struct.Application.md).
- [`IntoBoot`](trait.IntoBoot.md) — The initial state of some [`Application`](struct.Application.md).
- [`ThemeFn`](trait.ThemeFn.md) — The theme logic of some [`Application`](struct.Application.md).
- [`TitleFn`](trait.TitleFn.md) — The title logic of some [`Application`](struct.Application.md).
- [`UpdateFn`](trait.UpdateFn.md) — The update logic of some [`Application`](struct.Application.md).
- [`ViewFn`](trait.ViewFn.md) — The view logic of some [`Application`](struct.Application.md).

## Functions

- [`application`](fn.application.md) — Creates an iced [`Application`](struct.Application.md) given its boot, update, and view logic.
