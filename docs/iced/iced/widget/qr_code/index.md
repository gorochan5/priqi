---
title: "qr_code"
kind: module
path: "iced::widget::qr_code"
crate: "iced"
module: "iced::widget"
file: "iced/widget/qr_code/index.md"
docs_url: "https://docs.iced.rs/iced/widget/qr_code/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "149"
---

# `iced::widget::qr_code`

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

## Structs

- [`Data`](struct.Data.md) — The data of a [`QRCode`](../struct.QRCode.md).
- [`QRCode`](struct.QRCode.md) — A type of matrix barcode consisting of squares arranged in a grid which can be read by an imaging device, such as a camera.
- [`Style`](struct.Style.md) — The appearance of a QR code.

## Enums

- [`Error`](enum.Error.md) — An error that occurred when building a [`Data`](struct.Data.md) for a [`QRCode`](../struct.QRCode.md).
- [`ErrorCorrection`](enum.ErrorCorrection.md) — The error correction level.
- [`Version`](enum.Version.md) — The size of a [`QRCode`](../struct.QRCode.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`QRCode`](../struct.QRCode.md).

## Functions

- [`default`](fn.default.md) — The default style of a [`QRCode`](../struct.QRCode.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`QRCode`](../struct.QRCode.md).
