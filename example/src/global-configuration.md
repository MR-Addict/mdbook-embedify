# Global Configuration

Some apps are not third-party apps, for example **scroll-to-top** button. You can use it like this:

<!-- embed ignore begin -->

```text
{% embed scroll-to-top %}
```

<!-- embed ignore end -->

But this only works for chapter that embeds this app, not for the whole book. Most of cases, you want to use it for the whole book. You can do this by adding `scroll-to-top` option to embedify preprocessor in `book.toml` file like this:

```toml
[preprocessor.embedify]
scroll-to-top = true
```

And now `scroll-to-top` is the only app that supports global configuration.
