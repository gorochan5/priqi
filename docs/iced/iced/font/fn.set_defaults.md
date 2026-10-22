---
title: "set_defaults"
kind: function
path: "iced::font::set_defaults"
crate: "iced"
module: "iced::font"
file: "iced/font/fn.set_defaults.md"
docs_url: "https://docs.iced.rs/iced/font/fn.set_defaults.html"
---

# `iced::font::set_defaults`

```rust
pub fn set_defaults<Message>(
    font: Font,
    text_size: impl Into<Pixels>,
) -> Task<Message>
```

Sets a new default [`Font`](../struct.Font.md) and text size for the running application.
