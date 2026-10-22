---
title: "BoxFuture"
kind: type
path: "futures_core::future::BoxFuture"
crate: "futures_core"
module: "futures_core::future"
file: "futures_core/future/type.BoxFuture.md"
docs_url: "https://docs.iced.rs/futures_core/future/type.BoxFuture.html"
source_file: "src/futures_core/future.rs"
source_lines: "18"
---

# `futures_core::future::BoxFuture`

```rust
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
```

An owned dynamically typed [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) for use in cases where you can’t statically type your result or need to add some indirection.

This type is often created by the [`boxed`](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.boxed) method on [`FutureExt`](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html). See its documentation for more.

**Source:** [future.rs](../../src/futures_core/future.rs.md) (lines 18)
