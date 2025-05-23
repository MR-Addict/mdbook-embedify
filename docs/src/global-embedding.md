# Global Embedding

Some apps allow you to automatically embed to every chapter. You can do this by modifying `book.toml` file to enable them.

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

footer.enable = true
footer.message = "Copyright © 2025 • Created with ❤️ by [MR-Addict](https://github.com/MR-Addict)"

announcement-banner.enable = true
announcement-banner.id = "0.2.13"
announcement-banner.message = "*Version **0.2.13** now has relased, check it out [here](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.13).*"

giscus.enable = true
giscus.repo = "MR-Addict/mdbook-embedify"
giscus.repo-id = "R_XXXXXXXXXX"
giscus.category = "General"
giscus.category-id = "DIC_XXXXXXXXXXXXXXXX"
giscus.reactions-enabled = "1"
giscus.theme = "book"
giscus.lang = "en"
giscus.loading = "lazy"
```

You can see more details about each app at its own page.
