---
title: "sensor"
kind: function
path: "iced_widget::sensor"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.sensor.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.sensor.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "680-682"
---

# `iced_widget::sensor`

```rust
pub fn sensor<'a, Message, W>(content: W) -> Sensor<'a, (), Message, W>
```

Creates a new [`Sensor`](sensor/struct.Sensor.md) widget.

A [`Sensor`](sensor/struct.Sensor.md) widget can generate messages when its contents are shown, hidden, or resized.

It can even notify you with anticipation at a given distance!

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 680-682)
