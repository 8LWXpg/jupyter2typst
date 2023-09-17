# Convert List

## Symbols

| LaTeX    | Typst            |
| -------- | ---------------- |
| `!`      | `!`              |
| `\!`     | `#h(-1em/6)`     |
| `#`      | TODO             |
| `\#`     | `\#`             |
| `%`      | `//`             |
| `\%`     | `%`              |
| `&`      | TODO             |
| `\&`     | `\&`             |
| `'`      | `'`              |
| `\'`     | `acute($1)`      |
| `(`      | `(`              |
| `)`      | `)`              |
| `\(…\)`  | TODO             |
| `\`      | `space`          |
| `\"`     | `dot.double($1)` |
| `\$`     | `$`              |
| `\,`     | `space.sixth`    |
| `\:`     | `#h(2em/9)`      |
| `\;`     | `#h(5em/18)`     |
| `_`      | `_`              |
| `\_`     | `\_`             |
| `` \` `` | `grave($1)`      |
| `<`      | `<`              |
| `\=`     | `macron($1)`     |
| `>`      | `>`              |
| `\>`     | `#h(2em/9)`      |
| `[`      | `[`              |
| `]`      | `]`              |
| `{}`     | TODO             |
| `\{`     | `{`              |
| `\}`     | `}`              |
| `\|`     | `\|`             |
| `\\|`    | `\|\|`           |
| `~`      | `space.nobreak`  |
| `\~`     | `tilde($1)`      |
| `^`      | `^`              |
| `\^`     | `hat($1)`        |

## A

| LaTeX               | Typst                |
| ------------------- | -------------------- |
| `\AA`               | `circle(A)`          |
| `\aa`               | `circle(a)`          |
| `\above`            | TODO                 |
| `\acute`            | `acute($1)`          |
| `\AE`               | `Æ`                  |
| `\ae`               | `æ`                  |
| `\alef`             | `alef`               |
| `\alefsym`          | `alef`               |
| `\aleph`            | `aleph`              |
| `\allowbreak`       | TODO                 |
| `\Alpha`            | `Alpha`              |
| `\alpha`            | `alpha`              |
| `\amalg`            | `product.co`         |
| `\And`              | `\&`                 |
| `\angl`             | TODO                 |
| `\angln`            | TODO                 |
| `\angle`            | `angle`              |
| `\approx`           | `approx`             |
| `\approxeq`         | `approx.eq`          |
| `\approxcolon`      | `approx:`            |
| `\approxcoloncolon` | `approx::`           |
| `\arccos`           | `arccos`             |
| `\arcctg`           | `#math.op("arcctg")` |
| `\arcsin`           | `arcsin`             |
| `\arctan`           | `arctan`             |
| `\arg`              | `arg`                |
| `\argmax`           | `arg max`            |
| `\argmin`           | `arg min`            |
| `\arraystretch`     | TODO                 |
| `\ast`              | `*`                  |
| `\asymp`            | `≍`                  |
| `\atop`             | TODO                 |

## B

| LaTeX                   | Typst                       |
| ----------------------- | --------------------------- |
| `\backepsilon`          | `in.rev.small`              |
| `\backprime`            | `prime.rev`                 |
| `\backsim`              | `tilde.rev`                 |
| `\backsimeq`            | `tilde.eq.rev`              |
| `\backslash`            | `\\`                        |
| `\bar`                  | `macron($1)`                |
| `\barwedge`             | `⊼`                         |
| `\Bbb`                  | `bb($1)`                    |
| `\bcancel`              | `cancel(inverted: #true)`   |
| `\begin`                | TODO                        |
| `\begingroup`           | TODO                        |
| `Beta`                  | `Beta`                      |
| `\beta`                 | `beta`                      |
| `\beth`                 | `beth`                      |
| `\between`              | `≬`                         |
| `\bf`                   | TODO                        |
| `\big` and its variants | TODO                        |
| `\bigcap`               | `sect.big`                  |
| `\bigcirc`              | `circle.stroked.big`        |
| `\bigcup`               | `union.big`                 |
| `\bigdot`               | `dot.circle.big`            |
| `\bigplus`              | `plus.circle.big`           |
| `\bigtimes`             | `times.circle.big`          |
| `\bigsqcup`             | `union.square.big`          |
| `\bigstar`              | `star.stroked`              |
| `\bigtriangledown`      | `triangle.stroked.b`        |
| `\bigtriangleup`        | `triangle.stroked.t`        |
| `\biguplus`             | `union.plus.big`            |
| `\bigvee`               | `or.big`                    |
| `\bigwedge`             | `and.big`                   |
| `\binom`                | `binom($1, $2)`             |
| `\blacklozenge`         | `lozenge.filled`            |
| `\blacksquare`          | `square.filled`             |
| `\blacktriangle`        | `triangle.filled.t`         |
| `\blacktriangledown`    | `triangle.filled.b`         |
| `\blacktriangleleft`    | `triangle.filled.l`         |
| `\blacktriangleright`   | `triangle.filled.r`         |
| `\bm`                   | `bold($1)`                  |
| `\bmod`                 | `mod`                       |
| `\bold`                 | `bold($1)`                  |
| `\boldsymbol`           | `bold($1)`                  |
| `\bot`                  | `bot`                       |
| `\bowtie`               | `⋈`                         |
| `\Box`                  | `square.stroked`            |
| `\boxdot`               | `dot.square`                |
| `\boxed`                | `#box(stroke: 0.5pt)[$$1$]` |
| `\boxminus`             | `minus.square`              |
| `\boxplus`              | `plus.square`               |
| `\boxtimes`             | `times.square`              |
| `\bra` and its variants | TODO                        |
| `\brace`                | TODO                        |
| `\brack`                | TODO                        |
| `\breve`                | `breve($1)`                 |
| `\bull`                 | `circle.filled.small`       |
| `\bullet`               | `circle.filled.small`       |
| `\Bumpeq`               | `≎`                         |
| `\bumpeq`               | `≏`                         |

## C

| LaTeX               | Typst                       |
| ------------------- | --------------------------- |
| `\cal`              | TODO                        |
| `\cancel`           | `cancel($1)`                |
| `\Cap`              | `sect.double`               |
| `\cap`              | `sect`                      |
| `\cdot`             | `dot.op`                    |
| `\cdotp`            | `dot.op`                    |
| `\cdots`            | `dots.h.c`                  |
| `\ce`               | not supported in ipynb      |
| `\centerdot`        | `dot.op`                    |
| `\cfrac`            | TODO                        |
| `\char`             | TODO                        |
| `\check`            | `caron($1)`                 |
| `\ch`               | not supported in ipynb      |
| `\Chi`              | `Chi`                       |
| `\chi`              | `chi`                       |
| `\choose`           | TODO                        |
| `\circ`             | `compose`                   |
| `\circeq`           | `≗`                         |
| `\circlearrowleft`  | `arrow.ccw`                 |
| `\circlearrowright` | `arrow.cw`                  |
| `\circledast`       | `ast.circle`                |
| `\circledcirc`      | `circle.nested`             |
| `\circleddash`      | `dash.circle`               |
| `\circledR`         | `®`                         |
| `\circledS`         | `Ⓢ`                         |
| `\clubs`            | `suit.club`                 |
| `\clubsuit`         | `suit.club`                 |
| `\cnums`            | `CC`                        |
| `\colon`            | `colon`                     |
| `\Colonapprox`      | `::approx`                  |
| `\colonapprox`      | `:approx`                   |
| `\coloncolon`       | `::`                        |
| `\coloncolonapprox` | `::approx`                  |
| `\coloncolonequals` | `::=`                       |
| `\coloncolonminus`  | `"::−"`                     |
| `\coloncolonsim`    | `"::~"`                     |
| `\Coloneq`          | `"::−"`                     |
| `\coloneq`          | `":-"`                      |
| `\colonequals`      | `:=`                        |
| `\Coloneqq`         | `::=`                       |
| `\coloneqq`         | `:=`                        |
| `\colonminus`       | `":−"`                      |
| `\Colonsim`         | `"::~"`                     |
| `\colonsim`         | `":~"`                      |
| `\color`            | TODO                        |
| `\colorbox`         | `#box(fill: rgb("$1"))[$2]` |
| `\complement`       | `complement`                |
| `\Complex`          | `CC`                        |
| `\cong`             | `tilde.equiv`               |
| `\coprod`           | `product.co`                |
| `\copyright`        | `copyright`                 |
| `\cos`              | `cos`                       |
| `\cosec`            | `#math.op("cosec")`         |
| `\cosh`             | `cosh`                      |
| `\cot`              | `cot`                       |
| `\cotg`             | `#math.op("cotg")`          |
| `\coth`             | `coth`                      |
| `\cr`               | TODO                        |
| `\csc`              | `csc`                       |
| `\ctg`              | `ctg`                       |
| `\cth`              | `#math.op("cth")`           |
| `\Cup`              | `union.double`              |
| `\cup`              | `union`                     |
| `\curlyeqprec`      | `eq.prec`                   |
| `\curlyeqsucc`      | `eq.succ`                   |
| `\curlyvee`         | `or.curly`                  |
| `\curlywedge`       | `and.curly`                 |
| `\curvearrowleft`   | `arrow.ccw.half`            |
| `\curvearrowright`  | `arrow.cw.half`             |

## D

| LaTeX               | Typst                   |
| ------------------- | ----------------------- |
| `\dag`              | `dagger`                |
| `\Dagger`           | `dagger.double`         |
| `\dagger`           | `dagger`                |
| `\daleth`           | `ℸ`                     |
| `\Darr`             | `arrow.b.double`        |
| `\dArr`             | `arrow.b.double`        |
| `\darr`             | `arrow.b`               |
| `\dashleftarrow`    | `arrow.l.dash`          |
| `\dashrightarrow`   | `arrow.r.dash`          |
| `\dashv`            | `tack.l`                |
| `\dbinom`           | `binom($1, $2)`         |
| `\dbcolon`          | `::`                    |
| `\ddag`             | `dagger.double`         |
| `\ddagger`          | `dagger.double`         |
| `\ddot`             | `dot.double($1)`        |
| `\ddots`            | `dots.down`             |
| `\def`              | TODO                    |
| `\deg`              | `deg`                   |
| `\degree`           | `degree`                |
| `\Delta`            | `Delta`                 |
| `\delta`            | `delta`                 |
| `\det`              | `det`                   |
| `\digamma`          | `ϝ`                     |
| `\dfrac`            | `frac($1, $2)`          |
| `\diagdown`         | `╲`                     |
| `\diagup`           | `╱`                     |
| `\Diamond`          | `lozenge.stroked`       |
| `\diamond`          | `diamond.stroked.small` |
| `\diamonds`         | `♢`                     |
| `\diamondsuit`      | `♢`                     |
| `\dim`              | `dim`                   |
| `\displaystyle`     | `display($1)`           |
| `\div`              | `div`                   |
| `\divideontimes`    | `times.div`             |
| `\dot`              | `dot($1)`               |
| `\Doteq`            | `≑`                     |
| `\doteq`            | `≐`                     |
| `\doteqdot`         | `≑`                     |
| `\dotplus`          | `plus.dot`              |
| `\dots`             | `dots.h.c`              |
| `\dotsb`            | `dots.h.c`              |
| `\dotsc`            | `dots.h.c`              |
| `\dotsi`            | `dots.h.c`              |
| `\dotsm`            | `dots.h.c`              |
| `\dotso`            | `dots.h`                |
| `\doublebarwedge`   | `⩞`                     |
| `\doublecap`        | `sect.double`           |
| `\doublecup`        | `union.double`          |
| `\Downarrow`        | `arrow.b.double`        |
| `\downarrow`        | `arrow.b`               |
| `\downdownarrows`   | `arrow.bb`              |
| `\downharpoonleft`  | `harpoon.bl`            |
| `\downharpoonright` | `harpoon.br`            |

## E

| LaTeX | Typst |
| ----- | ----- |
