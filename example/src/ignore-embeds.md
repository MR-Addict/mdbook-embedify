# Ignore Embeds

Sometimes you may want preprocessors to ignore some embeds.

You can do it by adding group of `<!-- embed ignore begin -->` and `<!-- embed ignore end -->` comment to the content where you want to ignore. For example:

```text

<!-- embed ignore begin -->

{% embed codepen user="MR-Addict" slug="NWBOqKw" height="600" theme="dark" %}

<!-- embed ignore end -->

```
