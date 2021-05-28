const fs = require("fs")

const entries = JSON.parse(fs.readFileSync("sorted.json"))
const authors = JSON.parse(fs.readFileSync("/local/mullermft/0data/mag/authors.json"))

const author_ids = []
for (const entry of entries) {
  if (entry.authors != null && entry.authors.length > 0) {
    const author_names = []
    for (const author_id of entry.authors) {
      if (authors[author_id] != null) {
        author_names.push(authors[author_id].name)
      }
    }
    entry.author_names = author_names
  }
}

entries.sort((a, b) => {
  a.id - b.id
})

fs.writeFileSync("sorted.json", JSON.stringify(entries, undefined, 2))