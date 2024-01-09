# Mdbook Embedify

![Crates.io](https://img.shields.io/crates/v/mdbook-embedify) ![Crates.io](https://img.shields.io/crates/l/mdbook-embedify)

This is a [mdbook](https://rust-lang.github.io/mdBook) preprocessor plugin that allows you to embed apps to your book.

## 1. Installation

First, you need to install mdbook-embedify to your computer.You can install it from crates.io using cargo.

```sh
cargo install mdbook-embedify
```

After installation, add the following code to your `book.toml` file:

```toml
[preprocessor.embedify]
```

## 2. Usage

Then you can use `embed` macro to embed an app. The syntax is like this:

```text
{% embed app options[] %}
```

**options** are key-value based array seperated by space and its value must be wrapped by quotes. For example:

```text
{% embed codepen username="MR-Addict" slug="NWBOqKw" height="600" theme="dark" %}
```

## 3. Examples

![example](docs/example.png)

You can see a live demo and more detailed documentation [here](https://mr-addict.github.io/mdbook-embedify).
