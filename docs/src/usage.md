# Usage

## Installation

There are two ways to install this preprocessor.

You can install it from crates.io using cargo.

```sh
cargo install mdbook-embedify
```

Or you can download the binary from [releases](https://github.com/mr-addict/mdbook-embedify/releases) page.

Then you can check your installation by running:

```sh
mdbook-embedify --version
```

After installation, add the following code to your `book.toml` file:

```toml
[preprocessor.embedify]
```

And that's it! You can now use `embed` macro to embed apps to your book.

## Syntax

The basic syntax is like this:

```text
{% embed-ignore app options[] %}
```

**options** are key-value based array seperated by space and its value should be wrapped by **quotes**.

For example:

```text
{% embed-ignore codepen user="MR-Addict" slug="NWBOqKw" height="600" theme="dark" loading="lazy" %}
```

See more examples at apps section.
