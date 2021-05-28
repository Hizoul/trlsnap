const fs = require("fs")

const entries = JSON.parse(fs.readFileSync("sorted.json"))

const author_ids = []
for (const entry of entries) {
  if (entry.authors != null && entry.authors.length > 0) {
    for (const author_id of entry.authors) {
      if (author_ids.indexOf(author_id) === -1) {
        author_ids.push(author_id)
      }
    }
  }
}
const authors = JSON.parse(fs.readFileSync("/local/mullermft/0data/mag/authors.json"))

const relevant_authors = {}

for (const author_id of author_ids) {
  relevant_authors[author_id] = authors[author_id]
}

fs.writeFileSync("authors.json", JSON.stringify(rele))