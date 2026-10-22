---
title: "sections"
kind: function
path: "iced_widget::markdown::sections"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.sections.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.sections.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "677-726"
---

# `iced_widget::markdown::sections`

```rust
pub fn sections<'a>(
    items: &'a [Item],
) -> impl Iterator<Item = (Option<&'a Item>, &'a [Item])> + 'a
```

Groups the given Markdown [`Item`](enum.Item.md)s by [`Item::Heading`](enum.Item.md).

The returned iterator yields a `(Option<&Item>, &[Item])` pair for each group, without cloning any [`Item`](enum.Item.md):

- The first element is the heading that starts the group, if any. It is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) for the group of items that appears before the first heading, if there is any;
- The second element is the slice of items that follow the heading, up to (but not including) the next one.

Every item in the given slice is yielded exactly once: a heading is returned as the first element of the group it starts, and every other item is part of the slice that follows the last heading before it.

## Example

```rust
use iced_widget::markdown;

let items: Vec<_> = markdown::parse("# Title\n\nHello!\n\n# Subtitle\n\nMore!").collect();

let mut groups = markdown::sections(&items);

let (heading, contents) = groups.next().unwrap();
assert!(heading.is_some());
assert_eq!(contents.len(), 1);

let (heading, contents) = groups.next().unwrap();
assert!(heading.is_some());
assert_eq!(contents.len(), 1);

assert!(groups.next().is_none());
```

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 677-726)
