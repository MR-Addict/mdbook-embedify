# Embed Ignore Testing

This page tests the ignore functionality for preventing embeds from being rendered.

## Inline Ignore Tests

### Basic Inline Ignore

This should show the embed syntax as text instead of rendering it:

{% embed include file="src/samples/hello.py" %}

### Inline Ignore with YouTube

This should show the YouTube embed syntax as text:

{% embed youtube id="DyTCOwB0DVw" loading="lazy" %}

### Inline Ignore with Multiple Options

This should show the include embed with range options as text:

{% embed include file="src/samples/fibonacci.js" range="1-5" lang="python" %}

## Block Ignore Tests

### Basic Block Ignore

This section contains embeds that should not be rendered:

<!-- embed ignore begin -->

{% embed include file="src/samples/hello.py" %}
{% embed include file="src/samples/fibonacci.js" range="1-3" %}

<!-- embed ignore end -->

### Block Ignore with Multiple Embeds

Multiple embeds in a block should all be ignored:

<!-- embed ignore begin -->

{% embed include file="src/samples/package.json" %}
{% embed include file="src/samples/config.md" %}
{% embed youtube id="testid123" %}

<!-- embed ignore end -->

### Nested Block with Inline Ignore

Block ignore should take precedence over inline ignore:

<!-- embed ignore begin -->

{% embed-ignore include file="src/samples/hello.py" range="1-2" %}
{% embed include file="src/samples/fibonacci.js" %}

<!-- embed ignore end -->

## Mixed Content Tests

### Valid Embeds Mixed with Ignored

This embed should render:

<!-- {% embed include file="src/samples/hello.py" range="1-3" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" data-option-range="1-3" style="display:none"></div>

```python
#!/usr/bin/env python3


```

This embed should be ignored (inline):

{% embed include file="src/samples/fibonacci.js" range="1-3" %}

This embed should also render:

<!-- {% embed include file="src/samples/package.json" lang="javascript" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/package.json" data-option-lang="javascript" style="display:none"></div>

```javascript
{
  "name": "test-project",
  "version": "1.0.0",
  "description": "A test project for include functionality",
  "main": "index.js",
  "scripts": {
    "start": "node index.js",
    "test": "jest",
    "build": "webpack"
  },
  "dependencies": {
    "express": "^4.18.0",
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "jest": "^29.0.0",
    "webpack": "^5.0.0"
  }
}
```

### Block Ignore with Valid Content Around

Content before the ignored block.

<!-- embed ignore begin -->

{% embed include file="src/samples/hello.py" %}
{% embed include file="src/samples/fibonacci.js" %}

<!-- embed ignore end -->

Content after the ignored block.

<!-- {% embed include file="src/samples/config.md" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/config.md" style="display:none"></div>

```markdown
# Sample Configuration

This is a sample configuration file for testing include functionality.

## Database Settings

- Host: localhost
- Port: 5432
- Database: test_db

## Features

- Auto-backup: enabled
- Logging: debug
- Cache: redis

## Notes

> This is just for testing purposes.
>
> Do not use in production!
```

## Edge Cases

### Empty Block Ignore

This should be handled gracefully:

<!-- embed ignore begin -->
<!-- embed ignore end -->

### Multiple Block Ignores

First block:

<!-- embed ignore begin -->

{% embed include file="src/samples/hello.py" %}

<!-- embed ignore end -->

Second block:

<!-- embed ignore begin -->

{% embed include file="src/samples/fibonacci.js" %}

<!-- embed ignore end -->

### Block Ignore with Text Content

<!-- embed ignore begin -->

This is regular text content.
{% embed include file="src/samples/hello.py" %}
More text content here.
{% embed include file="src/samples/package.json" %}
Final text content.

<!-- embed ignore end -->

### Malformed Ignore Syntax Tests

This should still work (extra spaces):

<!-- embed ignore begin -->

{% embed include file="src/samples/hello.py" %}

<!-- embed ignore end -->

This should also work (case insensitive):

<!--  embed  ignore  begin  -->

{% embed include file="src/samples/fibonacci.js" %}

<!--  embed  ignore  end  -->

## Priority Tests

### Block Takes Precedence Over Inline

The inline ignore should be preserved as-is within the block:

<!-- embed ignore begin -->

{% embed-ignore include file="src/samples/hello.py" %}

<!-- embed ignore end -->

### Multiple Inline Ignores

{% embed include file="src/samples/hello.py" range="1-2" %}
{% embed include file="src/samples/fibonacci.js" range="3-5" %}
{% embed youtube id="test123" %}

## Real-world Examples

### Documentation Example

When documenting how to use embeds, you want to show the syntax without rendering:

To include a Python file, use:
{% embed include file="path/to/file.py" %}

To include a specific range:
{% embed include file="path/to/file.js" range="1-10" %}

### Tutorial Content

<!-- embed ignore begin -->

Step 1: Add a YouTube video
{% embed youtube id="your-video-id" %}

Step 2: Include source code
{% embed include file="src/example.py" %}

<!-- embed ignore end -->

The above examples show how to use embeds without actually rendering them.
