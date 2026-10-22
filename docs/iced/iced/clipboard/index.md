---
title: "clipboard"
kind: module
path: "iced::clipboard"
crate: "iced"
module: "iced"
file: "iced/clipboard/index.md"
docs_url: "https://docs.iced.rs/iced/clipboard/index.html"
source_file: "src/iced/lib.rs"
source_lines: "564"
---

# `iced::clipboard`

Access the clipboard.

## Structs

- [`Image`](struct.Image.md) *(image-without-codecs)* — A clipboard image.

## Enums

- [`Content`](enum.Content.md) — Some clipboard content.
- [`Error`](enum.Error.md) — A clipboard error.
- [`Kind`](enum.Kind.md) — The kind of some clipboard [`Content`](enum.Content.md).

## Functions

- [`read`](fn.read.md) — Read the given [`Kind`](enum.Kind.md) of [`Content`](enum.Content.md) from the clipboard.
- [`read_files`](fn.read_files.md) — Read the current file paths of the clipboard.
- [`read_html`](fn.read_html.md) — Read the current HTML contents of the clipboard.
- [`read_image`](fn.read_image.md) *(image-without-codecs)* — Read the current [`Image`](struct.Image.md) of the clipboard.
- [`read_text`](fn.read_text.md) — Read the current text contents of the clipboard.
- [`write`](fn.write.md) — Write the given [`Content`](enum.Content.md) to the clipboard.
