#block[
= h1
<h1>
== h2
<h2>
=== h3
<h3>
==== h4
<h4>
===== h5
<h5>
====== h6
<h6>
#strong[bold] #emph[italic] #strong[#emph[bold italic]] #strike[strike
through]

#block[
quote
]

- list
- list
- list

+ list
+ list
+ list #link("https://www.google.com")[link]
  // #image("https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png")
  `code`
#line(length: 100%)
```
code
block
```

#align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [table], [table], [table],
  [table],
  [table],
  [table],
  [table],
  [table],
  [table],
  [table],
  [table],
  [table],
)
]

]
