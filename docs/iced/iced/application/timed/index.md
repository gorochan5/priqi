---
title: "timed"
kind: module
path: "iced::application::timed"
crate: "iced"
module: "iced::application"
file: "iced/application/timed/index.md"
docs_url: "https://docs.iced.rs/iced/application/timed/index.html"
source_file: "src/iced/application/timed.rs"
source_lines: "1-162"
---

# `iced::application::timed`

An [`Application`](../struct.Application.md) that receives an [`Instant`](../../time/struct.Instant.md) in update logic.

## Traits

- [`UpdateFn`](trait.UpdateFn.md) — The update logic of some timed [`Application`](../struct.Application.md).

## Functions

- [`timed`](fn.timed.md) — Creates an [`Application`](../struct.Application.md) with an `update` function that also takes the [`Instant`](../../time/struct.Instant.md) of each `Message`.
