---
title: "pane_grid"
kind: module
path: "iced::widget::pane_grid"
crate: "iced"
module: "iced::widget"
file: "iced/widget/pane_grid/index.md"
docs_url: "https://docs.iced.rs/iced/widget/pane_grid/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "31"
---

# `iced::widget::pane_grid`

Pane grids let your users split regions of your application and organize layout dynamically.

![Pane grid - Iced](https://iced.rs/examples/pane_grid.gif)

This distribution of space is common in tiling window managers (like [`awesome`](https://awesomewm.org/), [`i3`](https://i3wm.org/), or even [`tmux`](https://github.com/tmux/tmux)).

A [`PaneGrid`](../struct.PaneGrid.md) supports:

- Vertical and horizontal splits
- Tracking of the last active pane
- Mouse-based resizing
- Drag and drop to reorganize panes
- Hotkey support
- Configurable modifier keys
- [`State`](struct.State.md) API to perform actions programmatically (`split`, `swap`, `resize`, etc.)

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

The [`pane_grid` example](https://github.com/iced-rs/iced/tree/master/examples/pane_grid) showcases how to use a [`PaneGrid`](../struct.PaneGrid.md) with resizing, drag and drop, and hotkey support.

## Modules

- [`state`](state/index.md) — The state of a [`PaneGrid`](../struct.PaneGrid.md).

## Structs

- [`Content`](struct.Content.md) — The content of a [`Pane`](struct.Pane.md).
- [`Controls`](struct.Controls.md) — The controls of a [`Pane`](struct.Pane.md).
- [`Highlight`](struct.Highlight.md) — The appearance of a highlight of the [`PaneGrid`](../struct.PaneGrid.md).
- [`Line`](struct.Line.md) — A line.
- [`Pane`](struct.Pane.md) — A rectangular region in a [`PaneGrid`](../struct.PaneGrid.md) used to display widgets.
- [`PaneGrid`](struct.PaneGrid.md) — A collection of panes distributed using either vertical or horizontal splits to completely fill the space available.
- [`ResizeEvent`](struct.ResizeEvent.md) — An event produced during a resize interaction of a [`PaneGrid`](../struct.PaneGrid.md).
- [`Split`](struct.Split.md) — A divider that splits a region in a [`PaneGrid`](../struct.PaneGrid.md) into two different panes.
- [`State`](struct.State.md) — The state of a [`PaneGrid`](../struct.PaneGrid.md).
- [`Style`](struct.Style.md) — The appearance of a [`PaneGrid`](../struct.PaneGrid.md).
- [`TitleBar`](struct.TitleBar.md) — The title bar of a [`Pane`](struct.Pane.md).

## Enums

- [`Axis`](enum.Axis.md) — A fixed reference line for the measurement of coordinates.
- [`Configuration`](enum.Configuration.md) — The arrangement of a [`PaneGrid`](../struct.PaneGrid.md).
- [`Direction`](enum.Direction.md) — A four cardinal direction.
- [`DragEvent`](enum.DragEvent.md) — An event produced during a drag and drop interaction of a [`PaneGrid`](../struct.PaneGrid.md).
- [`Edge`](enum.Edge.md) — The edges of an area.
- [`Node`](enum.Node.md) — A layout node of a [`PaneGrid`](../struct.PaneGrid.md).
- [`Region`](enum.Region.md) — The region of a [`Pane`](struct.Pane.md).
- [`Target`](enum.Target.md) — The [`Target`](enum.Target.md) area a pane can be dropped on.

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`PaneGrid`](../struct.PaneGrid.md).
- [`Draggable`](trait.Draggable.md) — A pane that can be dragged.

## Functions

- [`default`](fn.default.md) — The default style of a [`PaneGrid`](../struct.PaneGrid.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`PaneGrid`](../struct.PaneGrid.md).
