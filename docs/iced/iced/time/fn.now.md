---
title: "now"
kind: function
path: "iced::time::now"
crate: "iced"
module: "iced::time"
file: "iced/time/fn.now.md"
docs_url: "https://docs.iced.rs/iced/time/fn.now.html"
source_file: "src/iced/time.rs"
source_lines: "21-23"
---

# `iced::time::now`

```rust
pub fn now() -> Task<Instant>
```

Returns a [`Task`](../struct.Task.md) that produces the current [`Instant`](struct.Instant.md) by calling [`Instant::now`](struct.Instant.md).

While you can call [`Instant::now`](struct.Instant.md) directly in your application; that renders your application “impure” (i.e. no referential transparency).

You may care about purity if you want to leverage the `time-travel` feature properly.

**Source:** [time.rs](../../src/iced/time.rs.md) (lines 21-23)
