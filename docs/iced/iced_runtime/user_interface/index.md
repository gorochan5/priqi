---
title: "user_interface"
kind: module
path: "iced_runtime::user_interface"
crate: "iced_runtime"
module: "iced_runtime"
file: "iced_runtime/user_interface/index.md"
docs_url: "https://docs.iced.rs/iced_runtime/user_interface/index.html"
source_file: "src/iced_runtime/user_interface.rs"
source_lines: "1-714"
---

# `iced_runtime::user_interface`

Implement your own event loop to drive a user interface.

## Structs

- [`Cache`](struct.Cache.md) — Reusable data of a specific [`UserInterface`](struct.UserInterface.md).
- [`UserInterface`](struct.UserInterface.md) — A set of interactive graphical elements with a specific [`Layout`].

## Enums

- [`Change`](enum.Change.md) — The change experienced by a [`UserInterface`](struct.UserInterface.md) while being updated.
- [`State`](enum.State.md) — The resulting state after updating a [`UserInterface`](struct.UserInterface.md).
