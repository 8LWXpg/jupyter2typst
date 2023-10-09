#import "template.typ": *

#block[
= 1.

== (a)

]
#block[
#code-block("using Plots
gr()

p = 0:0.01:1

I(p) = -p * log2(p)
H(p) = I(p) + I(1 - p)

plot(p, [I.(p), I.(1 .- p), H.(p)], label=[\"I(p)\" \"I(1-p)\" \"H(p)\"])"
, lang: "julia", count: 1)

]
#block[
#image("./img/985daaaacb50fe430f2a5eae9d74119bb590aea2.svg")]
#block[
== (b)

]
#block[
#code-block("p = 0:0.01:1
I(p) = -p * log2(abs(p))
H(p1, p2) = I(p1) + I(p2) + I(1 - p1 - p2)

surface(p, p, H)"
, lang: "julia", count: 2)

]
#block[
#image("./img/e7da6cb820adc923140956e22a8c0d372f812a89.svg")]
#block[
= 2.

The derivative image has a lower entropy than the original image, because most of its pixel values are close to zero and have a high probability. This means that the derivative image contains less information per pixel than the original image, and therefore it can be compressed more efficiently.
]
#block[
= 3.

== (a)

]
#block[
#code-block("using Optim

function quantize(f::Function, bits::Int, first::Real, last::Real)
    min = optimize(f, first, last).minimum
    max = -optimize(x -> -f(x), first, last).minimum
    step = (max - min) / (2^bits - 1)
    # return quantize function and error function
    return [x -> min + step * round((f(x) - min) / step), x -> f(x) - min - step * round((f(x) - min) / step)]
end

bit = 3

f(x) = x
p1 = plot(f, -1, 1, label=\"f(x)\")
plot!(quantize(f, bit, -1, 1), -1, 1, label=[\"quantize(f, $bit)\" \"error\"], legend=:topleft)"
, lang: "julia", count: 3)

]
#block[
#image("./img/c99fc14339dd048ab2beb09d5d14b5bc121b9d0d.svg")]
#block[
== (b)

]
#block[
#code-block("f(x) = sin(x)
p2 = plot(f, 0, 2π, label=\"f(x)\")
plot!(quantize(f, 3, 0, 2π), 0, 2π, label=[\"quantize(f, $bit)\" \"error\"])"
, lang: "julia", count: 4)

]
#block[
#image("./img/5b73b52a47ccaa96e7996b317c04e1a15ef968d6.svg")]
