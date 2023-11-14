#import "@preview/ansi-render:0.5.0": *
#import "@preview/xarrow:0.1.1": xarrow as _xarrow

// workaround before set is implemented
#let xarrow = _xarrow.with(margin: .5em)

#let radius = 3pt
#let inset = 8pt

#let code-block(body, lang: "python", count: 0) = style(styles => {
  block(raw(body, lang: lang), fill: luma(230), inset: inset, radius: radius, width: 100%)
  v(0pt, weak: true)
  let c = raw("[" + str(count) + "]:")
  let size = measure(c, styles)
  box(height: 0pt, move(dx: -size.width, dy: -size.height - inset, c))
})

#let result-block(body) = {
  v(0pt, weak: true)
  ansi-render(body, radius: radius, inset: inset, width: 100%)
}

#let block-quote(body) =  style(styles => {
  let size = measure(body, styles)
  grid(
    columns: (4pt, auto),
    rows: auto,
    gutter: 0pt,
    rect(fill: luma(180), height: size.height + 2*inset, radius: (left: radius)),
    block(fill: luma(240), height: size.height + 2*inset, inset: inset, radius: (right: radius), width: 100%, body),
  )
})

#let template(body) = {
  set page(height: auto)
  body
}