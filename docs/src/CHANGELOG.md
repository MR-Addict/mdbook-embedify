# Changelog

This page tracks all notable changes to mdbook-embedify. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.3.1] - 2026-02-04

### ✨ New Features

- **YAML Configuration**: Migrated language detection configuration from JSON to YAML format for better readability and maintainability

### 🔧 Improvements

- **mdBook Compatibility**: Updated to support mdbook 0.5.x versions

## [0.2.18] - 2025-10-14

### ✨ New Features

- **Enhanced App Rendering**: Added additional information display to final app rendering, providing more context to users about embedded content

### 🔧 Improvements

- Better template documentation and examples for creating custom templates
- Improved include app functionality with enhanced error handling

## [0.2.17] - 2025-01-05 🎉

### ✨ New Features

<!-- embed ignore begin -->

- **Embed Ignore Functionality**: You can now prevent specific embed blocks from being processed using `embed-ignore`

  ```text
  {% embed-ignore codepen user="example" slug="demo" %}
  ```

  This is perfect for documentation where you want to show embed syntax without processing it.

<!-- embed ignore end -->

- **Comprehensive Testing Suite**: Added robust testing framework including:
  - Book integration tests that validate entire mdbook builds
  - Parser testing for embed syntax validation
  - Language detection testing
  - Automated comparison of generated vs expected output

- **Enhanced Language Detection**: Major improvements to file extension mapping:
  - Support for wildcard matching in filename patterns
  - Multi-dot file extensions (`.spec.ts`, `.test.js`, etc.)
  - More accurate language detection using consolidated patterns

- **Development Documentation**: Added comprehensive guides for contributors and developers

### 🔧 Improvements

- **Include App Enhancements**:
  - Added `type` and `range` options for more flexible content inclusion
  - Better handling of markdown files with automatic code block wrapping
  - Wildcard matching support for include files

- **Custom Template Support**: Configure custom templates folder for your specific needs

- **Better Error Reporting**: Enhanced error messages throughout the system

### 🐛 Bug Fixes

- Improved mdbook installation process with existing installation checks
- Fixed language detection issues with complex file extensions
- Resolved regex handling problems
- Better handling of draft chapters
- Fixed various CI/CD workflow issues

### 🔧 Technical Improvements

- Rewritten parser using Rust Pest for better performance
- Enhanced HTML minification process
- Canvas template improvements with dynamic height and touch support

## [0.2.16] - 2024-12-XX

### ✨ New Features

- **Advanced Language Detection**: Integrated hyperpolyglot for improved language detection accuracy
- Version number display in documentation interface

## [0.2.15] - 2024-12-XX

### ✨ New Features

- Enhanced canvas template with dynamic height and touch support

### 🔧 Improvements

- HTML minification now removes style and script comments for cleaner output

## [0.2.14] - 2024-12-XX

### ✨ New Features

- **Custom Templates**: Full support for custom templates in embed preprocessor
- **Canvas App Template**: New interactive canvas template for dynamic content

### 📚 Documentation

- Updated guides for creating new apps and using templates

## [0.2.13] - 2024-12-XX

### ✨ New Features

- **Enhanced Include App**: Added `type` and `range` options for precise content inclusion

### 🔧 Improvements

- Better error reporting and validation across the board

## [0.2.12] - 2024-12-XX

### ✨ New Features

- **Include App**: Brand new functionality for embedding content from other files
- Enhanced markdown code block support for better formatting

### 🐛 Bug Fixes

- Proper handling of draft chapters in mdbook
- Improved markdown file processing and rendering

---

## 🚀 What's Next?

Stay tuned for upcoming features and improvements! Check our [GitHub repository](https://github.com/MR-Addict/mdbook-embedify) for the latest updates and to contribute to the project.

## 🤝 Contributing

Found a bug or have a feature request? We'd love to hear from you:

- [Report issues](https://github.com/MR-Addict/mdbook-embedify/issues)
- [Submit pull requests](https://github.com/MR-Addict/mdbook-embedify/pulls)
- [Join discussions](https://github.com/MR-Addict/mdbook-embedify/discussions)
