# Giscus

[Giscus](https://giscus.app) is a comments system powered by GitHub Discussions. Let visitors leave comments and reactions on your website via GitHub! Heavily inspired by [utterances](https://github.com/utterance/utterances).

## Options

| Option            | Description                               | Required | Default |
| :---------------- | :---------------------------------------- | :------- | :------ |
| repo              | Repository                                | Yes      | - -     |
| repo-id           | Repository ID                             | Yes      | - -     |
| category          | Category                                  | Yes      | - -     |
| category-id       | Category ID                               | Yes      | - -     |
| reactions-enabled | Enable reactions                          | No       | 1       |
| theme             | Supports **book**, **light** and **dark** | No       | book    |
| lang              | Localization language                     | No       | en      |
| loading           | Supports **lazy** and **eager**           | No       | lazy    |

## Example

```text
{% embed-ignore giscus repo="MR-Addict/mdbook-embedify" repo-id="R_XXXXXXXXXX" category="General" category-id="DIC_XXXXXXXXXXXXXXXX" theme="book" loading="eager" %}
```

This book's giscus is enabled, you can see it at the bottom of this page. And you can also have a try by commenting below.

However, you may want to enable it for the whole book. You can do this by adding below options to `book.toml` file after `[preprocessor.embedify]` section:

```toml
giscus.enable = true
giscus.repo = "MR-Addict/mdbook-embedify"
giscus.repo-id = "R_XXXXXXXXXX"
giscus.category = "General"
giscus.category-id = "DIC_XXXXXXXXXXXXXXXX"
giscus.reactions-enabled = "1"
giscus.theme = "book"
giscus.lang = "en"
giscus.loading = "eager"
```

## Refuse to Connect

Giscus will refuse to connect if you build and preview your book with **file://** protocol. The easiest solution is to use some static server so that you can preview your book with **http://** protocol.

For exampe:

**node.js installed**

```sh
npx serve book -p 3000
```

Which will serve your book at [http://localhost:3000](http://localhost:3000).

**python installed**

```sh
python -m http.server --directory book 8080
```

Which will serve your book at [http://localhost:8080](http://localhost:8080).
