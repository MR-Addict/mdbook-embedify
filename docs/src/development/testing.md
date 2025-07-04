# Testing Guide

> Attention 💥
>
> Support since [v0.2.17](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.17).

## Test Suite Overview

mdbook-embedify covering two critical modules:

| Module                 | Coverage                           |
| ---------------------- | ---------------------------------- |
| **Parser Testing**     | Placeholder & embed syntax parsing |
| **Language Detection** | File extension to language mapping |

## Quick Commands

```bash
# Run complete test suite
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run specific test suites
cargo test --test parser
cargo test --test detect_lang

# Run tests with timing information
cargo test -- --show-output
```
