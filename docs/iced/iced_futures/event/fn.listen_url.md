---
title: "listen_url"
kind: function
path: "iced_futures::event::listen_url"
crate: "iced_futures"
module: "iced_futures::event"
file: "iced_futures/event/fn.listen_url.md"
docs_url: "https://docs.iced.rs/iced_futures/event/fn.listen_url.html"
source_file: "src/iced_futures/event.rs"
source_lines: "82-92"
---

# `iced_futures::event::listen_url`

```rust
pub fn listen_url() -> Subscription<String>
```

Creates a [`Subscription`](../subscription/struct.Subscription.md) that notifies of custom application URL received from the system.

***Note:** Currently, it only triggers on macOS and the executable needs to be properly [bundled](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html#//apple_ref/doc/uid/10000123i-CH101-SW19)!*

**Source:** [event.rs](../../src/iced_futures/event.rs.md) (lines 82-92)
