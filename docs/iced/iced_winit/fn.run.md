---
title: "run"
kind: function
path: "iced_winit::run"
crate: "iced_winit"
module: "iced_winit"
file: "iced_winit/fn.run.md"
docs_url: "https://docs.iced.rs/iced_winit/fn.run.html"
source_file: "src/iced_winit/lib.rs"
source_lines: "70-435"
---

# `iced_winit::run`

```rust
pub fn run<P>(program: P) -> Result<(), Error> where
    P: Program + 'static,
    P::Theme: Base,
```

Runs a [`Program`] with the provided settings.

**Source:** [lib.rs](../src/iced_winit/lib.rs.md) (lines 70-435)
