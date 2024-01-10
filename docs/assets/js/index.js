const apps = [
  {
    name: "youtube",
    style: "aspect-ratio: 16/9",
    url: "https://www.youtube.com/embed/EngW7tLk6R8",
  },
  {
    name: "bilibili",
    style: "aspect-ratio: 16/9",
    url: "https://player.bilibili.com/player.html?bvid=BV1uT4y1P7CX&danmaku=0",
  },
  {
    name: "codesandbox",
    style: "aspect-ratio: 16/9",
    url: "https://codesandbox.io/embed/ke8wx?&theme=dark",
  },
  {
    name: "stackblitz",
    style: "aspect-ratio: 16/9",
    url: "https://stackblitz.com/edit/vitejs-vite-y8mdxg?embed=1&theme=dark&view=preview",
  },
  {
    name: "codepen",
    style: "height: 600px;",
    url: "https://codepen.io/MR-Addict/embed/NWBOqKw?default-tab=result&theme-id=dark",
  },
];

const buttons = document.querySelectorAll(".app button");
buttons.forEach((button) => {
  button.addEventListener("click", () => {
    const container = button.parentNode;

    if (container.getAttribute("data-third-part") === null) container.querySelector("div").style = "";
    else {
      // get app
      const app = apps.find((app) => app.name === container.getAttribute("data-app"));
      // create iframe
      const iframe = document.createElement("iframe");
      iframe.src = app.url;
      iframe.allowFullscreen = true;
      iframe.style = app.style;
      iframe.classList.add("window");
      // append iframe
      container.appendChild(iframe);
    }
    // remove button
    button.remove();
  });
});
