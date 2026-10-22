---
title: "qr_code"
kind: function
path: "iced_widget::qr_code"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.qr_code.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.qr_code.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1638-1643"
---

# `iced_widget::qr_code`

```rust
pub fn qr_code<'a, Theme>(data: &'a Data) -> QRCode<'a, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`QRCode`](qr_code/struct.QRCode.md) widget from the given [`Data`](qr_code/struct.Data.md).

QR codes display information in a type of two-dimensional matrix barcode.

## Example

```rust
use iced::widget::qr_code;

struct State {
   data: qr_code::Data,
}

#[derive(Debug, Clone)]
enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    qr_code(&state.data)
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1638-1643)
