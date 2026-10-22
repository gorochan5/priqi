---
title: "read"
kind: function
path: "iced::clipboard::read"
crate: "iced"
module: "iced::clipboard"
file: "iced/clipboard/fn.read.md"
docs_url: "https://docs.iced.rs/iced/clipboard/fn.read.html"
---

# `iced::clipboard::read`

```rust
pub fn read(kind: Kind) -> Task<Result<Arc<Content>, Error>>
```

Read the given [`Kind`](enum.Kind.md) of [`Content`](enum.Content.md) from the clipboard.
