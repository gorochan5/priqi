---
title: "emulator"
kind: module
path: "iced_test::emulator"
crate: "iced_test"
module: "iced_test"
file: "iced_test/emulator/index.md"
docs_url: "https://docs.iced.rs/iced_test/emulator/index.html"
source_file: "src/iced_test/emulator.rs"
source_lines: "1-573"
---

# `iced_test::emulator`

Run your application in a headless runtime.

## Structs

- [`Action`](struct.Action.md) — An action that must be [performed](struct.Emulator.md) by the [`Emulator`](struct.Emulator.md).
- [`Emulator`](struct.Emulator.md) — A headless runtime that can run iced applications and execute [instructions](../instruction/enum.Instruction.md).

## Enums

- [`Event`](enum.Event.md) — An emulation event.
- [`Mode`](enum.Mode.md) — The strategy used by an [`Emulator`](struct.Emulator.md) when waiting for tasks to finish.
