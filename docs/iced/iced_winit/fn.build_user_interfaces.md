---
title: "build_user_interfaces"
kind: function
path: "iced_winit::build_user_interfaces"
crate: "iced_winit"
module: "iced_winit"
file: "iced_winit/fn.build_user_interfaces.md"
docs_url: "https://docs.iced.rs/iced_winit/fn.build_user_interfaces.html"
source_file: "src/iced_winit/lib.rs"
source_lines: "1820-1870"
---

# `iced_winit::build_user_interfaces`

```rust
pub fn build_user_interfaces<'a, P: Program, C>(
    program: &'a Instance<P>,
    window_manager: &mut Manager<P, C>,
    cached_user_interfaces: FxHashMap<Id, Cache>,
    proxy: &mut Proxy<P::Message>,
) -> FxHashMap<Id, UserInterface<'a, P::Message, P::Theme, P::Renderer>> where
    C: Compositor<Renderer = P::Renderer>,
    P::Theme: Base,
```

Build the user interface for every window.

**Source:** [lib.rs](../src/iced_winit/lib.rs.md) (lines 1820-1870)
