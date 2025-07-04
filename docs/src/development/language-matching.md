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

Language definitions are stored in [src/assets/config/languages.json](https://github.com/MR-Addict/mdbook-embedify/tree/main/src/assets/config/languages.json). Each language entry supports multiple matching strategies.

### Language Definition Structure

```json
{
  "name": "dockerfile",
  "extensions": [".dockerfile"],
  "filenames": ["Dockerfile", "dockerfile", "Containerfile"],
  "filename_patterns": ["Dockerfile.*", "dockerfile.*", "*.dockerfile"]
}
```

| Property            | Type   | Purpose                                     | Example                           |
| ------------------- | ------ | ------------------------------------------- | --------------------------------- |
| `name`              | String | Language identifier for syntax highlighting | `"javascript"`                    |
| `extensions`        | Array  | File extensions (with dots)                 | `[".js", ".jsx"]`                 |
| `filenames`         | Array  | Exact filename matches                      | `["Dockerfile", "Makefile"]`      |
| `filename_patterns` | Array  | Glob-style wildcard patterns                | `["Dockerfile.*", "*.config.js"]` |

## Matching System Details

### 1. Exact Filename Matching (Highest Precedence)

The system first performs a **case-sensitive** exact match against the `filenames` array.

**Performance**: O(1) HashMap lookup

**Examples:**

| Filename     | Detected Language | Reason                               |
| ------------ | ----------------- | ------------------------------------ |
| `Dockerfile` | `dockerfile`      | Exact match in filenames array       |
| `Makefile`   | `makefile`        | Exact match in filenames array       |
| `.bashrc`    | `shell`           | Exact match for configuration file   |
| `.gitignore` | `gitignore`       | Exact match for version control file |

### 2. File Extension Matching

If no exact filename match is found, the system checks the **last file extension** (case-insensitive).

**Performance**: O(1) HashMap lookup after string processing

**Extension Rules:**

- ✅ **Case-insensitive** (`.JS` matches `.js`)
- ✅ **Last dot wins** (`file.test.js` → `.js`)
- ✅ **Requires dot** (stored as `".js"`, not `"js"`)

**Examples:**

| Filename       | Extension | Detected Language               |
| -------------- | --------- | ------------------------------- |
| `script.js`    | `.js`     | `javascript`                    |
| `app.py`       | `.py`     | `python`                        |
| `styles.css`   | `.css`    | `css`                           |
| `file.test.js` | `.js`     | `javascript` (last extension)   |
| `Script.JS`    | `.js`     | `javascript` (case-insensitive) |

### 3. Wildcard Pattern Matching (Lowest Precedence)

When neither exact nor extension matching succeeds, the system tries **glob-style wildcard patterns**.

**Performance**: O(n) regex matching where n = number of patterns

**Examples:**
| Filename | Matching Pattern | Detected Language |
|----------|------------------|-------------------|
| `Dockerfile.base` | `Dockerfile.*` | `dockerfile` |
| `web.dockerfile` | `*.dockerfile` | `dockerfile` |
| `Makefile.local` | `Makefile.*` | `makefile` |
| `config.prod.json` | `config.*.json` | `json` |

## Wildcard Pattern Syntax

### Supported Wildcards

| Wildcard | Regex | Description             | Example Pattern | Matches                                    |
| -------- | ----- | ----------------------- | --------------- | ------------------------------------------ |
| `*`      | `.*`  | Zero or more characters | `Dockerfile.*`  | `Dockerfile.base`, `Dockerfile.production` |
| `?`      | `.`   | Exactly one character   | `test?.txt`     | `test1.txt`, `testa.txt`                   |

## Examples

### 🐳 Dockerfile Variants

| Filename                | Result       | Reason                                  |
| ----------------------- | ------------ | --------------------------------------- |
| `Dockerfile`            | `dockerfile` | ✅ Exact match                          |
| `Dockerfile.base`       | `dockerfile` | 🔍 Pattern: `Dockerfile.*`              |
| `Dockerfile.production` | `dockerfile` | 🔍 Pattern: `Dockerfile.*`              |
| `dockerfile.dev`        | `dockerfile` | 🔍 Pattern: `dockerfile.*`              |
| `web.dockerfile`        | `dockerfile` | 🔍 Pattern: `*.dockerfile`              |
| `api.dockerfile`        | `dockerfile` | 🔍 Pattern: `*.dockerfile`              |
| `Dockerfile.js`         | `javascript` | 📁 Extension: `.js` (overrides pattern) |

### 🔨 Makefile Variants

| Filename          | Result     | Reason                    |
| ----------------- | ---------- | ------------------------- |
| `Makefile`        | `makefile` | ✅ Exact match            |
| `Makefile.local`  | `makefile` | 🔍 Pattern: `Makefile.*`  |
| `Makefile.am`     | `makefile` | ✅ Exact match            |
| `custom.makefile` | `makefile` | 📁 Extension: `.makefile` |
| `build.mk`        | `makefile` | 📁 Extension: `.mk`       |

### 🐚 Shell Configuration Files

| Filename        | Result  | Reason                  |
| --------------- | ------- | ----------------------- |
| `.bashrc`       | `shell` | ✅ Exact match          |
| `.zshrc`        | `shell` | ✅ Exact match          |
| `custom.bashrc` | `shell` | 🔍 Pattern: `*.bashrc`  |
| `my.profile`    | `shell` | 🔍 Pattern: `*.profile` |
| `script.sh`     | `shell` | 📁 Extension: `.sh`     |

### 📦 Configuration Files

| Filename             | Result       | Reason                        |
| -------------------- | ------------ | ----------------------------- |
| `package.json`       | `json`       | 📁 Extension: `.json`         |
| `tsconfig.json`      | `typescript` | ✅ Exact match                |
| `tsconfig.base.json` | `typescript` | 🔍 Pattern: `tsconfig.*.json` |
| `webpack.config.js`  | `javascript` | 📁 Extension: `.js`           |
| `.eslintrc.json`     | `json`       | 📁 Extension: `.json`         |
