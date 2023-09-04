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
\#\#\#\#\#\# h6

======= h7
<h7>
#strong[bold] #emph[italic] #strong[#emph[bold italic]] #strike[strike
through]

#block[
quote
]

- list1
  - list2
    - list3
      - list4
        - list5
          - list6
            - list7

+ list
+ list
+ list #link("https://www.google.com")[link]
  // #image("https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png")
  `code` ---

Here is a simple footnote#link("https://example.com")[^1]. With some
additional text after it.

```
code
```

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

= Table
<table>
#align(center)[#table(
  columns: 5,
  align: (col, row) => (auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [Syntax test], [Description], [Test], [XXX], [XXX],
  [Header],
  [Title],
  [AAAA],
  [XXX],
  [XXX],
  [Header],
  [Title],
  [AAAA],
  [XXX],
  [XXX],
  [Paragraph],
  [Text],
  [BBBB],
  [XXX],
  [XXX],
)
]

== Pretty table
<pretty-table>
#align(center)[#table(
  columns: 2,
  align: (col, row) => (auto,auto,).at(col),
  inset: 6pt,
  [Header 1], [Header 2],
  // [#image("https://placekitten.com/200/300")],
  // [#image("https://placekitten.com/200/300?")],
)
]

== \#\#\# Lists
<-lists>
- Inline #emph[code] `aaaaz`
- My #strong[favorite search] engine is #strike[not]
  #link("https://duckduckgo.com")[Duck Duck Go].
- #strike[#strong[#emph[Recursive]]]
- #strong[#emph[#strike[Recursive reversed]]]
- Item 4

+ Item

+ Item

+ Item

+ Item

+ c

+ c

==== Multiline code
<multiline-code>
```
int method() {
	return 2137;
}
```

===== Quotes
<quotes>
#block[
aa aa

#block[
bb bb

#block[
cc cc bb bb aa aa
]
]
]

#block[
single line
]

]
