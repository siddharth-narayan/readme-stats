#import "util.typ": *

#set page(width: 275pt, height: auto, margin: 1pt, fill: color.rgb(255, 255, 255, 0))
#set text(fill: theme.text, font: "Noto Sans")

#block(width: 100%, inset: 12.5pt, fill: theme.background, stroke: theme.border + 1pt, radius: 5pt)[
  #pad(left: 7.5pt, top: 5pt, heading(depth: 2, sys.inputs.name + "'s GitHub Stats"))

  // Title
  #table(
    stroke: none,
    columns: (25pt, auto, 100pt),
    align: left + horizon,
    import-image("../assets/star.svg", color: "#8e6a1b"), "Total Stars:",          align(right, str(sys.inputs.star-count)),
    import-image("../assets/history.svg", color: "#8e6a1b"), "Total Commits:",     align(right, str(sys.inputs.commits)),
    import-image("../assets/pull.svg", color: "#8e6a1b"), "Total Pull Requests:",  align(right, str(sys.inputs.pull-requests)),
    import-image("../assets/issue.svg", color: "#8e6a1b"), "Total Issues:",        align(right, str(sys.inputs.issues)),
    import-image("../assets/book.svg", color: "#8e6a1b"), "Contributed to:",       align(right, str(sys.inputs.repo-contributions)),
  )
]
