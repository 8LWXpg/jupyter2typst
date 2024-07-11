#import "template.typ": *
#show: template

#block[
= h1

== h2

=== h3

==== h4

===== h5

====== h6

\#\#\#\#\#\#\# h7
*bold*
#emph[italic]
#emph[*bold italic*]
#strike[strike through]
#block-quote[
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
  #link("https://www.google.com")[link]
  #image("img/7e70d3c88dbf90fadf0da23862f0113ea3e96d91.png")
  `code`

#line(length: 100%)
#line(length: 100%)
Here is a simple footnote#link("https://www.example.com")[^1]. With some additional text after it#link("./out.typ no")[^2].
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

#table(
  columns: 5,
  align: (auto, auto, auto, auto, auto),
  [Syntax test], [Description], [Test], [XXX], [XXX],
  [Header], [Title], [AAAA], [XXX], [XXX],
  [Header], [Title], [AAAA], [XXX], [XXX],
  [Paragraph], [Text], [BBBB], [XXX], [XXX],
)

== Pretty table

#table(
  columns: 2,
  align: (auto, auto),
  [Header 1], [Header 2],
  [#image("img/ab2e0fb48e36657b7719077fe47911c7b5f84884.jpg")], [#image("img/4f3c219d9527429061b3bb90d9f8cb6c53f36556.jpg")],
)

=== Lists

#line(length: 100%)
- Inline #emph[code] `aaaaz`
- My *favorite search* engine is #strike[not] #link("https://duckduckgo.com")[Duck Duck Go].
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

```
int method() {
	return 2137;
}
```
===== Quotes

#block-quote[
  aa
  aa
  #block-quote[
    bb
    bb
    #block-quote[
      cc
      cc
      bb
      bb
      aa
      aa
    ]
  ]
]
#block-quote[
  single line
]
]
