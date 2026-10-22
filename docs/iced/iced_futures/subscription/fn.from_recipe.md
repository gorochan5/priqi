---
title: "from_recipe"
kind: function
path: "iced_futures::subscription::from_recipe"
crate: "iced_futures"
module: "iced_futures::subscription"
file: "iced_futures/subscription/fn.from_recipe.md"
docs_url: "https://docs.iced.rs/iced_futures/subscription/fn.from_recipe.html"
source_file: "src/iced_futures/subscription.rs"
source_lines: "426-430"
---

# `iced_futures::subscription::from_recipe`

```rust
pub fn from_recipe<T>(
    recipe: impl Recipe<Output = T> + 'static,
) -> Subscription<T>
```

Creates a [`Subscription`](struct.Subscription.md) from a [`Recipe`](trait.Recipe.md) describing it.

**Source:** [subscription.rs](../../src/iced_futures/subscription.rs.md) (lines 426-430)
