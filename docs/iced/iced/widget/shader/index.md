---
title: "shader"
kind: module
path: "iced::widget::shader"
crate: "iced"
module: "iced::widget"
file: "iced/widget/shader/index.md"
docs_url: "https://docs.iced.rs/iced/widget/shader/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "121"
---

# `iced::widget::shader`

A custom shader widget for wgpu applications.

## Structs

- [`Action`](struct.Action.md) — A runtime action that can be performed by some widgets.
- [`Shader`](struct.Shader.md) — A widget which can render custom shaders with Iced’s `wgpu` backend.
- [`Storage`](struct.Storage.md) — Stores custom, user-provided types.
- [`Viewport`](struct.Viewport.md) — A viewing region for displaying computer graphics.

## Traits

- [`Pipeline`](trait.Pipeline.md) — The pipeline of a graphics [`Primitive`](trait.Primitive.md).
- [`Primitive`](trait.Primitive.md) — A set of methods which allows a [`Primitive`](trait.Primitive.md) to be rendered.
- [`Program`](trait.Program.md) — The state and logic of a [`Shader`](../struct.Shader.md) widget.
