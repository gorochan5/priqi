---
title: "subscription"
kind: module
path: "iced::advanced::subscription"
crate: "iced"
module: "iced::advanced"
file: "iced/advanced/subscription/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/subscription/index.html"
source_file: "src/iced/advanced.rs"
source_lines: "2"
---

# `iced::advanced::subscription`

Write your own subscriptions.

## Enums

- [`Event`](enum.Event.md) — A subscription event.
- [`MacOS`](enum.MacOS.md) — Describes an event specific to MacOS
- [`PlatformSpecific`](enum.PlatformSpecific.md) — A platform specific event

## Traits

- [`Recipe`](trait.Recipe.md) — The description of a [`Subscription`](../../struct.Subscription.md).

## Functions

- [`from_recipe`](fn.from_recipe.md) — Creates a [`Subscription`](../../struct.Subscription.md) from a [`Recipe`](trait.Recipe.md) describing it.
- [`into_recipes`](fn.into_recipes.md) — Returns the different recipes of the [`Subscription`](../../struct.Subscription.md).

## Type Aliases

- [`EventStream`](type.EventStream.md) — A stream of runtime events.
- [`Hasher`](type.Hasher.md) — The hasher used for identifying subscriptions.
