#let colors = json("assets/lang-colors.json")
#let lang-color(color) = {
  rgb(colors.at(color).color)
}

#let book-icon = read("assets/book.svg")
#{ 
  book-icon = book-icon.replace("currentColor", "green")
  book-icon = image(bytes(book-icon))
}


#let star-icon = read("assets/star.svg")
#{
  star-icon = star-icon.replace("currentColor", "white")
  star-icon = scale(75%, image(bytes(star-icon)))
}