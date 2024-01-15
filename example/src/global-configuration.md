# Global Configuration

Some apps allow you to automatically add to every chapter. You can do this by modifying `book.toml` file to enable them.

For example:

```toml
[preprocessor.embedify]
scroll-to-top.enable = true
```

<!-- embed ignore begin -->

> 💥Attention
>
> When you do this, you don't need to add `{% embed scroll-to-top %}` manually. It will be automatically added it to every chapter. If you do, it will be rendered twice.

<!-- embed ignore end -->

Below is a full list of apps that support global configuration:

```toml
[preprocessor.embedify]
scroll-to-top.enable = true

announcement-banner.enable = true
announcement-banner.id = "0.2.3"
announcement-banner.theme = "default"
announcement-banner.message = "*Version **0.2.3** now has relased, check it out [here](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.3).*"
```

You can see more details about each app at its own page.
