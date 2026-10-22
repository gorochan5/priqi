---
title: "simulator"
kind: function
path: "iced_test::simulator::simulator"
crate: "iced_test"
module: "iced_test::simulator"
file: "iced_test/simulator/fn.simulator.md"
docs_url: "https://docs.iced.rs/iced_test/simulator/fn.simulator.html"
source_file: "src/iced_test/simulator.rs"
source_lines: "391-400"
---

# `iced_test::simulator::simulator`

```rust
pub fn simulator<'a, Message, Theme, Renderer>(
    element: impl Widget<Message, Theme, Renderer> + 'a,
) -> Simulator<'a, Message, Theme, Renderer> where
    Message: 'a,
    Theme: Base + 'a,
    Renderer: Renderer + Headless + 'a,
```

Creates a new [`Simulator`](struct.Simulator.md).

This is just a function version of [`Simulator::new`](struct.Simulator.md).

**Source:** [simulator.rs](../../src/iced_test/simulator.rs.md) (lines 391-400)
