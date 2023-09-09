#block[
```julia
using Plots
gr()
default(fmt = :png)

using DataFrames
```

] <26b33cee>
#block[
== Using Plots.jl
<using-plotsjl>
Plots.jl outputs plots in different formats. It is written in
#link("https://julialang.org")[Julia]:

#image("9ab2ada9-image.png")

] <9ab2ada9>
#block[
```julia
f(x) = sin(x)
g(x) = cos(x)
h(x) = tan(x)
```

#block[
```
h (generic function with 1 method)
```

]
] <85bd78ed>
#block[
```julia
xs = LinRange(0, 2pi, 100)
```

#block[
```
100-element LinRange{Float64, Int64}:
 0.0, 0.0634665, 0.126933, 0.1904, …, 6.09279, 6.15625, 6.21972, 6.28319
```

]
] <37a89891>
#block[
These are the trigonometric functions,

$ sin lr((x)) $ $ cos lr((x)) $ $ tan lr((x)) $

According to Wikipedia, their graphs look like this:

#image("63f92630-image.png")

] <63f92630>
#block[
```julia
plot(xs, [f, g, h]; ylim = (-2, 2), framestyle = :box, grid = false, palette = :tab10)
```

#block[
#image("817d75b3e0b22597d27ddb33b8ebd8e80b4f6bc4.png")

]
] <5af82814>
#block[
Let\'s produce an error:

] <a33baaeb>
#block[
```julia
i(x)
```

#block[
```
UndefVarError: `i` not defined

Stacktrace:
 [1] top-level scope
   @ In[24]:1
```

]
] <82b95d53>
#block[
== Rich Outputs
<rich-outputs>
We can try some table outputs, for example:

] <2fb78fbc>
#block[
```julia
df = DataFrame((col1 = ["First", "Second", "Third"], col2 = [1, 2, 3]))
```

#block[
```
3×2 DataFrame
 Row │ col1    col2  
     │ String  Int64 
─────┼───────────────
   1 │ First       1
   2 │ Second      2
   3 │ Third       3
```

]
] <ae37e722>
