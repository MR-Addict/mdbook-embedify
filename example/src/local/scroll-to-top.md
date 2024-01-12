Scroll to top button app has no options. You can use it like this:

<!-- embed ignore begin -->

```text
{% embed scroll-to-top %}
```

<!-- embed ignore end -->

This only works for chapter that embeds this, not for the whole book. Most of cases, you want to use it for the whole book. You can do this by adding `scroll-to-top` option to embedify preprocessor in `book.toml` file like this:

```toml
[preprocessor.embedify]
scroll-to-top = true
```

<!-- embed ignore begin -->

> 💥Attention
>
> When you do this, you don't need to add `{% embed scroll-to-top %}` to every chapter.

<!-- embed ignore end -->

This book uses this option. You can see it at the bottom right corner of this page. But it only shows pages that are longer enough to scroll. You can see it in action by scrolling down this page. Or you can see it my another book [Notes](https://mr-addict.github.io/notes).
