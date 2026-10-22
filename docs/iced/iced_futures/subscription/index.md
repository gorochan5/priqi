---
title: "subscription"
kind: module
path: "iced_futures::subscription"
crate: "iced_futures"
module: "iced_futures"
file: "iced_futures/subscription/index.md"
docs_url: "https://docs.iced.rs/iced_futures/subscription/index.html"
source_file: "src/iced_futures/subscription.rs"
source_lines: "1-517"
---

# `iced_futures::subscription`

Listen to external events in your application.

## Structs

- [`Subscription`](struct.Subscription.md) — A request to listen to external events.
- [`Tracker`](struct.Tracker.md) — A registry of subscription streams.

## Enums

- [`Event`](enum.Event.md) — A subscription event.
- [`MacOS`](enum.MacOS.md) — Describes an event specific to MacOS
- [`PlatformSpecific`](enum.PlatformSpecific.md) — A platform specific event

## Traits

- [`Recipe`](trait.Recipe.md) — The description of a [`Subscription`](struct.Subscription.md).

## Functions

- [`filter_map`](fn.filter_map.md) — Creates a [`Subscription`](struct.Subscription.md) from a hashable id and a filter function.
- [`from_recipe`](fn.from_recipe.md) — Creates a [`Subscription`](struct.Subscription.md) from a [`Recipe`](trait.Recipe.md) describing it.
- [`into_recipes`](fn.into_recipes.md) — Returns the different recipes of the [`Subscription`](struct.Subscription.md).

## Type Aliases

- [`EventStream`](type.EventStream.md) — A stream of runtime events.
- [`Hasher`](type.Hasher.md) — The hasher used for identifying subscriptions.
