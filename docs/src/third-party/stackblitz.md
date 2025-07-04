# Stackblitz

[Stackblitz](https://stackblitz.com) is an instant fullstack web IDE for the JavaScript ecosystem. It's powered by [WebContainers](https://blog.stackblitz.com/posts/introducing-webcontainers), the first WebAssembly-based operating system which boots the Node.js environment in milliseconds, securely within your browser tab.

## Options

| Option  | Description                     | Required | Default |
| :------ | :------------------------------ | :------- | :------ |
| id      | Project ID                      | Yes      | - -     |
| theme   | Supports **light** and **dark** | No       | dark    |
| loading | Supports **lazy** and **eager** | No       | lazy    |

## Example

```text
{% embed-ignore stackblitz id="vitejs-vite-y8mdxg" theme="light" loading="lazy" %}
```

{% embed stackblitz id="vitejs-vite-y8mdxg" theme="light" loading="lazy" %}
