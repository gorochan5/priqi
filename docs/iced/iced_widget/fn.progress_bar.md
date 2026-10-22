---
title: "progress_bar"
kind: function
path: "iced_widget::progress_bar"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.progress_bar.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.progress_bar.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1437-1442"
---

# `iced_widget::progress_bar`

```rust
pub fn progress_bar<'a, Theme>(
    range: RangeInclusive<f32>,
    value: f32,
) -> ProgressBar<'a, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`ProgressBar`](progress_bar/struct.ProgressBar.md).

Progress bars visualize the progression of an extended computer operation, such as a download, file transfer, or installation.

It expects:

- an inclusive range of possible values, and
- the current value of the [`ProgressBar`](progress_bar/struct.ProgressBar.md).

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

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1437-1442)
