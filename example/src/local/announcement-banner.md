## Options

| Option  | Description                                                  | Required | Default |
| :------ | :----------------------------------------------------------- | :------- | :------ |
| name    | Announcement name                                            | Yes      | - -     |
| message | Announcement message, supports markdown syntax               | Yes      | - -     |
| style   | Supports style: **default**, **ocean**, **forest**, **lava** | No       | default |

## Example

<!-- embed ignore begin -->

```text
{% embed announcement-banner name="0.2.2" style="default" message="*Version **0.2.2** now has relased, check it out [here](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.2).*" %}
```

<!-- embed ignore end -->

This book's announcement banner is enabled, you can see it at the top of this page.

However, you may want to enable it for the whole book. You can do this by adding below options to `book.toml` file:

```toml
[preprocessor.embedify]
announcement-banner.enable = true
announcement-banner.name = "0.2.2"
announcement-banner.style = "default"
announcement-banner.message = "*Version **0.2.2** now has relased, check it out [here](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.2).*"
```

Note that announcement banner name must be **unique**, otherwise it won't be shown if there is another announcement banner with the same name when user closed it.
