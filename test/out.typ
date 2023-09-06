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

\#\#\#\#\#\#\# h7
*bold*
#emph[italic]
#emph[*bold italic*]
#strike[strike through]
#block_quote[
  quote
]

- l
- l
- l

#line(length: 100%)
- list1
  - list2
    - list3
      - list4
        - list5
          - list6
            - list7

- list1
  - list2
    - list3
      - list4
        - list5
          - list6
            - list7

+ list
+ list
+ list
  #link[link]
  #image("././img/7e70d3c88dbf90fadf0da23862f0113ea3e96d91.png")
  `code`

#line(length: 100%)
#line(length: 100%)
Here is a simple footnote. With some additional text after it.
```
code
```
```
code 
block
```
#table(
  columns: 3,
  align: (auto, auto, auto),
  [table], [table], [table],
  [table], [table], [table],
  [table], [table], [table],
  [table], [table], [table],
)

= Table
<table>

#table(
  columns: 5,
  align: (auto, auto, auto, auto, auto),
  [Syntax test], [Description], [Test], [XXX], [XXX],
  [Header], [Title], [AAAA], [XXX], [XXX],
  [Header], [Title], [AAAA], [XXX], [XXX],
  [Paragraph], [Text], [BBBB], [XXX], [XXX],
)

== Pretty table
<pretty-table>

#table(
  columns: 2,
  align: (auto, auto),
  [Header 1], [Header 2],
  [#image("././img/ded905929f9f27d5e6f084b799f4c8f1d145ec72.png")], [#image("././img/6e58c2da06fe64145ccd937de4236df64ecb506f.png")],
)

=== Lists
<lists>

#line(length: 100%)
- Inline #emph[code] `aaaaz`
- My *favorite search* engine is #strike[not] #link[Duck Duck Go].
- #strike[#emph[*Recursive*]]
- #emph[*#strike[Recursive reversed]*]
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

#block_quote[
  aa
  aa
  #block_quote[
    bb
    bb
    #block_quote[
      cc
      cc
      bb
      bb
      aa
      aa
    ]
  ]
]

#block_quote[
  single line
]

]
