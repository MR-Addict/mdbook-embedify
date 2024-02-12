# Giscus

[Giscus](https://giscus.app) is a comments system powered by GitHub Discussions. Let visitors leave comments and reactions on your website via GitHub! Heavily inspired by [utterances](https://github.com/utterance/utterances).

## Options

| Option            | Description      | Required | Default |
| :---------------- | :--------------- | :------- | :------ |
| repo              | Repository       | Yes      | - -     |
| repo-id           | Repository ID    | Yes      | - -     |
| category          | Category         | Yes      | - -     |
| category-id       | Category ID      | Yes      | - -     |
| reactions-enabled | Enable reactions | No       | 1       |
| theme             | Theme            | No       | light   |
| lang              | Language         | No       | en      |

## Example

<!-- embed ignore begin -->

```text
{% embed giscus repo="MR-Addict/mdbook-embedify" repo-id="R_kgDOLCxX0Q" category="General" category-id="DIC_kwDOLCxX0c4CdGx-" theme="light" %}
```

<!-- embed ignore end -->

This book's giscus is enabled, you can see it at the bottom of this page. And you can also have a try by commenting below.

However, you may want to enable it for the whole book. You can do this by adding below options to `book.toml` file:

```toml
giscus.enable = true
giscus.repo = "MR-Addict/mdbook-embedify"
giscus.repo-id = "R_kgDOLCxX0Q"
giscus.category = "General"
giscus.category-id = "DIC_kwDOLCxX0c4CdGx-"
giscus.reactions-enabled = "1"
giscus.theme = "light"
giscus.lang = "en"
```

## Refuse to Connect

Giscus will refuse to connect if you build and preview your book with **file://** protocol. The easiest solution is to use some static server so that you can preview your book with **http://** protocol.

For exampe:

**node.js installed**

```sh
npx serve book
```

Which will serve your book at [http://localhost:3000](http://localhost:3000).

**python installed**

```sh
python -m http.server --directory book
```

Which will serve your book at [http://localhost:8080](http://localhost:8080).
