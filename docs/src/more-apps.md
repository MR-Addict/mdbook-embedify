# More Apps

In this section, I will show you how to add more apps to this preprocessor.

## Create a new app

You may have some other apps that this preprocessor doesn't support. Actually, it's very easy to add a new app based on my custom template engine.

What we need to do is put a new app template in the `templates` folder. The template file name should be the app name.

For example we want to add a new app called `youtube`, and we know that youtube embedding code is using an iframe. It looks like this:

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
> You can even add `style` and `js` content to the template file. you can take a look of [scroll-to-top](templates/scroll-to-top.html) example.

## Add options

We want the src youtube `id` and `loading` strategy to be dynamic and loading strategy has default `lazy` value. So we can replace them with placeholders like this:

```html
<iframe
  allowfullscreen
  name="youtube"
  loading="{% raw(loading=lazy) %}"
  src="https://www.youtube.com/embed/{% raw(id) %}"
  style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"
  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
></iframe>
```

The placeholder syntax is similar to a **function call** in programming languages.

Which `raw` means the value will not be changed. Now the preprocessor supports two functions, `raw` and `markdown`. `markdown` call will treat the value as markdown content and render it to be html. This call is used in `footer` and `announcement-banner` apps. And inner value will be the `key`, if the key has a `default` value, we put an equal sign and the default value after the key.

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

That's it. If you have some apps need to be added, you can create a new issue or pull request.
