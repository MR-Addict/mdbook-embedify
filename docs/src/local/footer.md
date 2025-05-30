# Footer

The footer app is useful for displaying copyright information, privacy policy, and other legal information. It supports **markdown** syntax so that you can easily customize the message.

## Options

| Option  | Description                            | Required | Default |
| :------ | :------------------------------------- | :------- | :------ |
| message | Footer message, **markdown** supported | Yes      | - -     |

## Example

<!-- embed ignore begin -->

```text
{% embed footer message="Copyright © 2025 • Created with ❤️ by [MR-Addict](https://github.com/MR-Addict)" %}
```

<!-- embed ignore end -->

This book's footer is enabled, you can see it at the bottom of this page.

However, you may want to enable it for the whole book. You can do this by adding below options to `book.toml` file after `[preprocessor.embedify]` section:

{% embed include file="book.toml" range="49-50" %}
