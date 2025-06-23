function appendVersion() {
  const shield = document.createElement("span");
  shield.textContent = "0.2.15";
  document.querySelector(".menu-title").appendChild(shield);
}

function main() {
  appendVersion();
}

document.addEventListener("DOMContentLoaded", main);
