---
title: "pane_grid"
kind: function
path: "iced_widget::pane_grid"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.pane_grid.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.pane_grid.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1709-1722"
---

# `iced_widget::pane_grid`

```rust
pub fn pane_grid<'a, T, Message, Title, W, Theme, Renderer>(
    state: &'a State<T>,
    view: impl Fn(Pane, &'a T, bool) -> Content<'a, Message, Title, W, Theme, Renderer>,
) -> PaneGrid<'a, Message, Title, W, Theme, Renderer> where
    Theme: Catalog,
    Renderer: Renderer,
```

Creates a [`PaneGrid`](pane_grid/struct.PaneGrid.md) with the given [`pane_grid::State`](pane_grid/state/struct.State.md) and view function.

Pane grids let your users split regions of your application and organize layout dynamically.

## Example

```rust
use iced::widget::{pane_grid, text};

struct State {
    panes: pane_grid::State<Pane>,
}

enum Pane {
    SomePane,
    AnotherKindOfPane,
}

enum Message {
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
}

fn view(state: &State) -> impl Widget<Message> {
    pane_grid(&state.panes, |pane, state, is_maximized| {
        pane_grid::Content::new(match state {
            Pane::SomePane => text("This is some pane"),
            Pane::AnotherKindOfPane => text("This is another kind of pane"),
        })
    })
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1709-1722)
