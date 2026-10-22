---
title: "Catalog"
kind: trait
path: "iced_widget::button::Catalog"
crate: "iced_widget"
module: "iced_widget::button"
file: "iced_widget/button/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/button/trait.Catalog.html"
source_file: "src/iced_widget/button.rs"
source_lines: "551-560"
---

# `iced_widget::button::Catalog`

```rust
pub trait Catalog {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}
```

The theme catalog of a [`Button`](struct.Button.md).

All themes that can be used with [`Button`](struct.Button.md) must implement this trait.

## Example

```rust
#[derive(Debug, Default)]
pub enum ButtonClass {
    #[default]
    Primary,
    Secondary,
    Danger
}

impl Catalog for MyTheme {
    type Class<'a> = ButtonClass;
    fn default<'a>() -> Self::Class<'a> {
        ButtonClass::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let mut style = Style::default();

        match class {
            ButtonClass::Primary => {
                style.background = Some(Background::Color(Color::from_rgb(0.529, 0.808, 0.921)));
            },
            ButtonClass::Secondary => {
                style.background = Some(Background::Color(Color::WHITE));
            },
            ButtonClass::Danger => {
                style.background = Some(Background::Color(Color::from_rgb(0.941, 0.502, 0.502)));
            },
        }

        style
    }
}
```

Although, in order to use [`Button::style`](struct.Button.md) with `MyTheme`, [`Catalog::Class`](trait.Catalog.md) must implement `From<StyleFn<'_, MyTheme>>`.

**Source:** [button.rs](../../src/iced_widget/button.rs.md) (lines 551-560)

## Required Associated Types

### `Class`

```rust
type Class<'a>
```

The item class of the [`Catalog`](trait.Catalog.md).

## Required Methods

### `default`

```rust
fn default<'a>() -> Self::Class<'a>
```

The default class produced by the [`Catalog`](trait.Catalog.md).

### `style`

```rust
fn style(&self, class: &Self::Class<'_>, status: Status) -> Style
```

The [`Style`](struct.Style.md) of a class with the given status.

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>`
