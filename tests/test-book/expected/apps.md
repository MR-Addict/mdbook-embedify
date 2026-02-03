# Apps Testing

This page tests various third-party app embeds and local apps supported by mdbook-embedify.

## Third-Party Apps

### Gist

Basic Gist embed:

<!-- {% embed gist id="76cf171d1bdd7da41d4ca96b908eb57a" %} -->

<div data-embedify data-app="gist" data-option-id="76cf171d1bdd7da41d4ca96b908eb57a" style="display:none"></div>

<script src="https://gist.github.com/76cf171d1bdd7da41d4ca96b908eb57a.js"></script>

### Vimeo

Basic Vimeo embed with lazy loading:

<!-- {% embed vimeo id="914391191" loading="lazy" %} -->

<div data-embedify data-app="vimeo" data-option-id="914391191" data-option-loading="lazy" style="display:none"></div>

<iframe name="vimeo" loading="lazy" allow="autoplay; fullscreen; picture-in-picture" src="https://player.vimeo.com/video/914391191" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"></iframe>

Vimeo with eager loading:

<!-- {% embed vimeo id="914391191" loading="eager" %} -->

<div data-embedify data-app="vimeo" data-option-id="914391191" data-option-loading="eager" style="display:none"></div>

<iframe name="vimeo" loading="eager" allow="autoplay; fullscreen; picture-in-picture" src="https://player.vimeo.com/video/914391191" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"></iframe>

### YouTube

Basic YouTube embed with lazy loading:

<!-- {% embed youtube id="DyTCOwB0DVw" loading="lazy" %} -->

<div data-embedify data-app="youtube" data-option-id="DyTCOwB0DVw" data-option-loading="lazy" style="display:none"></div>

<iframe allowfullscreen name="youtube" loading="lazy" src="https://www.youtube.com/embed/DyTCOwB0DVw" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"></iframe>

YouTube with eager loading:

<!-- {% embed youtube id="DyTCOwB0DVw" loading="eager" %} -->

<div data-embedify data-app="youtube" data-option-id="DyTCOwB0DVw" data-option-loading="eager" style="display:none"></div>

<iframe allowfullscreen name="youtube" loading="eager" src="https://www.youtube.com/embed/DyTCOwB0DVw" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"></iframe>

### CodePen

CodePen with dark theme and custom height:

<!-- {% embed codepen user="MR-Addict" slug="NWBOqKw" height="600" theme="dark" loading="lazy" %} -->

<div data-embedify data-app="codepen" data-option-user="MR-Addict" data-option-slug="NWBOqKw" data-option-height="600" data-option-theme="dark" data-option-loading="lazy" style="display:none"></div>

<iframe allowfullscreen name="codepen" loading="lazy" height="600" style="width: 100%; border: none; border-radius: 1rem; background: black" src="https://codepen.io/MR-Addict/embed/NWBOqKw?default-tab=result&theme-id=dark"></iframe>

CodePen with light theme and different height:

<!-- {% embed codepen user="MR-Addict" slug="NWBOqKw" height="400" theme="light" loading="eager" %} -->

<div data-embedify data-app="codepen" data-option-user="MR-Addict" data-option-slug="NWBOqKw" data-option-height="400" data-option-theme="light" data-option-loading="eager" style="display:none"></div>

<iframe allowfullscreen name="codepen" loading="eager" height="400" style="width: 100%; border: none; border-radius: 1rem; background: black" src="https://codepen.io/MR-Addict/embed/NWBOqKw?default-tab=result&theme-id=light"></iframe>

### StackBlitz

StackBlitz with light theme:

<!-- {% embed stackblitz id="vitejs-vite-y8mdxg" theme="light" loading="lazy" %} -->

<div data-embedify data-app="stackblitz" data-option-id="vitejs-vite-y8mdxg" data-option-theme="light" data-option-loading="lazy" style="display:none"></div>

<iframe name="stackblitz" loading="lazy" src="https://stackblitz.com/edit/vitejs-vite-y8mdxg?embed=1&theme=light&view=preview" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"></iframe>

StackBlitz with dark theme:

<!-- {% embed stackblitz id="vitejs-vite-y8mdxg" theme="dark" loading="eager" %} -->

<div data-embedify data-app="stackblitz" data-option-id="vitejs-vite-y8mdxg" data-option-theme="dark" data-option-loading="eager" style="display:none"></div>

<iframe name="stackblitz" loading="eager" src="https://stackblitz.com/edit/vitejs-vite-y8mdxg?embed=1&theme=dark&view=preview" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"></iframe>

### CodeSandbox

CodeSandbox with light theme:

<!-- {% embed codesandbox id="ke8wx" theme="light" loading="lazy" %} -->

<div data-embedify data-app="codesandbox" data-option-id="ke8wx" data-option-theme="light" data-option-loading="lazy" style="display:none"></div>

<iframe name="sandbox" loading="lazy" src="https://codesandbox.io/embed/ke8wx?&theme=light" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"></iframe>

CodeSandbox with dark theme:

<!-- {% embed codesandbox id="ke8wx" theme="dark" loading="eager" %} -->

<div data-embedify data-app="codesandbox" data-option-id="ke8wx" data-option-theme="dark" data-option-loading="eager" style="display:none"></div>

<iframe name="sandbox" loading="eager" src="https://codesandbox.io/embed/ke8wx?&theme=dark" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"></iframe>

### Bilibili

Basic Bilibili embed with lazy loading:

<!-- {% embed bilibili id="BV1uT4y1P7CX" loading="lazy" %} -->

<div data-embedify data-app="bilibili" data-option-id="BV1uT4y1P7CX" data-option-loading="lazy" style="display:none"></div>

<iframe allowfullscreen name="bilibili" loading="lazy" src="https://player.bilibili.com/player.html?bvid=BV1uT4y1P7CX&danmaku=0" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"></iframe>

Bilibili with eager loading:

<!-- {% embed bilibili id="BV1uT4y1P7CX" loading="eager" %} -->

<div data-embedify data-app="bilibili" data-option-id="BV1uT4y1P7CX" data-option-loading="eager" style="display:none"></div>

<iframe allowfullscreen name="bilibili" loading="eager" src="https://player.bilibili.com/player.html?bvid=BV1uT4y1P7CX&danmaku=0" style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"></iframe>

### Giscus

Giscus comment system with book theme:

<!-- {% embed giscus repo="MR-Addict/mdbook-embedify" repo-id="R_kgDOL7sJMw" category="General" category-id="DIC_kwDOL7sJM84CfqRp" theme="book" loading="eager" %} -->

<div data-embedify data-app="giscus" data-option-repo="MR-Addict/mdbook-embedify" data-option-repo-id="R_kgDOL7sJMw" data-option-category="General" data-option-category-id="DIC_kwDOL7sJM84CfqRp" data-option-theme="book" data-option-loading="eager" style="display:none"></div>

<style>.giscus { margin-top: 6rem; }</style><script src="https://giscus.app/client.js" data-repo="MR-Addict/mdbook-embedify" data-repo-id="R_kgDOL7sJMw" data-category="General" data-category-id="DIC_kwDOL7sJM84CfqRp" data-mapping="title" data-strict="0" data-reactions-enabled="1" data-emit-metadata="0" data-input-position="bottom" data-lang="en" data-loading="eager" crossorigin="anonymous" data-theme="book" async></script><script>(() => { const giscusScript = document.querySelector("script[data-repo][data-repo-id]"); if (giscusScript?.getAttribute("data-theme") !== "book") return; const mapTheme = (theme) => (["light", "rust"].includes(theme) ? "light" : "dark"); const bookTheme = localStorage.getItem("mdbook-theme") || html.getAttribute("class"); giscusScript.setAttribute("data-theme", mapTheme(bookTheme)); document.querySelectorAll("button[role='menuitem'].theme").forEach((btn) => { btn.addEventListener("click", (event) => { const theme = mapTheme(event.target.id); const iframe = document.querySelector("iframe.giscus-frame"); if (iframe) iframe.contentWindow.postMessage({ giscus: { setConfig: { theme } } }, "*"); }); }); })();</script>

## Local Apps

### Footer

Footer with markdown support:

<!-- {% embed footer message="Copyright © 2025 • Created with ❤️ by [MR-Addict](https://github.com/MR-Addict)" %} -->

<div data-embedify data-app="footer" data-option-message="Copyright © 2025 • Created with ❤️ by [MR-Addict](https://github.com/MR-Addict)" style="display:none"></div>

<style>footer { text-align: center; text-wrap: balance; margin-top: 5rem; display: flex; flex-direction: column; justify-content: center; align-items: center; } footer p { margin: 0; }</style><footer><p>Copyright © 2025 • Created with ❤️ by <a href="https://github.com/MR-Addict">MR-Addict</a></p></footer>

Simple footer:

<!-- {% embed footer message="© 2025 Test Book" %} -->

<div data-embedify data-app="footer" data-option-message="© 2025 Test Book" style="display:none"></div>

<style>footer { text-align: center; text-wrap: balance; margin-top: 5rem; display: flex; flex-direction: column; justify-content: center; align-items: center; } footer p { margin: 0; }</style><footer><p>© 2025 Test Book</p></footer>

### Scroll to Top

Scroll to top button:

<!-- {% embed scroll-to-top %} -->

<div data-embedify data-app="scroll-to-top"  style="display:none"></div>

<style>.scroll-to-top { font-size: 2.5rem; width: 3.2rem; height: 3.2rem; display: none; align-items: center; justify-content: center; position: fixed; padding: 0.75rem; bottom: 4rem; right: calc(1.25rem + 90px + var(--page-padding)); z-index: 999; cursor: pointer; border: none; color: var(--bg); background: var(--fg); border-radius: 50%; } .scroll-to-top.hidden { display: none; } .scroll-to-top i { transform: translateY(-2px); } @media (min-width: 1080px) { .scroll-to-top { display: flex; } }</style><button type="button" aria-label="scroll-to-top" class="scroll-to-top hidden" onclick="scrollToTop()"> <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg"> <path d="m18 15-6-6-6 6"></path> </svg></button><script>const scrollToTop = () => window.scroll({ top: 0, behavior: "smooth" }); window.addEventListener("scroll", () => { const button = document.querySelector(".scroll-to-top"); button.classList.toggle("hidden", window.scrollY <200); });</script>

### Announcement Banner

Important announcement with markdown:

<!-- {% embed announcement-banner id="test-announcement-1" message="*This is a **test announcement** with [markdown support](https://example.com).*" %} -->

<div data-embedify data-app="announcement-banner" data-option-id="test-announcement-1" data-option-message="*This is a **test announcement** with [markdown support](https://example.com).*" style="display:none"></div>

<style>.announcement-banner { --site-announcement-bar-stripe-color1: #e5e7eb; --site-announcement-bar-stripe-color2: #d1d5db; z-index: 150; position: relative; flex-direction: column; justify-content: center; align-items: center; margin: 0; padding: 1rem 3.5rem; background: repeating-linear-gradient( 45deg, var(--site-announcement-bar-stripe-color1), var(--site-announcement-bar-stripe-color1) 20px, var(--site-announcement-bar-stripe-color2) 10px, var(--site-announcement-bar-stripe-color2) 40px ); } html:is(.navy, .coal, .ayu) .announcement-banner { --site-announcement-bar-stripe-color1: #1f2937; --site-announcement-bar-stripe-color2: #111827; } .announcement-banner p { color: var(--fg); width: 100%; margin: 0; padding: 0; overflow: hidden; text-align: center; white-space: nowrap; text-overflow: ellipsis; text-wrap: balance; } .announcement-banner button[data-close] { top: 50%; right: 1rem; position: absolute; transform: translateY(-50%); width: 3rem; height: 3rem; cursor: pointer !important; border: none; font-weight: 900; border-radius: 50%; background-color: transparent; }</style><div style="display: none" data-id="test-announcement-1" class="announcement-banner"> <p><em>This is a <strong>test announcement</strong> with <a href="https://example.com">markdown support</a>.</em></p> <button type="button" data-close>X</button></div><script>(() => { const banner = document.querySelector(".announcement-banner"); const id = banner.getAttribute("data-id"); const message = banner.querySelector("p").textContent; const localData = JSON.parse(localStorage.getItem("mdbook-announcement-banner")); if (!localData || localData.id !== id || localData.hide !== true) { banner.style.display = "flex"; const page = document.querySelector(".page"); page.parentNode.insertBefore(banner, page); banner.querySelector("button").addEventListener("click", () => { banner.remove(); localStorage.setItem("mdbook-announcement-banner", JSON.stringify({ id, hide: true, message })); }); } })();</script>

Simple announcement:

<!-- {% embed announcement-banner id="test-announcement-2" message="Welcome to the test book!" %} -->

<div data-embedify data-app="announcement-banner" data-option-id="test-announcement-2" data-option-message="Welcome to the test book!" style="display:none"></div>

<style>.announcement-banner { --site-announcement-bar-stripe-color1: #e5e7eb; --site-announcement-bar-stripe-color2: #d1d5db; z-index: 150; position: relative; flex-direction: column; justify-content: center; align-items: center; margin: 0; padding: 1rem 3.5rem; background: repeating-linear-gradient( 45deg, var(--site-announcement-bar-stripe-color1), var(--site-announcement-bar-stripe-color1) 20px, var(--site-announcement-bar-stripe-color2) 10px, var(--site-announcement-bar-stripe-color2) 40px ); } html:is(.navy, .coal, .ayu) .announcement-banner { --site-announcement-bar-stripe-color1: #1f2937; --site-announcement-bar-stripe-color2: #111827; } .announcement-banner p { color: var(--fg); width: 100%; margin: 0; padding: 0; overflow: hidden; text-align: center; white-space: nowrap; text-overflow: ellipsis; text-wrap: balance; } .announcement-banner button[data-close] { top: 50%; right: 1rem; position: absolute; transform: translateY(-50%); width: 3rem; height: 3rem; cursor: pointer !important; border: none; font-weight: 900; border-radius: 50%; background-color: transparent; }</style><div style="display: none" data-id="test-announcement-2" class="announcement-banner"> <p>Welcome to the test book!</p> <button type="button" data-close>X</button></div><script>(() => { const banner = document.querySelector(".announcement-banner"); const id = banner.getAttribute("data-id"); const message = banner.querySelector("p").textContent; const localData = JSON.parse(localStorage.getItem("mdbook-announcement-banner")); if (!localData || localData.id !== id || localData.hide !== true) { banner.style.display = "flex"; const page = document.querySelector(".page"); page.parentNode.insertBefore(banner, page); banner.querySelector("button").addEventListener("click", () => { banner.remove(); localStorage.setItem("mdbook-announcement-banner", JSON.stringify({ id, hide: true, message })); }); } })();</script>
