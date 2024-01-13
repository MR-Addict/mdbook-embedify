Scroll to top button app has no options. You can use it like this:

<!-- embed ignore begin -->

```text
{% embed scroll-to-top %}
```

This only works for chapter that embeds this, not for the whole book. Most of cases, you want to use it for the whole book. You can do this by adding `scroll-to-top` option to embedify preprocessor in `book.toml` file like this:

```toml
[preprocessor.embedify]
scroll-to-top = true
```

> 💥Attention
>
> When you do this, you don't need to add `{% embed scroll-to-top %}` manually. It will be automatically added it to every chapter. Otherwise, it will be rendered twice.

<!-- embed ignore end -->

This book uses this option. You can see it at the bottom right corner of this page. But it only shows when pages are long enough to scroll. Or you can see it my another book [Notes](https://mr-addict.github.io/notes).
