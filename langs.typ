#import "util.typ": *

#stack(dir: ltr,
  for (lang-name, width) in sys.inputs.languages {
    box(width: width, height: 1em, fill: lang-color(lang-name))
  }
)