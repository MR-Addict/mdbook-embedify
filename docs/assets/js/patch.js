function appendVersion() {
  const shield = document.createElement("span");
  shield.textContent = "02.17-rc.1";
  document.querySelector(".menu-title").appendChild(shield);
}

function main() {
  appendVersion();
}

document.addEventListener("DOMContentLoaded", main);
