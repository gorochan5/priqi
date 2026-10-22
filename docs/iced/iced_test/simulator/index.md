---
title: "simulator"
kind: module
path: "iced_test::simulator"
crate: "iced_test"
module: "iced_test"
file: "iced_test/simulator/index.md"
docs_url: "https://docs.iced.rs/iced_test/simulator/index.html"
source_file: "src/iced_test/simulator.rs"
source_lines: "1-474"
---

# `iced_test::simulator`

Run a simulation of your application without side effects.

## Structs

- [`Simulator`](struct.Simulator.md) — A user interface that can be interacted with and inspected programmatically.
- [`Snapshot`](struct.Snapshot.md) — A frame of a user interface rendered by a [`Simulator`](struct.Simulator.md).

## Functions

- [`click`](fn.click.md) — Returns the sequence of events of a click.
- [`press_key`](fn.press_key.md) — Returns the sequence of events of a key press.
- [`release_key`](fn.release_key.md) — Returns the sequence of events of a key release.
- [`scroll`](fn.scroll.md) — Returns the sequence of events of a scroll, with the given `delta`.
- [`simulator`](fn.simulator.md) — Creates a new [`Simulator`](struct.Simulator.md).
- [`tap_key`](fn.tap_key.md) — Returns the sequence of events of a “key tap” (i.e. pressing and releasing a key).
- [`typewrite`](fn.typewrite.md) — Returns the sequence of events of typewriting the given text in a keyboard.
