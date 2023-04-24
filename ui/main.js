const { invoke } = window.__TAURI__.tauri;

async function set(i, n) {
  await invoke("set", { index: i, value: n });
}

async function exited(i) {
  await invoke("exited", { index: i });
}

async function reset() {
  await invoke("reset", {});
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("body")
    .addEventListener("touchend", () => reset());
  document
    .querySelector("#leftinator")
    .addEventListener("mouseenter", () => exited(0));
  document
    .querySelector("#rightinator")
    .addEventListener("mouseenter", () => exited(3));
  for (let i = 0; i < 4; i++) {
    for (let n = 0; n < 4; n++) {
      let q = ("#q" + i) + n;
      document
        .querySelector(q)
        .addEventListener("mouseover", () => set(i, n));
    }
  }
});
