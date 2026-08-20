#import "util.typ": *

#set page(width: 300pt, height: auto, margin: 1pt, fill: color.rgb(255, 255, 255, 0))
#set text(fill: theme.text, font: "Noto Sans")

#block(width: 100%, inset: 12.5pt, fill: theme.background, stroke: theme.border + 1pt, radius: 5pt)[
  // Title
  #stack(dir: ltr, spacing: 5pt,
    import-image("../assets/book.svg", "green"),
    align(horizon, strong(sys.inputs.repo-name)))

  #set text(fill: theme.text-unemph, size: 10pt)

  // Description
  #sys.inputs.repo-desc

  // Language
  #align(bottom,
    stack(dir: ltr, spacing: 10pt,
        align(horizon,
        stack(dir: ltr, spacing: 5pt,
          circle(radius: 5pt, fill: lang-color(sys.inputs.repo-lang)),
          sys.inputs.repo-lang,
        )),
        align(horizon,
        if sys.inputs.repo-stars > 0 {
          stack(dir: ltr, spacing: 3pt,
            import-image("../assets/star.svg"),
            align(horizon, [#sys.inputs.repo-stars])
          )
        })
    )
  )
]

// #set text(size: 5pt)
// Debug info:
// - `repo-name`: #sys.inputs.repo-name
// - `repo-desc`: #sys.inputs.repo-desc
// - `repo-lang`: #sys.inputs.repo-lang
// - `repo-stars`: #sys.inputs.repo-stars