---
title: "iced_runtime"
kind: crate
path: "iced_runtime"
crate: "iced_runtime"
file: "iced_runtime/index.md"
docs_url: "https://docs.iced.rs/iced_runtime/index.html"
source_file: "src/iced_runtime/lib.rs"
source_lines: "1-141"
---

# `iced_runtime`

A renderer-agnostic native GUI runtime.

![The native path of the Iced ecosystem](https://github.com/iced-rs/iced/blob/master/docs/graphs/native.png?raw=true)

`iced_runtime` takes [`iced_core`](https://github.com/iced-rs/iced/tree/master/core) and builds a native runtime on top of it.

## Re-exports

- [`Task`](task/struct.Task.md)
- [`UserInterface`](user_interface/struct.UserInterface.md)
- `pub use iced_core as core;`
- `pub use iced_futures as futures;`

## Modules

- [`backend`](backend/index.md) — Configure a `Backend` at runtime.
- [`clipboard`](clipboard/index.md) — Access the clipboard.
- [`font`](font/index.md) — Load and use fonts.
- [`image`](image/index.md) — Allocate images explicitly to control presentation.
- [`keyboard`](keyboard/index.md) — Track keyboard events.
- [`system`](system/index.md) — Access the native system.
- [`task`](task/index.md) — Create runtime tasks.
- [`user_interface`](user_interface/index.md) — Implement your own event loop to drive a user interface.
- [`widget`](widget/index.md) — Operate on widgets and query them at runtime.
- [`window`](window/index.md) — Build window-based GUI applications.

## Enums

- [`Action`](enum.Action.md) — An action that the iced runtime can perform.

## Functions

- [`exit`](fn.exit.md) — Creates a [`Task`](task/struct.Task.md) that exits the iced runtime.
