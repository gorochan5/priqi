---
title: "system"
kind: module
path: "iced_runtime::system"
crate: "iced_runtime"
module: "iced_runtime"
file: "iced_runtime/system/index.md"
docs_url: "https://docs.iced.rs/iced_runtime/system/index.html"
source_file: "src/iced_runtime/system.rs"
source_lines: "1-72"
---

# `iced_runtime::system`

Access the native system.

## Structs

- [`Information`](struct.Information.md) — Contains information about the system (e.g. system name, processor, memory, graphics adapter).

## Enums

- [`Action`](enum.Action.md) — An operation to be performed on the system.

## Functions

- [`information`](fn.information.md) — Returns available system information.
- [`theme`](fn.theme.md) — Returns the current system theme.
- [`theme_changes`](fn.theme_changes.md) — Subscribes to system theme changes.
