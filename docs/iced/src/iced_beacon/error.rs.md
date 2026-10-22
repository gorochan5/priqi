---
title: "src/iced_beacon/error.rs"
kind: source
path: "src/iced_beacon/error.rs"
crate: "iced_beacon"
file: "src/iced_beacon/error.rs.md"
docs_url: "https://docs.iced.rs/src/iced_beacon/error.rs.html"
source_file: "src/iced_beacon/error.rs"
---

# `src/iced_beacon/error.rs`

```rust
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("input/output operation failed: {0}")]
    IOFailed(#[from] io::Error),
    #[error("decoding failed: {0}")]
    DecodingFailed(#[from] Box<bincode::ErrorKind>),
}
```
