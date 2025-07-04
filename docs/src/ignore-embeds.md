# Ignore Embeds

Sometimes you may want preprocessor to ignore some embeds.

There are two ways to do it:

<!-- embed ignore begin -->

- **Inline**: You can use `{% embed-ignore %}` syntax to ignore the embed in the current line.
- **Block**: You can use `comments` to wrap the content you want to ignore.

## Inline Ignore

To ignore an embed in the current line, you can use the `{% embed-ignore %}`. The preprocessor will automatically convert it to `{% embed %}`:

For example:

```text
{% embed-ignore youtube id="DyTCOwB0DVw" loading="lazy" %}
```

<!-- embed ignore end -->

Will be replaced as:

```text
{% embed-ignore youtube id="DyTCOwB0DVw" loading="lazy" %}
```

## Block Ignore

To ignore a block of embeds, you can use HTML comments to wrap the content you want to ignore. The preprocessor will not render any embeds inside these comments.

- `<!-- embed ignore begin -->`
- `<!-- embed ignore end -->`

For example:

```text
<!-- embed ignore begin -->
{% embed youtube id="DyTCOwB0DVw" loading="lazy" %}
<!-- embed ignore end -->
```

Will also be shown as:

```text
{% embed-ignore youtube id="DyTCOwB0DVw" loading="lazy" %}
```

## Priorities

Block ignore has **higher** priority than inline ignore, so if you wrap an inline ignore in a block ignore, it will keep as it is and will not be changed.

For example:

```text
<!-- embed ignore begin -->
{% embed-ignore youtube id="DyTCOwB0DVw" loading="lazy" %}
<!-- embed ignore end -->
```

Will be shown as:

<!-- embed ignore begin -->

```text
{% embed-ignore youtube id="DyTCOwB0DVw" loading="lazy" %}
```

<!-- embed ignore end -->
