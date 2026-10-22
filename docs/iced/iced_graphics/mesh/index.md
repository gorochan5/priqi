---
title: "mesh"
kind: module
path: "iced_graphics::mesh"
crate: "iced_graphics"
module: "iced_graphics"
file: "iced_graphics/mesh/index.md"
docs_url: "https://docs.iced.rs/iced_graphics/mesh/index.html"
source_file: "src/iced_graphics/mesh.rs"
source_lines: "1-206"
---

# `iced_graphics::mesh`

Draw triangles!

## Structs

- [`AttributeCount`](struct.AttributeCount.md) — The result of counting the attributes of a set of meshes.
- [`Cache`](struct.Cache.md) — A cache of multiple meshes.
- [`GradientVertex2D`](struct.GradientVertex2D.md) — A vertex which contains 2D position & packed gradient data.
- [`Id`](struct.Id.md) — The unique id of a [`Cache`](struct.Cache.md).
- [`Indexed`](struct.Indexed.md) — A set of vertices and indices representing a list of triangles.
- [`SolidVertex2D`](struct.SolidVertex2D.md) — A two-dimensional vertex with a color.

## Enums

- [`Mesh`](enum.Mesh.md) — A low-level primitive to render a mesh of triangles.

## Traits

- [`Renderer`](trait.Renderer.md) — A renderer capable of drawing a [`Mesh`](enum.Mesh.md).

## Functions

- [`attribute_count_of`](fn.attribute_count_of.md) — Returns the number of total vertices & total indices of all [`Mesh`](enum.Mesh.md)es.
