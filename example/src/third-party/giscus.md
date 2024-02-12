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

## Good to know

Giscus is only supported while your app is deployed. You can't see it if you open it with **file://** protocol which is mdbook default behavior.

The simple way is to serve your book with a static server so that you can see the giscus by opening it with **http://** protocol. Typically, there is no problem if you deploy your book to GitHub Pages or Netlify. But if you want to see it locally, you can use below commands to serve your book.

**node.js installed**

```sh
npx serve book
```

**python installed**

```sh
python -m http.server --directory book
```
