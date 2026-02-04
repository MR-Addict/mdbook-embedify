# Include App Testing

This page tests the include app functionality for including source files with various options and configurations.

## Basic File Inclusion

### Python File (Auto-detected Language)

Include a Python file with automatic language detection:

<!-- {% embed include file="src/samples/hello.py" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" style="display:none"></div>

```python
#!/usr/bin/env python3


def greet(name):
    """Greet someone with a hello message."""
    return f"Hello, {name}!"


def main():
    name = "World"
    message = greet(name)
    print(message)


if __name__ == "__main__":
    main()
```

### JavaScript File (Auto-detected Language)

Include a JavaScript file with automatic language detection:

<!-- {% embed include file="src/samples/fibonacci.js" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/fibonacci.js" style="display:none"></div>

```javascript
function fibonacci(n) {
  if (n <= 1) {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}

function generateSequence(length) {
  const sequence = [];
  for (let i = 0; i < length; i++) {
    sequence.push(fibonacci(i));
  }
  return sequence;
}

console.log(generateSequence(10));
```

### JSON File (Auto-detected Language)

Include a JSON file with automatic language detection:

<!-- {% embed include file="src/samples/package.json" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/package.json" style="display:none"></div>

```json
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

## Range-based Inclusion

### First 5 Lines Only

Include only the first 5 lines of the Python file:

<!-- {% embed include file="src/samples/hello.py" range="-5" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" data-option-range="-5" style="display:none"></div>

```python
#!/usr/bin/env python3


def greet(name):
    """Greet someone with a hello message."""
```

### Lines 3-8

Include lines 3 through 8 of the JavaScript file:

<!-- {% embed include file="src/samples/fibonacci.js" range="3-8" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/fibonacci.js" data-option-range="3-8" style="display:none"></div>

```javascript
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}

function generateSequence(length) {
```

### From Line 10 to End

Include from line 10 to the end of the JavaScript file:

<!-- {% embed include file="src/samples/fibonacci.js" range="10-" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/fibonacci.js" data-option-range="10-" style="display:none"></div>

```javascript
  for (let i = 0; i < length; i++) {
    sequence.push(fibonacci(i));
  }
  return sequence;
}

console.log(generateSequence(10));
```

### Single Line Range

Include only line 1 of the Python file:

<!-- {% embed include file="src/samples/hello.py" range="1-1" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" data-option-range="1-1" style="display:none"></div>

```python
#!/usr/bin/env python3
```

## Language Override

### Python File as Plain Text

Include Python file but override language to text:

<!-- {% embed include file="src/samples/hello.py" lang="text" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" data-option-lang="text" style="display:none"></div>

```text
#!/usr/bin/env python3


def greet(name):
    """Greet someone with a hello message."""
    return f"Hello, {name}!"


def main():
    name = "World"
    message = greet(name)
    print(message)


if __name__ == "__main__":
    main()
```

### JSON File as JavaScript

Include JSON file but override language to JavaScript:

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

## Explicit Codeblock Wrapping

### Markdown File (Wrapped in Code Block)

Include markdown file wrapped as a code block:

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

### Python File with Range

Include lines 3-6 of Python file as code block:

<!-- {% embed include file="src/samples/hello.py" range="3-6" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" data-option-range="3-6" style="display:none"></div>

```python

def greet(name):
    """Greet someone with a hello message."""
    return f"Hello, {name}!"
```

## Combined Options

### Range + Language Override

Include lines 1-3 of JavaScript file with Python syntax highlighting:

<!-- {% embed include file="src/samples/fibonacci.js" range="1-3" lang="python" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/fibonacci.js" data-option-range="1-3" data-option-lang="python" style="display:none"></div>

```python
function fibonacci(n) {
  if (n <= 1) {
    return n;
```

### Range + Language Override (Continued)

Include lines 2-4 of JSON file with language override:

<!-- {% embed include file="src/samples/package.json" range="2-4" lang="json" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/package.json" data-option-range="2-4" data-option-lang="json" style="display:none"></div>

```json
  "name": "test-project",
  "version": "1.0.0",
  "description": "A test project for include functionality",
```

### Multiple Options Combined

Include lines 5-10 of Python file with language override:

<!-- {% embed include file="src/samples/hello.py" range="5-10" lang="bash" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" data-option-range="5-10" data-option-lang="bash" style="display:none"></div>

```bash
    """Greet someone with a hello message."""
    return f"Hello, {name}!"


def main():
    name = "World"
```

## Edge Cases

### Empty Range (Should Include Full File)

Test empty range defaults to full file:

<!-- {% embed include file="src/samples/hello.py" range="" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" data-option-range="" style="display:none"></div>

```python
#!/usr/bin/env python3


def greet(name):
    """Greet someone with a hello message."""
    return f"Hello, {name}!"


def main():
    name = "World"
    message = greet(name)
    print(message)


if __name__ == "__main__":
    main()
```

### Invalid Range (Should Default to Full File)

Test invalid range defaults to full file:

<!-- {% embed include file="src/samples/hello.py" range="invalid-range" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" data-option-range="invalid-range" style="display:none"></div>

```python
#!/usr/bin/env python3


def greet(name):
    """Greet someone with a hello message."""
    return f"Hello, {name}!"


def main():
    name = "World"
    message = greet(name)
    print(message)


if __name__ == "__main__":
    main()
```

### Out-of-bounds Range (Should Clamp to Valid Range)

Test range that goes beyond file length:

<!-- {% embed include file="src/samples/hello.py" range="1-1000" %} -->

<div data-embedify data-app="include" data-option-file="src/samples/hello.py" data-option-range="1-1000" style="display:none"></div>

```python
#!/usr/bin/env python3


def greet(name):
    """Greet someone with a hello message."""
    return f"Hello, {name}!"


def main():
    name = "World"
    message = greet(name)
    print(message)


if __name__ == "__main__":
    main()
```
