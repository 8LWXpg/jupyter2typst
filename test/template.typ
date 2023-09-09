#import "@preview/ansi-render:0.3.0": *

#let template(doc) = {
  show raw.where(block: true): block.with(
    fill: luma(230),
    inset: 8pt,
    radius: 3pt,
  )

  let block-quote(body) = {

  }

  [#doc]
}
