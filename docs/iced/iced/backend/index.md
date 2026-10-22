---
title: "backend"
kind: module
path: "iced::backend"
crate: "iced"
module: "iced"
file: "iced/backend/index.md"
docs_url: "https://docs.iced.rs/iced/backend/index.html"
source_file: "src/iced/lib.rs"
source_lines: "653"
---

# `iced::backend`

Graphical backends are designed to aid in rendering computer graphics to a monitor.

## Structs

- [`Settings`](struct.Settings.md) — The settings usted to configure a [`Backend`](../enum.Backend.md).

## Enums

- [`Action`](enum.Action.md) — An backend operation.
- [`Api`](enum.Api.md) — A hardware graphics API.
- [`Backend`](enum.Backend.md) — A graphical backend.
- [`Error`](enum.Error.md) — An error that occurred while creating an application’s graphical context.
- [`PowerPreference`](enum.PowerPreference.md) — The power-usage preference for graphics adapters.
- [`Reason`](enum.Reason.md) — The reason why a graphics adapter could not be found

## Functions

- [`configure`](fn.configure.md) — Returns a [`Task`](../struct.Task.md) that switches the [`backend::Settings`](struct.Settings.md) of the current application.
