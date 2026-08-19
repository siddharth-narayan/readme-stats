#import "util.typ": book-icon, lang-color

#set page(width: 300pt, height: 125pt, fill: color.rgb(255, 255, 255, 0), margin: 20pt)
#set text(fill: white, font: "Noto Sans")

// Title
#stack(dir: ltr, spacing: 7.5pt,
  book-icon,
  align(horizon, text(strong(sys.inputs.repo-name), size: 14pt))
)

// Description
#text(sys.inputs.repo-desc, fill: gray.darken(30%))

// Language
#align(bottom, 
  stack(dir: ltr, spacing: 7.5pt,
    circle(radius: 5pt, fill: lang-color(sys.inputs.repo-lang)),
    text(fill: gray.darken(30%), sys.inputs.repo-lang)
  )
)