#let lang-colors = json("../assets/lang-colors.json")
#let lang-color(lang) = {
  let color = lang-colors.at(lang).color
  if color == none {
    color = gray
  }

  rgb(color)
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

#let import-image(path, color: theme.text.to-hex()) = {
  let icon = read(path)
  icon = icon.replace("currentColor", color)
  icon = scale(75%, image(bytes(icon)))

  return icon
}