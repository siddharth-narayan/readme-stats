#let colors = json("assets/lang-colors.json")
#let lang-color(color) = {
  rgb(colors.at(color).color)
}

#let themes = (
  github-dark: (
    background: rgb("#0d1117"),
    border: rgb("#3d444d"),
    text: rgb("#f0f6fc"),
    text-unemph: rgb("#f0f6fc").darken(30%)
  ),

  github-light: (
    background: rgb("#ffffff"),
    border: rgb("#d1d9e0"),
    text: rgb("#58636e"),
    text-unemph: rgb("#58636e").lighten(30%)
  )
)

#let theme = themes.at(sys.inputs.theme)

#let book-icon = read("assets/book.svg")
#{ 
  book-icon = book-icon.replace("currentColor", "green")
  book-icon = scale(75%, image(bytes(book-icon)))
}


#let star-icon = read("assets/star.svg")
#{
  star-icon = star-icon.replace("currentColor", theme.text.to-hex())
  star-icon = scale(75%, image(bytes(star-icon)))
}
