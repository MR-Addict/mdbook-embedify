# Include

The include app is for including source file and wrapped it as markdown code block. The language is automatically detect by the file name extension. You can override the language by passing `lang` option. The file path is relative to the book root directory.

## Options

| Option | Description                                            | Required | Default |
| :----- | :----------------------------------------------------- | :------- | :------ |
| file   | File path to include                                   | Yes      | - -     |
| lang   | This will override the automatically detected language | No       | - -     |

## Example

<!-- embed ignore begin -->

```text
{% embed include file="docs/src/SUMMARY.md" %}
```

<!-- embed ignore end -->

This will include the [docs/src/SUMMARY.md](https://github.com/MR-Addict/mdbook-embedify/blob/main/docs/src/SUMMARY.md) file and wrap it as a code block which is the source code of this book's summary.

{% embed include file="docs/src/SUMMARY.md" %}
