---
title: "tap_key"
kind: function
path: "iced_test::simulator::tap_key"
crate: "iced_test"
module: "iced_test::simulator"
file: "iced_test/simulator/fn.tap_key.md"
docs_url: "https://docs.iced.rs/iced_test/simulator/fn.tap_key.html"
source_file: "src/iced_test/simulator.rs"
source_lines: "451-458"
---

# `iced_test::simulator::tap_key`

```rust
pub fn tap_key(
    key: impl Into<Key>,
    text: Option<SmolStr>,
) -> impl Iterator<Item = Event>
```

Returns the sequence of events of a “key tap” (i.e. pressing and releasing a key).

**Source:** [simulator.rs](../../src/iced_test/simulator.rs.md) (lines 451-458)
