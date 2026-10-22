---
title: "into_recipes"
kind: function
path: "iced_futures::subscription::into_recipes"
crate: "iced_futures"
module: "iced_futures::subscription"
file: "iced_futures/subscription/fn.into_recipes.md"
docs_url: "https://docs.iced.rs/iced_futures/subscription/fn.into_recipes.html"
source_file: "src/iced_futures/subscription.rs"
source_lines: "433-435"
---

# `iced_futures::subscription::into_recipes`

```rust
pub fn into_recipes<T>(
    subscription: Subscription<T>,
) -> Vec<Box<dyn Recipe<Output = T>>>
```

Returns the different recipes of the [`Subscription`](struct.Subscription.md).

**Source:** [subscription.rs](../../src/iced_futures/subscription.rs.md) (lines 433-435)
