# Global Configuration

Some apps allow you to automatically add to every chapter. You can do this by adding `app-name = true` to embedify preprocessor in `book.toml` file to enable it.

For example:

```toml
[preprocessor.embedify]
scroll-to-top = true
```

<!-- embed ignore begin -->

> 💥Attention
>
> When you do this, you don't need to add `{% embed scroll-to-top %}` manually. It will be automatically added it to every chapter. Otherwise, it will be rendered twice.

<!-- embed ignore end -->

Now `scroll-to-top` is the only app that supports global configuration.
