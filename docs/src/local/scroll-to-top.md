# Scroll to top button

Scroll to top button allows users to quickly smoothly scroll back to the top of the page.

## Options

Scroll to top button app has no options.

## Example

<!-- embed ignore begin -->

```text
{% embed scroll-to-top %}
```

<!-- embed ignore end -->

Typically, we want to use it for the whole book. You can do this by adding below options to `book.toml` file after `[preprocessor.embedify]` section:

```toml
scroll-to-top.enable = true
```

This book uses this option. You can see it at the bottom right corner of this page. But it only shows when pages are long enough to scroll. Or you can see it my another book [Notes](https://mr-addict.github.io/notes).
