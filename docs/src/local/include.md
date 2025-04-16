# Include

The include app is for including source file and wrapped it as markdown code block.

The language is automatically detected by the file name extension. You can override it by passing `lang` option. The file path should be relative to the execution directory.

## Options

| Option | Description                                            | Required | Default |
| :----- | :----------------------------------------------------- | :------- | :------ |
| file   | File to include, relative to the exection directory    | Yes      | - -     |
| lang   | This will override the automatically detected language | No       | - -     |

## Example

<!-- embed ignore begin -->

```text
{% embed include file="docs/src/SUMMARY.md" %}
```

<!-- embed ignore end -->

This will include the [docs/src/SUMMARY.md](https://github.com/MR-Addict/mdbook-embedify/blob/main/docs/src/SUMMARY.md) file and wrap it as a code block which is the source code of this book's summary.

{% embed include file="docs/src/SUMMARY.md" %}
