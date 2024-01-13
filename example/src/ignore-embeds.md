# Ignore Embeds

Sometimes you may want preprocessors to ignore some embeds.

You can do it by wrapping content that you want to ignore with below two comments:

- `<!-- embed ignore begin -->`
- `<!-- embed ignore end -->`

For example:

```text
<!-- embed ignore begin -->

{% embed youtube id="DyTCOwB0DVw" %}

<!-- embed ignore end -->
```

And youtube embed won't be rendered.