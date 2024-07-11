// #import "template.typ": *
#show: template

#block[
= Jupyter Notebook files

#lorem(50)

== Markdown content

#lorem(50)
]
#block[
#code-block("import pandas as pd
pd.DataFrame([['hi', 'there'], ['this', 'is'], ['a', 'DataFrame']], columns=['Word A', 'Word B'])"
, lang: "python", count: 6)
]
#block[
#result-block("  Word A     Word B
0     hi      there
1   this         is
2      a  DataFrame")
]
#block[
#result-block("[1;31m---------------------------------------------------------------------------[0m
[1;31mNameError[0m                                 Traceback (most recent call last)
Cell [1;32mIn[9], line 1[0m
[1;32m----> 1[0m this_will_error

[1;31mNameError[0m: name 'this_will_error' is not defined")
]
