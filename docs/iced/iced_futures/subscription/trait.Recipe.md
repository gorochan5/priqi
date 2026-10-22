---
title: "Recipe"
kind: trait
path: "iced_futures::subscription::Recipe"
crate: "iced_futures"
module: "iced_futures::subscription"
file: "iced_futures/subscription/trait.Recipe.md"
docs_url: "https://docs.iced.rs/iced_futures/subscription/trait.Recipe.html"
source_file: "src/iced_futures/subscription.rs"
source_lines: "448-461"
---

# `iced_futures::subscription::Recipe`

```rust
pub trait Recipe {
    type Output;

    // Required methods
    fn hash(&self, state: &mut Hasher);
    fn stream(self: Box<Self>, input: EventStream) -> BoxStream<Self::Output>;
}
```

The description of a [`Subscription`](struct.Subscription.md).

A [`Recipe`](trait.Recipe.md) is the internal definition of a [`Subscription`](struct.Subscription.md). It is used by runtimes to run and identify subscriptions. You can use it to create your own!

**Source:** [subscription.rs](../../src/iced_futures/subscription.rs.md) (lines 448-461)

## Required Associated Types

### `Output`

```rust
type Output
```

The events that will be produced by a [`Subscription`](struct.Subscription.md) with this [`Recipe`](trait.Recipe.md).

## Required Methods

### `hash`

```rust
fn hash(&self, state: &mut Hasher)
```

Hashes the [`Recipe`](trait.Recipe.md).

This is used by runtimes to uniquely identify a [`Subscription`](struct.Subscription.md).

### `stream`

```rust
fn stream(self: Box<Self>, input: EventStream) -> BoxStream<Self::Output>
```

Executes the [`Recipe`](trait.Recipe.md) and produces the stream of events of its [`Subscription`](struct.Subscription.md).

## Implementors
