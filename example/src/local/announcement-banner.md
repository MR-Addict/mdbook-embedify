# Announcement Banner

Announcement banner allows you put important messages at the top of the page. It supports **markdown** syntax too.

## Options

| Option  | Description                                                  | Required | Default |
| :------ | :----------------------------------------------------------- | :------- | :------ |
| id      | Announcement id                                              | Yes      | - -     |
| message | Announcement message, markdown supported                     | Yes      | - -     |
| theme   | Supports theme: **default**, **ocean**, **forest**, **lava** | No       | default |

## Example

<!-- embed ignore begin -->

```text
{% embed announcement-banner id="0.2.5" theme="default" message="*Version **0.2.5** now has relased, check it out [here](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.5).*" %}
```

<!-- embed ignore end -->

This book's announcement banner is enabled, you can see it at the top of this page.

However, you may want to enable it for the whole book. You can do this by adding below options to `book.toml` file after `[preprocessor.embedify]` section:

```toml
announcement-banner.enable = true
announcement-banner.id = "0.2.5"
announcement-banner.theme = "default"
announcement-banner.message = "*Version **0.2.5** now has relased, check it out [here](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.5).*"
```

Note that announcement banner id must be **unique**, otherwise it won't be shown if there is another announcement banner with the same id when user closed it.
