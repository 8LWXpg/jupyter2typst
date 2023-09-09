#import "template.typ": *
#show: template

#block[
```julia
using Plots
gr()
default(fmt = :png)

using DataFrames
```

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
```julia
f(x) = sin(x)
g(x) = cos(x)
h(x) = tan(x)
```

]
#block[
#ansi-render("h (generic function with 1 method)")
]
#block[
```julia
xs = LinRange(0, 2pi, 100)
```

]
#block[
#ansi-render("100-element LinRange{Float64, Int64}:
 0.0, 0.0634665, 0.126933, 0.1904, …, 6.09279, 6.15625, 6.21972, 6.28319")
]
#block[
These are the trigonometric functions,
\$\$\\sin(x)\$\$
\$\$\\cos(x)\$\$
\$\$\\tan(x)\$\$
According to Wikipedia, their graphs look like this:
#image("img/e4f510a108a52350c25b6485f4c9058cdae2ccba.png")
]
#block[
```julia
plot(xs, [f, g, h]; ylim = (-2, 2), framestyle = :box, grid = false, palette = :tab10)
```

]
#block[
#image("./img/ea844af1262c9c7267aaed6e3f6bb2a54b115ac9.png")]
#block[
Let\'s produce an error:
]
#block[
```julia
i(x)
```

]
#block[
#ansi-render("UndefVarError: `i` not definedStacktrace: [1] top-level scope   @ In[24]:1")
]
#block[
== Rich Outputs

We can try some table outputs, for example:
]
#block[
```julia
df = DataFrame((col1 = ["First", "Second", "Third"], col2 = [1, 2, 3]))
```

]
#block[
#ansi-render("[1m3×2 DataFrame[0m
[1m Row [0m│[1m col1   [0m[1m col2  [0m
     │[90m String [0m[90m Int64 [0m
─────┼───────────────
   1 │ First       1
   2 │ Second      2
   3 │ Third       3")
]
