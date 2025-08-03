// Direct JSX file, no framework — runs with your runtime
import { signal, effect } from "../runtime/signals.js"
import { insert, createElement, setProp } from "../runtime/dom.js"

// Core signals
const todos = signal([])
const filter = signal("all")
const input = signal("")

// Add 10,000 todos initially
for (let i = 0; i < 10000; i++) {
  todos.value.push({ id: i, text: `Task #${i}`, done: false })
}

// Reactive rendering
effect(() => {
  const root = document.getElementById("app")
  root.innerHTML = ""

  const visible = todos.value.filter(todo => {
    if (filter.value === "done") return todo.done
    if (filter.value === "active") return !todo.done
    return true
  })

  for (const todo of visible) {
    const div = createElement("div")
    div.className = "todo"
    div.style = `padding:2px;border-bottom:1px solid #eee`
    div.onclick = () => {
      todo.done = !todo.done
      todos.value = [...todos.value] // trigger update
    }
    div.textContent = `${todo.done ? "✅" : "⬜️"} ${todo.text}`
    insert(root, div)
  }
})

// Input UI
window.onload = () => {
  const inputBox = document.getElementById("input")
  const filterAll = document.getElementById("filter-all")
  const filterDone = document.getElementById("filter-done")
  const filterActive = document.getElementById("filter-active")
  const addButton = document.getElementById("add")

  inputBox.oninput = (e) => input.value = e.target.value
  addButton.onclick = () => {
    if (!input.value.trim()) return
    todos.value = [
      ...todos.value,
      { id: todos.value.length, text: input.value, done: false }
    ]
    input.value = ""
    inputBox.value = ""
  }

  filterAll.onclick = () => filter.value = "all"
  filterDone.onclick = () => filter.value = "done"
  filterActive.onclick = () => filter.value = "active"
}
