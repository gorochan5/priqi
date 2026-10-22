---
title: "commands"
kind: module
path: "iced::widget::canvas::path::lyon_path::commands"
crate: "iced"
module: "iced::widget::canvas::path::lyon_path"
file: "iced/widget/canvas/path/lyon_path/commands/index.md"
docs_url: "https://docs.iced.rs/iced/widget/canvas/path/lyon_path/commands/index.html"
---

# `iced::widget::canvas::path::lyon_path::commands`

A generic representation for paths that allow more control over how endpoints and control points are stored.

## Motivation

The default `Path` data structure in this crate is works well for the most common use cases. Sometimes, however, it is useful to be able to specify exactly how endpoints and control points are stored instead of relying on implicitly following the order of the events.

This module contains bricks to help with building custom path representations. The central piece is the [`PathCommands`](struct.PathCommands.md) buffer and its [`PathCommandsBuilder`](struct.PathCommandsBuilder.md), providing a compact representation for path events with IDs instead of positions.

## Examples

The following example shows how `PathCommands` can be used together with an external buffers for positions to implement features similar to the default Path type with a different data structure.

```rust
use lyon_path::{EndpointId, Event, IdEvent, commands::PathCommands};
let points = &[
    [0.0, 0.0],
    [1.0, 1.0],
    [0.0, 2.0],
];

let mut cmds = PathCommands::builder();
cmds.begin(EndpointId(0));
cmds.line_to(EndpointId(1));
cmds.line_to(EndpointId(2));
cmds.end(true);

let cmds = cmds.build();

for event in &cmds {
    match event {
        IdEvent::Begin { at } => { println!("move to {:?}", points[at.to_usize()]); }
        IdEvent::Line { to, .. } => { println!("line to {:?}", points[to.to_usize()]); }
        IdEvent::End { close: true, .. } => { println!("close"); }
        _ => { panic!("unexpected event!") }
    }
}

// Iterate over the points directly using CommandsPathSlice
for event in cmds.path_slice(points, points).events() {
    match event {
        Event::Begin { at } => { println!("move to {:?}", at); }
        Event::Line { to, .. } => { println!("line to {:?}", to); }
        Event::End { close: true, .. } => { println!("close"); }
        _ => { panic!("unexpected event!") }
    }
}
```

## Structs

- [`CommandsPathSlice`](struct.CommandsPathSlice.md) — A view on a [`PathCommands`](struct.PathCommands.md) buffer and two slices for endpoints and control points, providing similar functionalities as `PathSlice`.
- [`Events`](struct.Events.md) — An iterator of `Event<&Endpoint, &ControlPoint>`.
- [`Iter`](struct.Iter.md) — An iterator of `Event<&Endpoint, &ControlPoint>`.
- [`PathCommands`](struct.PathCommands.md) — The commands of a path encoded in a single array using IDs to refer to endpoints and control points externally.
- [`PathCommandsBuilder`](struct.PathCommandsBuilder.md) — Builds path commands.
- [`PathCommandsSlice`](struct.PathCommandsSlice.md) — A view over [`PathCommands`](struct.PathCommands.md).
- [`PointEvents`](struct.PointEvents.md) — An iterator of `PathEvent`.
