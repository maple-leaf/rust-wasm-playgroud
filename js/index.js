const loadWASM = import("../pkg/index.js").catch(console.error);

const quantity = 1000;
const runBtn = document.querySelector("#run");
runBtn.addEventListener("click", () => {
  loadWASM.then(m => m.get_json(quantity)).then(console.log);
});

const getResBtn = document.querySelector("#getRes");
getResBtn.addEventListener("click", () => {
  loadWASM
    .then(m => m.get_str(quantity))
    .then(res => JSON.parse(res))
    .then(console.log);
});

const runJsBtn = document.querySelector("#runJs");
runJsBtn.addEventListener("click", () => {
  fetch(`https://fakerapi.it/api/v1/companies?_quantity=${quantity}`)
    .then(res => res.json())
    .then(console.log);
});
