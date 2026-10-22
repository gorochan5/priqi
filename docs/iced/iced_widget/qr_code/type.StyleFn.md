---
title: "StyleFn"
kind: type
path: "iced_widget::qr_code::StyleFn"
crate: "iced_widget"
module: "iced_widget::qr_code"
file: "iced_widget/qr_code/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/qr_code/type.StyleFn.html"
source_file: "src/iced_widget/qr_code.rs"
source_lines: "386"
---

# `iced_widget::qr_code::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`QRCode`](struct.QRCode.md).

**Source:** [qr_code.rs](../../src/iced_widget/qr_code.rs.md) (lines 386)
