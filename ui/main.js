const { invoke } = window.__TAURI__.tauri;

async function entered_key(n) {
  await invoke("entered_key", { column: Math.floor(n / 4), row: n % 4 });
}

async function entered_end() {
  await invoke("entered_end", {});
}

async function lifted() {
  await invoke("lifted", {});
  document.querySelectorAll(".shine").forEach((e) => {
    e.classList.remove("shine");
  });
}

async function swipe_hover(e) {
  document.querySelectorAll(".shine").forEach((e) => {
    e.classList.remove("shine");
  });
  let cx = e.touches[0].clientX;
  let cy = e.touches[0].clientY;
  let e = document.elementFromPoint(cx,cy);
  if (e.classList.contains("key")) {
    e.classList.add("shine");
  }
  // if (e.key_num != null) {
  //   entered_key(e.key_num);
  // } else if (e.id == "left" || e.id == "right") {
  //   entered_end();
  // }
}

window.addEventListener("DOMContentLoaded", () => {
  let body = document
    .querySelector("body");
  body.addEventListener("touchmove", swipe_hover);
  body.addEventListener("touchend", () => lifted());
  for (let i = 0; i < 16; i++) {
    let q = "#k" + i;
    // document.querySelector(q).key_num = i;
  }
});
