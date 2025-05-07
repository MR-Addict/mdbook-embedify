# Include

The include app is for including source file and wrapped it as markdown code block.

The language is automatically detected by the file name extension. You can override it by passing `lang` option. The file path should be relative to the execution directory.

## Options

| Option | Description                                             | Required | Default |
| :----- | :------------------------------------------------------ | :------- | :------ |
| file   | File to include, relative to the exection directory     | Yes      | - -     |
| lang   | This will override the automatically detected language  | No       | - -     |
| range  | Range of lines to include, e.g. `1-10` or `1-` or `-10` | No       | - -     |
| type   | Include type, `insert` or `embed`                       | No       | `embed` |

> 💥Attention
>
> - When `range` is used, it will insert the specified lines.
> - The `insert` type will insert the file content into the markdown file directly, while the `embed` type will wrap it as a code block.

## Example

<!-- embed ignore begin -->

```text
{% embed include file="docs/src/SUMMARY.md" %}
```

<!-- embed ignore end -->

This will include the [docs/src/SUMMARY.md](https://github.com/MR-Addict/mdbook-embedify/blob/main/docs/src/SUMMARY.md) file and wrap it as a code block which is the source code of this book's summary.

{% embed include file="docs/src/SUMMARY.md" %}
