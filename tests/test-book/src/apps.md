# Apps Testing

This page tests various third-party app embeds and local apps supported by mdbook-embedify.

## Third-Party Apps

### Gist

Basic Gist embed:

{% embed gist id="76cf171d1bdd7da41d4ca96b908eb57a" %}

### Vimeo

Basic Vimeo embed with lazy loading:

{% embed vimeo id="914391191" loading="lazy" %}

Vimeo with eager loading:

{% embed vimeo id="914391191" loading="eager" %}

### YouTube

Basic YouTube embed with lazy loading:

{% embed youtube id="DyTCOwB0DVw" loading="lazy" %}

YouTube with eager loading:

{% embed youtube id="DyTCOwB0DVw" loading="eager" %}

### CodePen

CodePen with dark theme and custom height:

{% embed codepen user="MR-Addict" slug="NWBOqKw" height="600" theme="dark" loading="lazy" %}

CodePen with light theme and different height:

{% embed codepen user="MR-Addict" slug="NWBOqKw" height="400" theme="light" loading="eager" %}

### StackBlitz

StackBlitz with light theme:

{% embed stackblitz id="vitejs-vite-y8mdxg" theme="light" loading="lazy" %}

StackBlitz with dark theme:

{% embed stackblitz id="vitejs-vite-y8mdxg" theme="dark" loading="eager" %}

### CodeSandbox

CodeSandbox with light theme:

{% embed codesandbox id="ke8wx" theme="light" loading="lazy" %}

CodeSandbox with dark theme:

{% embed codesandbox id="ke8wx" theme="dark" loading="eager" %}

### Bilibili

Basic Bilibili embed with lazy loading:

{% embed bilibili id="BV1uT4y1P7CX" loading="lazy" %}

Bilibili with eager loading:

{% embed bilibili id="BV1uT4y1P7CX" loading="eager" %}

### Giscus

Giscus comment system with book theme:

{% embed giscus repo="MR-Addict/mdbook-embedify" repo-id="R_kgDOL7sJMw" category="General" category-id="DIC_kwDOL7sJM84CfqRp" theme="book" loading="eager" %}

## Local Apps

### Footer

Footer with markdown support:

{% embed footer message="Copyright © 2025 • Created with ❤️ by [MR-Addict](https://github.com/MR-Addict)" %}

Simple footer:

{% embed footer message="© 2025 Test Book" %}

### Scroll to Top

Scroll to top button:

{% embed scroll-to-top %}

### Announcement Banner

Important announcement with markdown:

{% embed announcement-banner id="test-announcement-1" message="*This is a **test announcement** with [markdown support](https://example.com).*" %}

Simple announcement:

{% embed announcement-banner id="test-announcement-2" message="Welcome to the test book!" %}
