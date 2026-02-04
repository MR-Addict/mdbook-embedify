# Include

> Attention 💥
>
> Support since [v0.2.12](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.12).
>
> Language is detected by file extension, which I manually referred, simplified and extended from [linguist](https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml).
>
> If you find any issues with the language detection, please open an [issue](https://github.com/MR-Addict/mdbook-embedify/issues). Currenltly language map is under [languages.yaml](https://github.com/MR-Addict/mdbook-embedify/blob/main/src/assets/config/languages.yaml).

> Tip 💡
>
> You can customize language detection rules in your `book.toml`. See [Language Matching](../development/language-matching.md#custom-configuration) for details.

The include app is for including source file and wrapping it as markdown code block.

The language is automatically detected by the file name extension. You can override it by passing `lang` option. The file path should be relative to book root directory.

## Options

| Option | Description                                             | Required | Default |
| :----- | :------------------------------------------------------ | :------- | :------ |
| file   | File to include, relative to book root directory        | Yes      | - -     |
| lang   | This will override the automatically detected language  | No       | - -     |
| range  | Range of lines to include, e.g. `1-10` or `1-` or `-10` | No       | - -     |

> Attention 💥
>
> When `range` is used, it will insert the specified lines starts from 1.

## Example

```text
{% embed-ignore include file="src/SUMMARY.md" %}
```

This will include the [src/SUMMARY.md](https://github.com/MR-Addict/mdbook-embedify/blob/main/docs/src/SUMMARY.md) file and wrap it as a markdown code block which is the source code of this book's summary.

{% embed include file="src/SUMMARY.md" %}
