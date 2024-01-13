## Installation

There are two ways to install this plugin.

You can install it from crates.io using cargo.

```sh
cargo install mdbook-embedify
```

Or you can download the binary from [releases](https://github.com/mr-addict/mdbook-embedify/releases) page. But it only supports **Linux** and **Windows**, if you are using **MacOS**, you have to install it from crates.io.

After installation, add the following code to your `book.toml` file:

```toml
[preprocessor.embedify]
```

And that's it! You can now use `embed` macro to embed apps to your book.

## Syntax

The basic syntax is like this:

<!-- embed ignore begin -->

```text
{% embed app options[] %}
```

**options** are key-value based array seperated by space and its value must be wrapped by **quotes**.

For example:

```text
{% embed codepen user="MR-Addict" slug="NWBOqKw" height="600" theme="dark" %}
```

<!-- embed ignore end -->

You can see some examples at examples section.
