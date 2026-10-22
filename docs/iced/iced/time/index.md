---
title: "time"
kind: module
path: "iced::time"
crate: "iced"
module: "iced"
file: "iced/time/index.md"
docs_url: "https://docs.iced.rs/iced/time/index.html"
source_file: "src/iced/time.rs"
source_lines: "1-23"
---

# `iced::time`

Listen and react to time.

## Structs

- [`Duration`](struct.Duration.md) — A `Duration` type to represent a span of time, typically used for system timeouts.
- [`Instant`](struct.Instant.md) — A measurement of a monotonically nondecreasing clock. Opaque and useful only with [`Duration`](struct.Duration.md).
- [`SystemTime`](struct.SystemTime.md) — A measurement of the system clock, useful for talking to external entities like the file system or other processes.

## Functions

- [`days`](fn.days.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of days.
- [`every`](fn.every.md) *(tokio or smol or WebAssembly)* — Returns a [`Subscription`](../struct.Subscription.md) that produces messages at a set interval.
- [`hours`](fn.hours.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of hours.
- [`milliseconds`](fn.milliseconds.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of milliseconds.
- [`minutes`](fn.minutes.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of minutes.
- [`now`](fn.now.md) — Returns a [`Task`](../struct.Task.md) that produces the current [`Instant`](struct.Instant.md) by calling [`Instant::now`](struct.Instant.md).
- [`repeat`](fn.repeat.md) *(tokio or smol or WebAssembly)* — Returns a [`Subscription`](../struct.Subscription.md) that runs the given async function at a set interval; producing the result of the function as output.
- [`seconds`](fn.seconds.md) — Creates a [`Duration`](struct.Duration.md) representing the given amount of seconds.
