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
#block_quote[
quote
]
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
  [table],
  [table],
  [table],
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
= Table
#table(
  columns: 5,
  align: (auto, auto, auto, auto, auto),
  [Syntax test],
  [Description],
  [Test],
  [XXX],
  [XXX],
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
== Pretty table
#table(
  columns: 2,
  align: (auto, auto),
  [Header 1],
  [Header 2],
  [#image("././img/ded905929f9f27d5e6f084b799f4c8f1d145ec72.png")],
  [#image("././img/6e58c2da06fe64145ccd937de4236df64ecb506f.png")],
)
=== Lists
#line(length: 100%)
==== Multiline code
```
int method() {
	return 2137;
}
```
===== Quotes
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
