---
title: "backend"
kind: module
path: "iced_core::backend"
crate: "iced_core"
module: "iced_core"
file: "iced_core/backend/index.md"
docs_url: "https://docs.iced.rs/iced_core/backend/index.html"
source_file: "src/iced_core/backend.rs"
source_lines: "1-254"
---

# `iced_core::backend`

Graphical backends are designed to aid in rendering computer graphics to a monitor.

## Structs

- [`Settings`](struct.Settings.md) — The settings usted to configure a [`Backend`](enum.Backend.md).

## Enums

- [`Api`](enum.Api.md) — A hardware graphics API.
- [`Backend`](enum.Backend.md) — A graphical backend.
- [`Error`](enum.Error.md) — An error that occurred while creating an application’s graphical context.
- [`PowerPreference`](enum.PowerPreference.md) — The power-usage preference for graphics adapters.
- [`Reason`](enum.Reason.md) — The reason why a graphics adapter could not be found
