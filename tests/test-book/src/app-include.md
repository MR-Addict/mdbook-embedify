# Include App Testing

This page tests the include app functionality for including source files with various options and configurations.

## Basic File Inclusion

### Python File (Auto-detected Language)

Include a Python file with automatic language detection:

{% embed include file="src/samples/hello.py" %}

### JavaScript File (Auto-detected Language)

Include a JavaScript file with automatic language detection:

{% embed include file="src/samples/fibonacci.js" %}

### JSON File (Auto-detected Language)

Include a JSON file with automatic language detection:

{% embed include file="src/samples/package.json" %}

## Range-based Inclusion

### First 5 Lines Only

Include only the first 5 lines of the Python file:

{% embed include file="src/samples/hello.py" range="-5" %}

### Lines 3-8

Include lines 3 through 8 of the JavaScript file:

{% embed include file="src/samples/fibonacci.js" range="3-8" %}

### From Line 10 to End

Include from line 10 to the end of the JavaScript file:

{% embed include file="src/samples/fibonacci.js" range="10-" %}

### Single Line Range

Include only line 1 of the Python file:

{% embed include file="src/samples/hello.py" range="1-1" %}

## Language Override

### Python File as Plain Text

Include Python file but override language to text:

{% embed include file="src/samples/hello.py" lang="text" %}

### JSON File as JavaScript

Include JSON file but override language to JavaScript:

{% embed include file="src/samples/package.json" lang="javascript" %}

## Raw Type Inclusion

### Markdown File as Raw Content

Include markdown file as raw content (not wrapped in code block):

{% embed include file="src/samples/config.md" type="raw" %}

### Python File as Raw with Range

Include lines 3-6 of Python file as raw content:

{% embed include file="src/samples/hello.py" range="3-6" type="raw" %}

## Combined Options

### Range + Language Override

Include lines 1-3 of JavaScript file with Python syntax highlighting:

{% embed include file="src/samples/fibonacci.js" range="1-3" lang="python" %}

### Range + Raw Type

Include lines 2-4 of JSON file as raw content:

{% embed include file="src/samples/package.json" range="2-4" type="raw" %}

### Full Options Combination

Include lines 5-10 of Python file as raw content with language override:

{% embed include file="src/samples/hello.py" range="5-10" lang="bash" type="raw" %}

## Edge Cases

### Empty Range (Should Include Full File)

Test empty range defaults to full file:

{% embed include file="src/samples/hello.py" range="" %}

### Invalid Range (Should Default to Full File)

Test invalid range defaults to full file:

{% embed include file="src/samples/hello.py" range="invalid-range" %}

### Out-of-bounds Range (Should Clamp to Valid Range)

Test range that goes beyond file length:

{% embed include file="src/samples/hello.py" range="1-1000" %}
