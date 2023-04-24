const { invoke } = window.__TAURI__.tauri;

async function set(i, n) {
  console.log("over " + i);
  await invoke("set", { index: i, value: n });
}

async function exited(i) {
  await invoke("exited", { index: i });
}

async function reset() {
  await invoke("reset", {});
}

window.addEventListener("DOMContentLoaded", () => {
  console.log("hello from js!");
  document
    .querySelector("body")
    .addEventListener("touchend", () => reset());
  document
    .querySelector("#leftinator")
    .addEventListener("onmouseenter", () => exited(0));
  document
    .querySelector("#rightinator")
    .addEventListener("onmouseenter", () => exited(3));
  for (let i = 0; i < 4; i++) {
    for (let n = 0; n < 4; n++) {
      document
        .querySelector(("#q" + i) + n)
        .addEventListener("onmouseover", () => set(i, n));
    }
  }
});

console.log("hello from js!");