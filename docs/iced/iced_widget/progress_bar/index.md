---
title: "progress_bar"
kind: module
path: "iced_widget::progress_bar"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/progress_bar/index.md"
docs_url: "https://docs.iced.rs/iced_widget/progress_bar/index.html"
source_file: "src/iced_widget/progress_bar.rs"
source_lines: "1-309"
---

# `iced_widget::progress_bar`

Progress bars visualize the progression of an extended computer operation, such as a download, file transfer, or installation.

## Example

```rust
use iced::widget::progress_bar;

struct State {
   progress: f32,
}

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    progress_bar(0.0..=100.0, state.progress)
}
```

## Structs

- [`ProgressBar`](struct.ProgressBar.md) — A bar that displays progress.
- [`Style`](struct.Style.md) — The appearance of a progress bar.

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`ProgressBar`](struct.ProgressBar.md).

## Functions

- [`danger`](fn.danger.md) — The danger style of a [`ProgressBar`](struct.ProgressBar.md).
- [`primary`](fn.primary.md) — The primary style of a [`ProgressBar`](struct.ProgressBar.md).
- [`secondary`](fn.secondary.md) — The secondary style of a [`ProgressBar`](struct.ProgressBar.md).
- [`success`](fn.success.md) — The success style of a [`ProgressBar`](struct.ProgressBar.md).
- [`warning`](fn.warning.md) — The warning style of a [`ProgressBar`](struct.ProgressBar.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`ProgressBar`](struct.ProgressBar.md).
