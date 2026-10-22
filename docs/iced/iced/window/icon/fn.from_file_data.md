---
title: "from_file_data"
kind: function
path: "iced::window::icon::from_file_data"
crate: "iced"
module: "iced::window::icon"
file: "iced/window/icon/fn.from_file_data.md"
docs_url: "https://docs.iced.rs/iced/window/icon/fn.from_file_data.html"
source_file: "src/iced/window/icon.rs"
source_lines: "26-47"
---

# `iced::window::icon::from_file_data`

```rust
pub fn from_file_data(
    data: &[u8],
    explicit_format: Option<ImageFormat>,
) -> Result<Icon, Error>
```

Creates an icon from the content of an image file.

This content can be included in your application at compile-time, e.g. using the `include_bytes!` macro. You can pass an explicit file format. Otherwise, the file format will be guessed at runtime.

**Source:** [icon.rs](../../../src/iced/window/icon.rs.md) (lines 26-47)
