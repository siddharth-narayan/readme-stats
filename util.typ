#let colors = json("colors.json")
#let lang-color(color) = {
  rgb(colors.at(color).color)
}

#let book-icon = read("book.svg")
#{ 
  book-icon = book-icon.replace("currentColor", "green")
  book-icon = image(bytes(book-icon))
}

