# More Apps

> Attention 💥
>
> Support since [v0.2.14](https://github.com/MR-Addict/mdbook-embedify/releases/tag/0.2.14).

> Good to know 💡
>
> Custom templates acts like dynamic reusable components. If you want to just copy static content, you should use the [include app](local/include.md) instead.

You may have some other apps that preprocessor doesn't support yet. However, it's very easy to add a new app based on this project custom template engine.

In this section, I will show you how to add custom app to this preprocessor.

## Create a new app

### Template folder

First we need to put a new app template in the **assets/templates** folder (which is relative to `book.toml` file).

You can change the template folder path by setting `custom-templates-folder` value in the preprocessor section. The default value is `assets/templates`.

```toml
[preprocessor.embedify]
custom-templates-folder = "assets/templates"
```

The template folder path shoulde be relative to the book root directory, which is the directory where the `book.toml` file is located.

### Template file

Now let's create a new app called **canvas**. Which is a simple drawable canvas app.

The template file name will be the app name. For example, we want to add a new app called **canvas**, then we should create a **canvas.html** under templates folder.

If your custom app name is the same as the built-in app name, the custom app will **override** the built-in app while rendering.

First we add some basic html structure and some styles to the `canvas.html` file:

```html
<div class="canvas-container">
  <canvas height="400"></canvas>
</div>
{% embed include file="assets/templates/canvas.html" range="4-15" type='raw' %}
```

And then add some js code to make it drawable:

{% embed include file="assets/templates/canvas.html" range="16-" %}

> Good to know 💡
>
> You can add **css** and **js** content to the template file which should be put inside `style` and `script` blocks.

However, we want to the canvas height to be dynamic. We can do this by using placeholder syntax:

```html
<canvas height="{% height=400 %}"></canvas>
```

Which means the height of the canvas will be replaced by the value of **height** key. If user doesn't provide the value, the default value **400** will be used.

## Placeholder syntax

### Syntax

There are two ways of adding dynamic values to the template file:

- Put key name in the placeholder, like **{% key %}**, and you can add default value after the key name, like **{% key=default %}**. The default value will be used if user doesn't provide the value.
- Wrapped with preprocessor name, like **{% processor(key=default) %}**. The processor name acts like function name, it will be used to process the inner value and replace the placeholder.

**Placeholder**

The inner value is key follwed by a default value in the form of **key=default**. If the key is not provided, the default value will be used.

**Preprocessor**

Now only **markdown** is supported, markdown will treat the inner value as markdown content and render it to be html.

### Examples

- **{% height %}** means the placeholder will be replaced by the value of **height** key and height is not optional because it doesn't have a default value.
- **{% height=400 %}** means the placeholder will be replaced by the value of **height** key. If user doesn't provide the value, the default value **400** will be used.
- **{% markdown(message) %}** means the placeholder will be replaced by the value of **message** processed by **markdown** processor.

## Final template file

Here is the final template file for the **canvas** app:

{% embed include file="assets/templates/canvas.html" %}

## Use the new app

After creating the template file, we can use the new app in our book:

<!-- embed ignore begin -->

```text
{% embed canvas height=400 %}
```

Because the height has default value of **400**, we can omit it:

```text
{% embed canvas %}
```

<!-- embed ignore end -->

Test canvas app by drawing something on it:

{% embed canvas %}

## Conclusion

That's it.

You can also use the same method to add your own custom apps to this preprocessor. Just **clone** this repository and add your own app template to the [src/assets/templates](https://github.com/MR-Addict/mdbook-embedify/tree/main/src/assets/templates) folder.

Welcome to contribute to this project by adding more apps. If you have any questions or suggestions, feel free to open an issue or pull request on the [GitHub repository](https://github.com/mr-addict/mdbook-embedify).
