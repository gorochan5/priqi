---
title: "LocalBoxFuture"
kind: type
path: "futures_core::future::LocalBoxFuture"
crate: "futures_core"
module: "futures_core::future"
file: "futures_core/future/type.LocalBoxFuture.md"
docs_url: "https://docs.iced.rs/futures_core/future/type.LocalBoxFuture.html"
source_file: "src/futures_core/future.rs"
source_lines: "27"
---

# `futures_core::future::LocalBoxFuture`

```rust
pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
```

`BoxFuture`, but without the `Send` requirement.

This type is often created by the [`boxed_local`](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.boxed_local) method on [`FutureExt`](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html). See its documentation for more.

**Source:** [future.rs](../../src/futures_core/future.rs.md) (lines 27)
