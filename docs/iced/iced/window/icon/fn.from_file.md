---
title: "from_file"
kind: function
path: "iced::window::icon::from_file"
crate: "iced"
module: "iced::window::icon"
file: "iced/window/icon/fn.from_file.md"
docs_url: "https://docs.iced.rs/iced/window/icon/fn.from_file.html"
source_file: "src/iced/window/icon.rs"
source_lines: "15-19"
---

# `iced::window::icon::from_file`

```rust
pub fn from_file<P: AsRef<Path>>(icon_path: P) -> Result<Icon, Error>
```

Creates an icon from an image file.

This will return an error in case the file is missing at run-time. You may prefer [`from_file_data`](fn.from_file_data.md) instead.

**Source:** [icon.rs](../../../src/iced/window/icon.rs.md) (lines 15-19)
