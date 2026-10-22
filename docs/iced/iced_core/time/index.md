---
title: "time"
kind: module
path: "iced_core::time"
crate: "iced_core"
module: "iced_core"
file: "iced_core/time/index.md"
docs_url: "https://docs.iced.rs/iced_core/time/index.html"
source_file: "src/iced_core/time.rs"
source_lines: "1-30"
---

# `iced_core::time`

Keep track of time, both in native and web platforms!

## Structs

- [`Duration`](struct.Duration.md) — A `Duration` type to represent a span of time, typically used for system timeouts.
- [`Instant`](struct.Instant.md) — A measurement of a monotonically nondecreasing clock. Opaque and useful only with [`Duration`](struct.Duration.md).
- [`SystemTime`](struct.SystemTime.md) — A measurement of the system clock, useful for talking to external entities like the file system or other processes.

## Functions

- [`days`](fn.days.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of days.
- [`hours`](fn.hours.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of hours.
- [`milliseconds`](fn.milliseconds.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of milliseconds.
- [`minutes`](fn.minutes.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of minutes.
- [`seconds`](fn.seconds.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of seconds.
