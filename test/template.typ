#import "@preview/ansi-render:0.4.0": *
#import "@preview/xarrow:0.1.1": xarrow as _xarrow

// workaround before set is implemented
#let xarrow = _xarrow.with(margin: .5em)

#let radius = 3pt
#let inset = 8pt

#let code-block(body, lang: "python", count: 0) = {
  block(raw(body, lang: lang), fill: luma(230), inset: inset, radius: radius, width: 100%)
  v(0pt, weak: true)
  show: box.with(height: 0pt)
  move(dx: -2.2em, dy: -1em, text(size: 1em, raw("[" + str(count) + "]:")))
}

#let result-block(body) = {
  v(0pt, weak: true)
  ansi-render(body, radius: radius, inset: inset, width: 100%, size: 9pt)
}

#let block-quote(body) =  style(styles => {
  let ctx = block(fill: luma(240), inset: inset, radius: (right: radius))[#body]
  let size = measure(ctx, styles)
  grid(
    columns: (4pt, auto),
    rows: auto,
    gutter: 0pt,
    rect(fill: luma(180), height: size.height,radius: (left: radius)),
    ctx,
  )
})

// template for the whole document
#let template(doc) = {
  set box(inset: (left: 3pt, right: 3pt), outset: (top: 3pt, bottom: 3pt))
  [#doc]
}
