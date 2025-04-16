# Mdbook Embedify

![Crates.io](https://img.shields.io/crates/v/mdbook-embedify) ![Crates.io](https://img.shields.io/crates/l/mdbook-embedify)

This is a [mdbook](https://rust-lang.github.io/mdBook) preprocessor plugin that allows you to embed apps to your book, like youtube, codepen and some other apps.

## 1. Installation

There are two ways to install this plugin.

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

## 2. Usage

Then you can use `embed` macro to embed an app. The syntax is like this:

```text
{% embed app options[] %}
```

**options** are key-value based array seperated by space and its value should be wrapped by **quotes**. For example:

```text
{% embed codepen user="MR-Addict" slug="NWBOqKw" height="600" theme="dark" loading="lazy" %}
```

## 3. Examples

![preview](preview.png)

You can see a live demo and more detailed documentation [here](https://mr-addict.github.io/mdbook-embedify).

## 4. More Apps

If you want to add more apps to this preprocessor, you can follow the instructions [here](https://mr-addict.github.io/mdbook-embedify/more-apps.html).
