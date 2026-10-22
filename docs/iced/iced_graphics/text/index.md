---
title: "text"
kind: module
path: "iced_graphics::text"
crate: "iced_graphics"
module: "iced_graphics"
file: "iced_graphics/text/index.md"
docs_url: "https://docs.iced.rs/iced_graphics/text/index.html"
source_file: "src/iced_graphics/text.rs"
source_lines: "1-479"
---

# `iced_graphics::text`

Draw text.

## Re-exports

- [`Cache`](cache/struct.Cache.md)
- [`Editor`](editor/struct.Editor.md)
- [`Paragraph`](paragraph/struct.Paragraph.md)
- `pub use cosmic_text;`

## Modules

- [`cache`](cache/index.md) — Cache text.
- [`editor`](editor/index.md) — Draw and edit text.
- [`paragraph`](paragraph/index.md) — Draw paragraphs.

## Structs

- [`FontSystem`](struct.FontSystem.md) — A set of system fonts.
- [`Raw`](struct.Raw.md) — A weak reference to a [`cosmic_text::Buffer`] that can be drawn.
- [`Version`](struct.Version.md) — A version number.

## Enums

- [`Text`](enum.Text.md) — A text primitive.

## Constants

- [`FIRA_SANS_REGULAR`](constant.FIRA_SANS_REGULAR.md) *(fira-sans)* — The regular variant of the [Fira Sans](https://mozilla.github.io/Fira/) font.

## Traits

- [`Renderer`](trait.Renderer.md) — A text renderer coupled to `iced_graphics`.

## Functions

- [`align`](fn.align.md) — Aligns the given [`cosmic_text::Buffer`] with the given [`Alignment`] and returns its minimum [`Size`].
- [`font_system`](fn.font_system.md) — Returns the global [`FontSystem`](struct.FontSystem.md).
- [`hint_factor`](fn.hint_factor.md) — Returns the ideal hint factor given the size and scale factor of some text.
- [`measure`](fn.measure.md) — Measures the dimensions of the given [`cosmic_text::Buffer`].
- [`regions`](fn.regions.md) — Computes selection regions in the given buffer.
- [`to_attributes`](fn.to_attributes.md) — Returns the attributes of the given [`Font`].
- [`to_color`](fn.to_color.md) — Converts some [`Color`] to a [`cosmic_text::Color`].
- [`to_ellipsize`](fn.to_ellipsize.md) — Converts some [`Ellipsis`] strategy to a [`cosmic_text::Ellipsize`] strategy.
- [`to_shaping`](fn.to_shaping.md) — Converts some [`Shaping`] strategy to a [`cosmic_text::Shaping`] strategy.
- [`to_wrap`](fn.to_wrap.md) — Converts some [`Wrapping`] strategy to a [`cosmic_text::Wrap`] strategy.
