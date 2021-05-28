const fs = require("fs")

const entries_new = JSON.parse(fs.readFileSync("sorted.json"))
const entries_old = JSON.parse(fs.readFileSync("prev_sorted.json"))

console.log(`Prevlen ${entries_old.length} vs ${entries_new.length}`)

for (const old_entry of entries_old) {
  let found_entry = false
  FOUND: for (const new_entry of entries_new) {
    if (old_entry.id == new_entry.id) {
      found_entry = true
      if (old_entry.title != new_entry.title) {
        console.log("TWO ENTRIES BUT DIFFERENT NAME!")
        console.log(old_entry.title)
        console.log(new_entry.title)
      }
      break FOUND
    }
  }
  if (!found_entry) {
    console.log("missing ", old_entry.title)
  }
}