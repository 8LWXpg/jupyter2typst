#import "template.typ": *
#show: template

#block[
#code-block("using Plots
gr()
default(fmt = :png)

using DataFrames"
, lang: "julia", count: 2)

]
#block[
]
#block[
== Using Plots.jl

Plots.jl outputs plots in different formats. It is written in #link("https://julialang.org")[Julia]:
#image("img/e4f510a108a52350c25b6485f4c9058cdae2ccba.png")
#image("img/c75b0d358982c06f338e70a3759a053a212d8278.png")
]
#block[
#code-block("f(x) = sin(x)
g(x) = cos(x)
h(x) = tan(x)"
, lang: "julia", count: 21)

]
#block[
#result-block("h (generic function with 1 method)")
]
#block[
#code-block("xs = LinRange(0, 2pi, 100)"
, lang: "julia", count: 22)

]
#block[
#result-block("100-element LinRange{Float64, Int64}:
 0.0, 0.0634665, 0.126933, 0.1904, …, 6.09279, 6.15625, 6.21972, 6.28319")
]
#block[
These are the trigonometric functions,
$sin(x)$
$cos(x)$
$tan(x)$
According to Wikipedia, their graphs look like this:
#image("img/e4f510a108a52350c25b6485f4c9058cdae2ccba.png")
]
#block[
#code-block("plot(xs, [f, g, h]; ylim = (-2, 2), framestyle = :box, grid = false, palette = :tab10)"
, lang: "julia", count: 23)

]
#block[
#image("./img/ea844af1262c9c7267aaed6e3f6bb2a54b115ac9.png")]
#block[
Let\'s produce an error:
]
#block[
#code-block("i(x)"
, lang: "julia", count: 24)

]
#block[
#result-block("UndefVarError: `i` not defined

Stacktrace:
 [1] top-level scope
   @ In[24]:1")
]
#block[
== Rich Outputs

We can try some table outputs, for example:
]
#block[
#code-block("df = DataFrame((col1 = [\"First\", \"Second\", \"Third\"], col2 = [1, 2, 3]))"
, lang: "julia", count: 3)

]
#block[
#result-block("[1m3×2 DataFrame[0m
[1m Row [0m│[1m col1   [0m[1m col2  [0m
     │[90m String [0m[90m Int64 [0m
─────┼───────────────
   1 │ First       1
   2 │ Second      2
   3 │ Third       3")
]
