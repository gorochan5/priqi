---
title: "shader"
kind: module
path: "iced_widget::shader"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/shader/index.md"
docs_url: "https://docs.iced.rs/iced_widget/shader/index.html"
source_file: "src/iced_widget/shader.rs"
source_lines: "1-177"
---

# `iced_widget::shader`

A custom shader widget for wgpu applications.

## Re-exports

- [`Action`](../struct.Action.md)

## Structs

- [`Shader`](struct.Shader.md) — A widget which can render custom shaders with Iced’s `wgpu` backend.
- [`Storage`](struct.Storage.md) — Stores custom, user-provided types.
- [`Viewport`](struct.Viewport.md) — A viewing region for displaying computer graphics.

## Traits

- [`Pipeline`](trait.Pipeline.md) — The pipeline of a graphics [`Primitive`](trait.Primitive.md).
- [`Primitive`](trait.Primitive.md) — A set of methods which allows a [`Primitive`](trait.Primitive.md) to be rendered.
- [`Program`](trait.Program.md) — The state and logic of a [`Shader`](struct.Shader.md) widget.
