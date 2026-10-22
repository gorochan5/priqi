---
title: "icon"
kind: module
path: "iced::window::icon"
crate: "iced"
module: "iced::window"
file: "iced/window/icon/index.md"
docs_url: "https://docs.iced.rs/iced/window/icon/index.html"
source_file: "src/iced/window/icon.rs"
source_lines: "1-64"
---

# `iced::window::icon`

Attach an icon to the window of your application.

## Structs

- [`Icon`](struct.Icon.md) — An window icon normally used for the titlebar or taskbar.

## Enums

- [`Error`](enum.Error.md) — An error produced when creating an [`Icon`](../struct.Icon.md).

## Functions

- [`from_file`](fn.from_file.md) *(image)* — Creates an icon from an image file.
- [`from_file_data`](fn.from_file_data.md) *(image)* — Creates an icon from the content of an image file.
- [`from_rgba`](fn.from_rgba.md) — Builds an [`Icon`](../struct.Icon.md) from its RGBA pixels in the `sRGB` color space.
