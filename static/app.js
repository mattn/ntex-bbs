window.addEventListener("DOMContentLoaded", () => {
  const input = document.querySelector("#body");
  const list = document.querySelector("#list")

  const update = () => {
    fetch("/entries", {
      method: "GET"
    })
    .then((resp) => resp.json())
    .then((json) => {
      list.innerHTML = '';
      for (const entry of json.entries) {
        const li = document.createElement("li")
        li.textContent = entry.body
        list.appendChild(li)
      }
      input.value = "";
    })
  }
  input.addEventListener("keydown", (e) => {
    if (e.key === 'Enter') {
      const body = 
      fetch("/entries", {
        method: "POST",
        headers: { "Content-Type": "application/json", },
        body: JSON.stringify({"body": input.value}),
      })
      .then((resp) => update())
    }
  })
  update();
}, null)
