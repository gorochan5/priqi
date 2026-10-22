---
title: "never"
kind: function
path: "iced_core::never"
crate: "iced_core"
module: "iced_core"
file: "iced_core/fn.never.md"
docs_url: "https://docs.iced.rs/iced_core/fn.never.html"
source_file: "src/iced_core/lib.rs"
source_lines: "98-100"
---

# `iced_core::never`

```rust
pub fn never<T>(never: Never) -> T
```

A function that can *never* be called.

This is useful to turn generic types into anything you want by coercing them into a type with no possible values.

**Source:** [lib.rs](../src/iced_core/lib.rs.md) (lines 98-100)
