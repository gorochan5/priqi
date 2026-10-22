---
title: "allow_automatic_tabbing"
kind: function
path: "iced_runtime::window::allow_automatic_tabbing"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.allow_automatic_tabbing.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.allow_automatic_tabbing.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "475-479"
---

# `iced_runtime::window::allow_automatic_tabbing`

```rust
pub fn allow_automatic_tabbing<T>(enabled: bool) -> Task<T>
```

Sets whether the system can automatically organize windows into tabs.

See [https://developer.apple.com/documentation/appkit/nswindow/1646657-allowsautomaticwindowtabbing](https://developer.apple.com/documentation/appkit/nswindow/1646657-allowsautomaticwindowtabbing)

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 475-479)
