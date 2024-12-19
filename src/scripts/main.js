const { invoke } = window.__TAURI__.core;

async function getCap() {
  invoke("read_from_file", { fileName: "cap1.json" })
    .then((val) => {
      document.querySelector("#title").innerHTML = val.title;
      document.querySelector("#content").innerHTML = val.content;
    })
    .catch((e) => {
      // Improbabile che succeda ma se il fetch da json dà errore, gestire l'errore in caso
      
      console.error(e);
    });
}

window.addEventListener("DOMContentLoaded", () => {
  getCap();
});
