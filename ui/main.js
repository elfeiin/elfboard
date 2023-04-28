const { invoke } = window.__TAURI__.tauri;

async function entered_key(n) {
  await invoke("entered_key", { column: Math.floor(n / 4), row: n % 4 });
}

async function entered_end() {
  await invoke("entered_end", {});
}

async function lifted() {
  await invoke("lifted", {});
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("body")
    .addEventListener("touchend", () => lifted());
  for (let i = 0; i < 16; i++) {
    let q = "#k" + i;
    document
      .querySelector(q)
      .addEventListener("mouseover", () => entered_key(i));
  }
  document.querySelector("#left")
    .addEventListener("mouseover", () => entered_end());
  document.querySelector("#right")
    .addEventListener("mouseover", () => entered_end());
});
