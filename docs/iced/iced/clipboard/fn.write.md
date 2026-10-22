---
title: "write"
kind: function
path: "iced::clipboard::write"
crate: "iced"
module: "iced::clipboard"
file: "iced/clipboard/fn.write.md"
docs_url: "https://docs.iced.rs/iced/clipboard/fn.write.html"
---

# `iced::clipboard::write`

```rust
pub fn write(content: impl Into<Content>) -> Task<Result<(), Error>>
```

Write the given [`Content`](enum.Content.md) to the clipboard.
