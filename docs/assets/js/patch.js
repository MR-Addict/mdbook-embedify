function appendVersion() {
  const shield = document.createElement("span");
  shield.textContent = "0.2.17-rc.5";
  document.querySelector(".menu-title").appendChild(shield);
}

function main() {
  appendVersion();
}

document.addEventListener("DOMContentLoaded", main);
