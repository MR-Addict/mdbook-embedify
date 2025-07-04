# Include App Testing

This page tests the include app functionality for including source files with various options and configurations.

## Basic File Inclusion

### Python File (Auto-detected Language)

Include a Python file with automatic language detection:

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

```python
#!/usr/bin/env python3


def greet(name):
    """Greet someone with a hello message."""
```

### Lines 3-8

Include lines 3 through 8 of the JavaScript file:

```javascript
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}

function generateSequence(length) {
```

### From Line 10 to End

Include from line 10 to the end of the JavaScript file:

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

```python
#!/usr/bin/env python3
```

## Language Override

### Python File as Plain Text

Include Python file but override language to text:

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

## Raw Type Inclusion

### Markdown File as Raw Content

Include markdown file as raw content (not wrapped in code block):

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

### Python File as Raw with Range

Include lines 3-6 of Python file as raw content:


def greet(name):
    """Greet someone with a hello message."""
    return f"Hello, {name}!"

## Combined Options

### Range + Language Override

Include lines 1-3 of JavaScript file with Python syntax highlighting:

```python
function fibonacci(n) {
  if (n <= 1) {
    return n;
```

### Range + Raw Type

Include lines 2-4 of JSON file as raw content:

  "name": "test-project",
  "version": "1.0.0",
  "description": "A test project for include functionality",

### Full Options Combination

Include lines 5-10 of Python file as raw content with language override:

    """Greet someone with a hello message."""
    return f"Hello, {name}!"


def main():
    name = "World"

## Edge Cases

### Empty Range (Should Include Full File)

Test empty range defaults to full file:

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
