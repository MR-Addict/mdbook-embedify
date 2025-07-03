function appendVersion() {
  const shield = document.createElement("span");
  shield.textContent = "0.2.16";
  document.querySelector(".menu-title").appendChild(shield);
}

function main() {
  appendVersion();
}

document.addEventListener("DOMContentLoaded", main);
