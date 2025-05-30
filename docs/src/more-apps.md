# More Apps

In this section, I will show you how to add more apps to this preprocessor.

After reading this, you can open an issue or send a pull request to add a new app to this preprocessor.

## Create a new app

You may have some other apps that preprocessor doesn't support yet. However, it's very easy to add a new app based on project custom template engine.

What we need to do is put a new app template in the **src/assets/templates** folder. The template file name should be the app name ended with **.html**.

For example we want to add a new app called **youtube**, then we could create a **youtube.html** under templates folder.

We know that we can use an iframe to embed a youtube video. Template file could be like this:

```html
<iframe
  allowfullscreen
  name="youtube"
  loading="lazy"
  src="https://www.youtube.com/embed/dQw4w9WgXcQ"
  style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"
  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
></iframe>
```

> 💥Attention
>
> You can even add **css** and **js** content to the template file which should be put inside `style` and `script` blocks.

However, we want video **id** and **loading** strategy to be dynamic and loading strategy has default **lazy** value. So we can replace them with placeholders like this:

```html
<iframe
  allowfullscreen
  name="youtube"
  loading="{% loading=lazy %}"
  src="https://www.youtube.com/embed/{% id %}"
  style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"
  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
></iframe>
```

This way, we can use the **embed** macro to embed a youtube video by passing **id** and **loading** options.

## Template placeholder

The template placeholder is a way to add dynamic values to the template file. It allows you to replace static values with dynamic ones that can be provided by the user when they use the app.

### Syntax

There are two ways of adding dynamic values to the template file:

- Only put key name in the placeholder, like **{% key %}**, and you can add default value after the key name, like **{% key=default %}**. The default value will be used if user doesn't provide the value.
- Wrapped with preprocessor name, like **{% processor(key=default) %}**. The processor name is like as function name, it will be used to process the inner value and replace the placeholder.

**Placeholder**

The inner value is key follwed by a default value in the form of **key=default**. If the key is not provided, the default value will be used.

**Preprocessor**

Now only **markdown** is supported, markdown will treat the inner value as markdown content and render it to be html.

### Examples

- **id** means the placeholder will be replaced by the value of **id** key and id is not optional because it doesn't have a default value.
- **loading=lazy** means the placeholder will be replaced by the value of **loading** key. If user doesn't provide the value, the default value **lazy** will be used.
- **markdown(message)** means the placeholder will be replaced by the value of **message** processed by **markdown** processor.

## Build the project

After build the project, the new app will be automatically added to the preprocessor binary.

```sh
cargo build --release
```

And you can use it in your book:

<!-- embed ignore begin -->

```text
{% embed youtube id="dQw4w9WgXcQ" loading="eager" %}
```

Because the `loading` key has a default value `lazy`, we can omit it.

```text
{% embed youtube id="dQw4w9WgXcQ" %}
```

<!-- embed ignore end -->

If you are using this repository as a template, then you can build your book to see the result by running:

```sh
mdbook build docs
```

That's it.
