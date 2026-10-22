---
title: "run"
kind: function
path: "iced_test::run"
crate: "iced_test"
module: "iced_test"
file: "iced_test/fn.run.md"
docs_url: "https://docs.iced.rs/iced_test/fn.run.html"
source_file: "src/iced_test/lib.rs"
source_lines: "123-257"
---

# `iced_test::run`

```rust
pub fn run<P: Program + 'static>(
    program: P,
    tests_dir: impl AsRef<Path>,
) -> Result<(), Error>
```

Runs an [`Ice`](ice/struct.Ice.md) test suite for the given `Program`.

Any `.ice` tests will be parsed from the given directory and executed in an [`Emulator`](emulator/struct.Emulator.md) of the given `Program`.

Remember that an [`Emulator`](emulator/struct.Emulator.md) executes the real thing! Side effects *will* take place. It is up to you to ensure your tests have reproducible environments by leveraging [`Preset`][program::Preset].

**Source:** [lib.rs](../src/iced_test/lib.rs.md) (lines 123-257)
