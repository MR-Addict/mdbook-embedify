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
