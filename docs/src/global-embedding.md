# Global Embedding

Some apps allow you to automatically embed to every chapter. You can do this by modifying `book.toml` file to enable them.

For example:

```toml
[preprocessor.embedify]
scroll-to-top.enable = true
```

> Attention 💥
>
> When you do this, you don't need to add `{% embed-ignore scroll-to-top %}` manually. It will be automatically added it to every chapter. If you do, it will be rendered twice.

Below is a full list of apps that support global configuration:

{% embed include file="book.toml" range="44-64" %}

You can see more details about each app at its own page.
