# Language Matching System

> Attention 💥
>
> Support since [v0.2.17](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.17).

The language detection system is used for [include](../local/include.md) app to provide accurate syntax highlighting based on file paths.

## Overview

The language detection system intelligently identifies programming languages from file paths using a sophisticated **three-tier matching system** with clear precedence rules:

| Priority          | Method               | Example                          | Description               |
| ----------------- | -------------------- | -------------------------------- | ------------------------- |
| **1st** (Highest) | **Exact filename**   | `Dockerfile` → `dockerfile`      | Perfect filename matches  |
| **2nd**           | **File extension**   | `script.js` → `javascript`       | Extension-based detection |
| **3rd** (Lowest)  | **Wildcard pattern** | `Dockerfile.prod` → `dockerfile` | Flexible pattern matching |

## Configuration System

### Configuration Location

Language definitions are stored in [src/assets/config/languages.yaml](https://github.com/MR-Addict/mdbook-embedify/tree/main/src/assets/config/languages.yaml). Each language entry supports multiple matching strategies.

### Language Definition Structure

```yaml
- name: "dockerfile"
  extensions:
    - ".dockerfile"
  filenames:
    - "Dockerfile"
    - "dockerfile"
    - "Containerfile"
    - "Dockerfile.*"
    - "dockerfile.*"
    - "*.dockerfile"
```

| Property     | Type   | Purpose                                     | Example                                       |
| ------------ | ------ | ------------------------------------------- | --------------------------------------------- |
| `name`       | String | Language identifier for syntax highlighting | `"javascript"`                                |
| `extensions` | Array  | File extensions (with dots)                 | `[".js", ".jsx"]`                             |
| `filenames`  | Array  | Exact filenames and wildcard patterns       | `["Dockerfile", "Makefile.*", "*.config.js"]` |

## Wildcard Pattern Syntax

### Supported Wildcards

| Wildcard | Regex | Description             | Example Pattern | Matches                                    |
| -------- | ----- | ----------------------- | --------------- | ------------------------------------------ |
| `*`      | `.*`  | Zero or more characters | `Dockerfile.*`  | `Dockerfile.base`, `Dockerfile.production` |
| `?`      | `.`   | Exactly one character   | `test?.txt`     | `test1.txt`, `testa.txt`                   |

## Examples

### 🐳 Dockerfile Variants

| Filename                | Result       | Reason                                      |
| ----------------------- | ------------ | ------------------------------------------- |
| `Dockerfile`            | `dockerfile` | ✅ Exact match (non-wildcard entry)         |
| `Dockerfile.base`       | `dockerfile` | 🔍 Pattern: `Dockerfile.*` (wildcard entry) |
| `Dockerfile.production` | `dockerfile` | 🔍 Pattern: `Dockerfile.*` (wildcard entry) |
| `dockerfile.dev`        | `dockerfile` | 🔍 Pattern: `dockerfile.*` (wildcard entry) |
| `web.dockerfile`        | `dockerfile` | 🔍 Pattern: `*.dockerfile` (wildcard entry) |
| `api.dockerfile`        | `dockerfile` | 🔍 Pattern: `*.dockerfile` (wildcard entry) |
| `Dockerfile.js`         | `javascript` | 📁 Extension: `.js` (overrides pattern)     |

### 🔨 Makefile Variants

| Filename          | Result     | Reason                                    |
| ----------------- | ---------- | ----------------------------------------- |
| `Makefile`        | `makefile` | ✅ Exact match (non-wildcard entry)       |
| `Makefile.local`  | `makefile` | 🔍 Pattern: `Makefile.*` (wildcard entry) |
| `Makefile.am`     | `makefile` | ✅ Exact match (non-wildcard entry)       |
| `custom.makefile` | `makefile` | 📁 Extension: `.makefile`                 |
| `build.mk`        | `makefile` | 📁 Extension: `.mk`                       |

### 🐚 Shell Configuration Files

| Filename        | Result  | Reason                                   |
| --------------- | ------- | ---------------------------------------- |
| `.bashrc`       | `shell` | ✅ Exact match (non-wildcard entry)      |
| `.zshrc`        | `shell` | ✅ Exact match (non-wildcard entry)      |
| `custom.bashrc` | `shell` | 🔍 Pattern: `*.bashrc` (wildcard entry)  |
| `my.profile`    | `shell` | 🔍 Pattern: `*.profile` (wildcard entry) |
| `script.sh`     | `shell` | 📁 Extension: `.sh`                      |

### 📦 Configuration Files

| Filename            | Result       | Reason                |
| ------------------- | ------------ | --------------------- |
| `package.json`      | `json`       | 📁 Extension: `.json` |
| `tsconfig.json`     | `json`       | 📁 Extension: `.json` |
| `webpack.config.js` | `javascript` | 📁 Extension: `.js`   |
| `.eslintrc.json`    | `json`       | 📁 Extension: `.json` |
