---
title: "present_mode_from_env"
kind: function
path: "iced_wgpu::window::compositor::present_mode_from_env"
crate: "iced_wgpu"
module: "iced_wgpu::window::compositor"
file: "iced_wgpu/window/compositor/fn.present_mode_from_env.md"
docs_url: "https://docs.iced.rs/iced_wgpu/window/compositor/fn.present_mode_from_env.html"
source_file: "src/iced_wgpu/window/compositor.rs"
source_lines: "446-458"
---

# `iced_wgpu::window::compositor::present_mode_from_env`

```rust
pub fn present_mode_from_env() -> Option<PresentMode>
```

Obtains a [`wgpu::PresentMode`] from the current environment configuration, if set.

The value returned by this function can be changed by setting the `ICED_PRESENT_MODE` env variable. The possible values are:

- `vsync` → [`wgpu::PresentMode::AutoVsync`]
- `no_vsync` → [`wgpu::PresentMode::AutoNoVsync`]
- `immediate` → [`wgpu::PresentMode::Immediate`]
- `fifo` → [`wgpu::PresentMode::Fifo`]
- `fifo_relaxed` → [`wgpu::PresentMode::FifoRelaxed`]
- `mailbox` → [`wgpu::PresentMode::Mailbox`]

**Source:** [compositor.rs](../../../src/iced_wgpu/window/compositor.rs.md) (lines 446-458)
