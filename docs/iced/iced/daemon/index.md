---
title: "daemon"
kind: module
path: "iced::daemon"
crate: "iced"
module: "iced"
file: "iced/daemon/index.md"
docs_url: "https://docs.iced.rs/iced/daemon/index.html"
source_file: "src/iced/daemon.rs"
source_lines: "1-667"
---

# `iced::daemon`

Create and run daemons that run in the background.

## Structs

- [`Daemon`](struct.Daemon.md) — The underlying definition and configuration of an iced daemon.

## Traits

- [`ThemeFn`](trait.ThemeFn.md) — The theme logic of some [`Daemon`](struct.Daemon.md).
- [`TitleFn`](trait.TitleFn.md) — The title logic of some [`Daemon`](struct.Daemon.md).
- [`ViewFn`](trait.ViewFn.md) — The view logic of some [`Daemon`](struct.Daemon.md).

## Functions

- [`daemon`](fn.daemon.md) — Creates an iced [`Daemon`](struct.Daemon.md) given its boot, update, and view logic.
