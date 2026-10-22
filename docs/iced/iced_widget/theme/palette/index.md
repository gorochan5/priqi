---
title: "palette"
kind: module
path: "iced_widget::theme::palette"
crate: "iced_widget"
module: "iced_widget::theme"
file: "iced_widget/theme/palette/index.md"
docs_url: "https://docs.iced.rs/iced_widget/theme/palette/index.html"
---

# `iced_widget::theme::palette`

Define the colors of a theme.

## Structs

- [`Background`](struct.Background.md) — A set of background colors.
- [`Pair`](struct.Pair.md) — A pair of background and text colors.
- [`Palette`](struct.Palette.md) — An extended set of colors generated from a [`Seed`](struct.Seed.md).
- [`Seed`](struct.Seed.md) — The base set of colors of a [`Palette`](../struct.Palette.md).
- [`Swatch`](struct.Swatch.md) — A color sample in a palette of colors.

## Statics

- [`CATPPUCCIN_FRAPPE`](static.CATPPUCCIN_FRAPPE.md) — The built-in Catppuccin Frappé variant of a [`Palette`](../struct.Palette.md).
- [`CATPPUCCIN_LATTE`](static.CATPPUCCIN_LATTE.md) — The built-in Catppuccin Latte variant of a [`Palette`](../struct.Palette.md).
- [`CATPPUCCIN_MACCHIATO`](static.CATPPUCCIN_MACCHIATO.md) — The built-in Catppuccin Macchiato variant of a [`Palette`](../struct.Palette.md).
- [`CATPPUCCIN_MOCHA`](static.CATPPUCCIN_MOCHA.md) — The built-in Catppuccin Mocha variant of a [`Palette`](../struct.Palette.md).
- [`DARK`](static.DARK.md) — The built-in dark variant of a [`Palette`](../struct.Palette.md).
- [`DRACULA`](static.DRACULA.md) — The built-in Dracula variant of a [`Palette`](../struct.Palette.md).
- [`FERRA`](static.FERRA.md) — The built-in Ferra variant of a [`Palette`](../struct.Palette.md).
- [`GRUVBOX_DARK`](static.GRUVBOX_DARK.md) — The built-in Gruvbox Dark variant of a [`Palette`](../struct.Palette.md).
- [`GRUVBOX_LIGHT`](static.GRUVBOX_LIGHT.md) — The built-in Gruvbox Light variant of a [`Palette`](../struct.Palette.md).
- [`KANAGAWA_DRAGON`](static.KANAGAWA_DRAGON.md) — The built-in Kanagawa Dragon variant of a [`Palette`](../struct.Palette.md).
- [`KANAGAWA_LOTUS`](static.KANAGAWA_LOTUS.md) — The built-in Kanagawa Lotus variant of a [`Palette`](../struct.Palette.md).
- [`KANAGAWA_WAVE`](static.KANAGAWA_WAVE.md) — The built-in Kanagawa Wave variant of a [`Palette`](../struct.Palette.md).
- [`LIGHT`](static.LIGHT.md) — The built-in light variant of a [`Palette`](../struct.Palette.md).
- [`MOONFLY`](static.MOONFLY.md) — The built-in Moonfly variant of a [`Palette`](../struct.Palette.md).
- [`NIGHTFLY`](static.NIGHTFLY.md) — The built-in Nightfly variant of a [`Palette`](../struct.Palette.md).
- [`NORD`](static.NORD.md) — The built-in Nord variant of a [`Palette`](../struct.Palette.md).
- [`OXOCARBON`](static.OXOCARBON.md) — The built-in Oxocarbon variant of a [`Palette`](../struct.Palette.md).
- [`SOLARIZED_DARK`](static.SOLARIZED_DARK.md) — The built-in Solarized Dark variant of a [`Palette`](../struct.Palette.md).
- [`SOLARIZED_LIGHT`](static.SOLARIZED_LIGHT.md) — The built-in Solarized Light variant of a [`Palette`](../struct.Palette.md).
- [`TOKYO_NIGHT`](static.TOKYO_NIGHT.md) — The built-in Tokyo Night variant of a [`Palette`](../struct.Palette.md).
- [`TOKYO_NIGHT_LIGHT`](static.TOKYO_NIGHT_LIGHT.md) — The built-in Tokyo Night variant of a [`Palette`](../struct.Palette.md).
- [`TOKYO_NIGHT_STORM`](static.TOKYO_NIGHT_STORM.md) — The built-in Tokyo Night Storm variant of a [`Palette`](../struct.Palette.md).

## Functions

- [`darken`](fn.darken.md) — Darkens a [`Color`] by the given factor.
- [`deviate`](fn.deviate.md) — Deviates a [`Color`] by the given factor. Lightens if the [`Color`] is dark, darkens otherwise.
- [`is_dark`](fn.is_dark.md) — Returns true if the [`Color`] is dark.
- [`lighten`](fn.lighten.md) — Lightens a [`Color`] by the given factor.
- [`readable`](fn.readable.md) — Computes a [`Color`] from the given text color that is readable on top of the given background color.
