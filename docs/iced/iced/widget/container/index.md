---
title: "container"
kind: module
path: "iced::widget::container"
crate: "iced"
module: "iced::widget"
file: "iced/widget/container/index.md"
docs_url: "https://docs.iced.rs/iced/widget/container/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "25"
---

# `iced::widget::container`

Containers let you align a widget inside their boundaries.

## Example

```rust
use iced::widget::container;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    container("This text is centered inside a rounded box!")
        .padding(10)
        .center(800)
        .style(container::rounded_box)
}
```

## Structs

- [`Container`](struct.Container.md) — A widget that aligns its contents inside of its boundaries.
- [`Style`](struct.Style.md) — The appearance of a container.

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Container`](../struct.Container.md).

## Functions

- [`background`](fn.background.md) — A [`Container`](../struct.Container.md) with the given [`Background`](../../enum.Background.md).
- [`bordered_box`](fn.bordered_box.md) — A bordered [`Container`](../struct.Container.md) with a background.
- [`danger`](fn.danger.md) — A [`Container`](../struct.Container.md) with a danger background color.
- [`dark`](fn.dark.md) — A [`Container`](../struct.Container.md) with a dark background and white text.
- [`draw_background`](fn.draw_background.md) — Draws the background of a [`Container`](../struct.Container.md) given its [`Style`](struct.Style.md) and its `bounds`.
- [`layout`](fn.layout.md) — Computes the layout of a [`Container`](../struct.Container.md).
- [`primary`](fn.primary.md) — A [`Container`](../struct.Container.md) with a primary background color.
- [`rounded_box`](fn.rounded_box.md) — A rounded [`Container`](../struct.Container.md) with a background.
- [`secondary`](fn.secondary.md) — A [`Container`](../struct.Container.md) with a secondary background color.
- [`success`](fn.success.md) — A [`Container`](../struct.Container.md) with a success background color.
- [`transparent`](fn.transparent.md) — A transparent [`Container`](../struct.Container.md).
- [`warning`](fn.warning.md) — A [`Container`](../struct.Container.md) with a warning background color.

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Container`](../struct.Container.md).
