---
title: "request_user_attention"
kind: function
path: "iced::window::request_user_attention"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.request_user_attention.md"
docs_url: "https://docs.iced.rs/iced/window/fn.request_user_attention.html"
---

# `iced::window::request_user_attention`

```rust
pub fn request_user_attention<T>(
    id: Id,
    user_attention: Option<UserAttention>,
) -> Task<T>
```

Requests user attention to the window. This has no effect if the application is already focused. How requesting for user attention manifests is platform dependent, see [`UserAttention`](enum.UserAttention.md) for details.

Providing `None` will unset the request for user attention. Unsetting the request for user attention might not be done automatically by the WM when the window receives input.
