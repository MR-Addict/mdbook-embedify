Scroll to top button app has no options. You can use it like this:

<!-- embed ignore begin -->

```text
{% embed scroll-to-top %}
```

<!-- embed ignore end -->

Typically, we want to use it for the whole book. You can do this by adding below options to `book.toml` file:

```toml
[preprocessor.embedify]
scroll-to-top.enable = true
```

This book uses this option. You can see it at the bottom right corner of this page. But it only shows when pages are long enough to scroll. Or you can see it my another book [Notes](https://mr-addict.github.io/notes).
