# Testing Guide

> Attention 💥
>
> Support since [v0.2.17](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.17).

## Pre-requisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org)
- [mdbook](https://rust-lang.github.io/mdBook)

Make sure you have the same **mdbook** version installed on your system as the one used in the project.

You can find the version in the `Cargo.toml` file under the `[dependencies]` section.

## Test Suite Overview

mdbook-embedify covering these critical modules:

| Module                 | Coverage                           |
| ---------------------- | ---------------------------------- |
| **Book Testing**       | Real book building and validation  |
| **Parser Testing**     | Placeholder & embed syntax parsing |
| **Language Detection** | File extension to language mapping |

## Quick Commands

```bash
# Run complete test suite
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run specific test suites
cargo test --test book
cargo test --test parser
cargo test --test detect_lang

# Run tests with timing information
cargo test -- --show-output

# Run tests quietly
cargo test -- --quiet
```
