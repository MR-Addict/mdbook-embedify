# Template System

> Attention 💥
>
> Support since [v0.2.14](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.14).

The template system allows you to create custom embed applications that aren't supported by the built-in preprocessor. This powerful feature enables you to extend mdbook-embedify with your own interactive components and dynamic content.

> Real world example 💡
>
> You can take a looke of my [notes](https://mr-addict.github.io/notes/life/studio/disney/index.html) book.
>
> It supports **caption** and **preview** with much more complex behavior.

## Dynamic vs Static Content

Custom templates act like dynamic reusable components. If you want to just copy static content, you should use the [include app](../local/include.md) instead.

## Creating a Custom App

### Template Folder Setup

Templates must be placed in the **assets/templates** folder (relative to your `book.toml` file).

**Configure the template folder** (optional):

```toml
[preprocessor.embedify]
custom-templates-folder = "assets/templates"
```

> 📍 **Path Requirements**:
>
> The template folder path must be relative to the book root directory (where `book.toml` is located).

### Template File Naming

The template filename becomes the app name:

<!-- embed ignore begin -->

| Template File    | App Name    | Usage                                      |
| ---------------- | ----------- | ------------------------------------------ |
| `canvas.html`    | `canvas`    | `{% embed canvas %}`                       |
| `my-widget.html` | `my-widget` | `{% embed my-widget %}`                    |
| `youtube.html`   | `youtube`   | `{% embed youtube %}` (overrides built-in) |

<!-- embed ignore end -->

> ⚠️ **Override Behavior**:
>
> If your custom app name matches a built-in app, the custom template will **override** the built-in functionality.

### Example: Creating a Canvas App

Let's create a **canvas** app - a simple drawable canvas component.

**Step 1: Create the template file** (`assets/templates/canvas.html`)

**Basic HTML structure with styling:**

```html
<div class="canvas-container">
  <canvas height="400"></canvas>
</div>
{% embed include file="assets/templates/canvas.html" range="4-15" type="raw" %}
```

**Add interactive JavaScript:**

{% embed include file="assets/templates/canvas.html" range="16-" %}

> **CSS & JavaScript Integration** 💡
>
> You can include **CSS** and **JavaScript** directly in template files using `<style>` and `<script>` blocks.

**Step 2: Add dynamic height support**

Make the canvas height configurable using placeholder syntax:

```html
<canvas height="{% height=400 %}"></canvas>
```

<!-- embed ignore begin -->

This allows users to customize the height: `{% embed canvas height=600 %}`

<!-- embed ignore end -->

## Placeholder Syntax

The template system supports two types of dynamic content:

### Syntax Types

| Type            | Syntax                         | Purpose                | Example                   |
| --------------- | ------------------------------ | ---------------------- | ------------------------- |
| **Placeholder** | `{% key=default %}`            | Variable substitution  | `{% height=400 %}`        |
| **Processor**   | `{% processor(key=default) %}` | Content transformation | `{% markdown(content) %}` |

### Placeholder Examples

| Syntax                    | Behavior                                         | Use Case                  |
| ------------------------- | ------------------------------------------------ | ------------------------- |
| `{% height %}`            | **Required** - User must provide value           | Mandatory configuration   |
| `{% height=400 %}`        | **Optional** - Uses default if not provided      | Optional configuration    |
| `{% markdown(message) %}` | **Processed** - Content transformed by processor | Dynamic content rendering |

### Available Processors

| Processor  | Purpose                  | Input         | Output       |
| ---------- | ------------------------ | ------------- | ------------ |
| `markdown` | Renders markdown to HTML | Markdown text | HTML content |

**Example usage:**

```html
<div class="content">{% markdown(description="Description in **markdown**") %}</div>
```

## Complete Canvas Template

Here's the complete template file for our canvas app:

{% embed include file="assets/templates/canvas.html" %}

## Using Your Custom App

<!-- embed ignore begin -->

### Basic Usage

After creating the template file, use your app in your book:

```text
{% embed canvas height=400 %}
```

### With Default Values

Since height has a default value of **400**, you can omit it:

```text
{% embed canvas %}
```

<!-- embed ignore end -->

### Interactive Example

Test the canvas app by drawing on it:

{% embed canvas %}

## Contributing Templates

Want to share your custom templates with the community?

### For Personal Use

- Keep templates in your `assets/templates/` folder
- Customize as needed for your specific use case

### For Community Contribution

- **Fork** the [mdbook-embedify repository](https://github.com/mr-addict/mdbook-embedify)
- **Add** your template to [src/assets/templates](https://github.com/MR-Addict/mdbook-embedify/tree/main/src/assets/templates)
- **Submit** a pull request with documentation
- **Include** examples and usage instructions
