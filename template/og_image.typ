#let dark = true;

#let color = (
  bg: if (not dark) {
    color.rgb(247, 244, 254)
  } else {
    color.rgb(17, 15, 23)
  },
  text: if (not dark) {
    color.rgb(77, 69, 106)
  } else {
    color.rgb(227, 218, 250)
  },
  text_header: color.rgb(34, 30, 46),
  gradient_fill: gradient.linear(color.rgb(73.2745%, 56.7312%, 89.8178%), color.rgb(180, 151, 238)),
  gradient_stroke: gradient.linear(color.rgb(56.6422%, 36.7916%, 70.3849%), rgb(113, 96, 162)),
)

#set page(width: 1200pt/2, height: 630pt/2, fill: color.bg, margin: 0pt);

#let data = sys.inputs;
#let title = data.title;
#let subtitle = data.at("subtitle", default: "");
#let datestring = data.datestring;

#place(top + left, pad(8pt*5, [
  #set text(size: 36pt, weight: "medium", fill: color.text, font: "Hepta Slab")
  #title

  #set text(size: 24pt, weight: "regular", fill: color.text, font: "Hepta Slab")
  #subtitle

]))

#place(bottom, dy: 1pt,
  rect(
    width: 100%,
    stroke: (top: 2pt + color.gradient_stroke),
    inset: 8pt*2,
    fill: color.gradient_fill,
    [
        #set text(size: 28pt, weight: "medium", font: "Maple Mono", fill: color.text_header)
        j0.lol
        #h(1fr)
        #set text(size: 28pt, weight: "regular", font: "DM Sans", fill: color.text_header)
        _ Published on #datestring _

    ]
  )
)
