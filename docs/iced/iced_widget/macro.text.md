---
title: "text"
kind: macro
path: "iced_widget::text"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/macro.text.md"
docs_url: "https://docs.iced.rs/iced_widget/macro.text.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "173-177"
---

# `iced_widget::text`

```rust
macro_rules! text {
    ($($arg:tt)*) => { ... };
}
```

Creates a new `Text` widget with the provided content.

This macro uses the same syntax as [`format!`](https://doc.rust-lang.org/nightly/alloc/macro.format.html), but creates a new `Text` widget instead.

See [the formatting documentation in `std::fmt`](https://doc.rust-lang.org/nightly/alloc/fmt/index.html) for details of the macro argument syntax.

## Examples

```rust
use iced::widget::text;

enum Message {
    // ...
}

fn view(_state: &State) -> impl Widget<Message> {
    let simple = text!("Hello, world!");

    let keyword = text!("Hello, {}", "world!");

    let planet = "Earth";
    let local_variable = text!("Hello, {planet}!");
    // ...
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 173-177)
