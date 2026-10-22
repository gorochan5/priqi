---
title: "FusedStream"
kind: trait
path: "futures_core::stream::FusedStream"
crate: "futures_core"
module: "futures_core::stream"
file: "futures_core/stream/trait.FusedStream.md"
docs_url: "https://docs.iced.rs/futures_core/stream/trait.FusedStream.html"
source_file: "src/futures_core/stream.rs"
source_lines: "146-149"
---

# `futures_core::stream::FusedStream`

```rust
pub trait FusedStream: Stream {
    // Required method
    fn is_terminated(&self) -> bool;
}
```

A stream which tracks whether or not the underlying stream should no longer be polled.

`is_terminated` will return `true` if a future should no longer be polled. Usually, this state occurs after `poll_next` (or `try_poll_next`) returned `Poll::Ready(None)`. However, `is_terminated` may also return `true` if a stream has become inactive and can no longer make progress and should be ignored or dropped rather than being polled again.

**Source:** [stream.rs](../../src/futures_core/stream.rs.md) (lines 146-149)

## Required Methods

### `is_terminated`

```rust
fn is_terminated(&self) -> bool
```

Returns `true` if the stream should no longer be polled.

## Implementors
