---
title: "screenshot"
kind: function
path: "iced_test::screenshot"
crate: "iced_test"
module: "iced_test"
file: "iced_test/fn.screenshot.md"
docs_url: "https://docs.iced.rs/iced_test/fn.screenshot.html"
source_file: "src/iced_test/lib.rs"
source_lines: "261-297"
---

# `iced_test::screenshot`

```rust
pub fn screenshot<P: Program + 'static>(
    program: &P,
    theme: &P::Theme,
    viewport: impl Into<Size>,
    scale_factor: f32,
    duration: Duration,
) -> Screenshot
```

Takes a screenshot of the given `Program` with the given theme, viewport, and scale factor after running it for the given [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html).

**Source:** [lib.rs](../src/iced_test/lib.rs.md) (lines 261-297)
