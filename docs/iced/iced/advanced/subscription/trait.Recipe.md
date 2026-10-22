---
title: "Recipe"
kind: trait
path: "iced::advanced::subscription::Recipe"
crate: "iced"
module: "iced::advanced::subscription"
file: "iced/advanced/subscription/trait.Recipe.md"
docs_url: "https://docs.iced.rs/iced/advanced/subscription/trait.Recipe.html"
---

# `iced::advanced::subscription::Recipe`

```rust
pub trait Recipe {
    type Output;

    // Required methods
    fn hash(&self, state: &mut FxHasher);
    fn stream(
        self: Box<Self>,
        input: Pin<Box<dyn Stream<Item = Event> + Send>>,
    ) -> Pin<Box<dyn Stream<Item = Self::Output> + Send>>;
}
```

The description of a [`Subscription`](../../struct.Subscription.md).

A [`Recipe`](trait.Recipe.md) is the internal definition of a [`Subscription`](../../struct.Subscription.md). It is used by runtimes to run and identify subscriptions. You can use it to create your own!

## Required Associated Types

### `Output`

```rust
type Output
```

The events that will be produced by a [`Subscription`](../../struct.Subscription.md) with this [`Recipe`](trait.Recipe.md).

## Required Methods

### `hash`

```rust
fn hash(&self, state: &mut FxHasher)
```

Hashes the [`Recipe`](trait.Recipe.md).

This is used by runtimes to uniquely identify a [`Subscription`](../../struct.Subscription.md).

### `stream`

```rust
fn stream( self: Box<Self>, input: Pin<Box<dyn Stream<Item = Event> + Send>>, ) -> Pin<Box<dyn Stream<Item = Self::Output> + Send>>
```

Executes the [`Recipe`](trait.Recipe.md) and produces the stream of events of its [`Subscription`](../../struct.Subscription.md).

## Implementors
