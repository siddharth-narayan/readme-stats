#import "util.typ": *



#import "util.typ": *

#set page(width: 300pt, height: 125pt, margin: 1pt, fill: color.rgb(255, 255, 255, 0))
#set text(fill: theme.text, font: "Noto Sans")

#block(width: 100%, inset: 12.5pt, fill: theme.background, stroke: theme.border + 1pt, radius: 5pt)[
  // Title
  #strong("Most Used Languages")

  #set text(fill: theme.text-unemph, size: 10pt)

  #stack(dir: ltr,
    for (lang-name, width) in sys.inputs.languages {
      box(width: width, height: 1em, fill: lang-color(lang-name))
    }
  )

  #columns(
    2,
    for (lang-name, width) in sys.inputs.languages {
      stack(dir: ltr, spacing: 5pt,
        circle(radius: 5pt, fill: lang-color(lang-name)),
        lang-name,
        str(calc.round(width / 1%, digits: 2)) + "%"
      )
    }
  )
]